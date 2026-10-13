use crate::throttle::Throttler;

// #[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
    throttle: Throttler,
}

// From https://ygoprodeck.com/api-guide/
#[derive(serde::Deserialize, Debug)]
struct YGOProDeckResponse {
    data: Vec<YGOProDeckCardResponse>,
}

#[derive(serde::Deserialize, Debug)]
pub struct YGOProDeckCardResponse {
    pub name: String,
    // All alt arts
    pub card_images: Vec<YGOProDeckCardImageResponse>,
}

#[derive(serde::Deserialize, Debug)]
pub struct YGOProDeckCardImageResponse {
    pub id: i32,
}

#[derive(Debug)]
pub enum YGOProDeckErr {
    RequestErr(reqwest::Error),
    NotFound,
    TooManyMatches(usize),
}

impl Client {
    pub fn new() -> Result<Self, reqwest::Error> {
        Ok(Self {
            client: reqwest::Client::builder().user_agent("ygo-utils").build()?,
            throttle: Throttler::new(10),
        })
    }

    pub async fn get_card_by_konami_id(
        &self,
        konami_id: &str,
    ) -> Result<YGOProDeckCardResponse, YGOProDeckErr> {
        self.throttle.invoke().await;
        let res = self
            .client
            .get(format!(
                "https://db.ygoprodeck.com/api/v7/cardinfo.php?konami_id={}&misc=yes",
                konami_id
            ))
            .send()
            .await?
            .json::<YGOProDeckResponse>()
            .await?
            .data;

        if res.len() > 1 {
            Err(YGOProDeckErr::TooManyMatches(res.len()))
        } else {
            res.into_iter().next().ok_or(YGOProDeckErr::NotFound)
        }
    }
}

impl std::fmt::Display for YGOProDeckErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YGOProDeckErr::RequestErr(err) => std::write!(f, "Failed to get response {}", err),
            YGOProDeckErr::NotFound => std::write!(f, "Card not found"),
            YGOProDeckErr::TooManyMatches(count) => std::write!(f, "Found {} matches, expected 1", count),
        }
    }
}

impl std::error::Error for YGOProDeckErr {}

impl From<reqwest::Error> for YGOProDeckErr {
    fn from(value: reqwest::Error) -> Self {
        Self::RequestErr(value)
    }
}