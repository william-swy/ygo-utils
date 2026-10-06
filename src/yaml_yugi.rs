pub struct Client {
    client: reqwest::Client,
}

pub struct BanListName {
    effective_date: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct BanListVector {
    pub date: String,
    // String is Konami id, i32 is count
    pub regulation: std::collections::HashMap<String, i32>
}

#[derive(Debug)]
pub struct ParseBanListNameErr;

#[derive(serde::Deserialize, Debug)]
struct GithubDirectory {
    name: String,
}

impl Client {
    pub fn new() -> Result<Self, reqwest::Error> {
        Ok(Self { 
            client: reqwest::Client::builder()
                .user_agent("ygo-utils")
                .build()?
        })
    }

    pub async fn get_banlists(&self) -> Result<Vec<BanListName>, reqwest::Error> {
        let res = self.client
            .get("https://api.github.com/repos/DawnbrandBots/yaml-yugi-limit-regulation/contents/data/master-duel")
            .send()
            .await?
            .json::<Vec<GithubDirectory>>()
            .await?;

        Ok(res.iter()
            .filter_map(|dir| 
                Self::banlist_date_file(&dir.name)
                    .and_then(|date| 
                        Some(BanListName {
                            effective_date: date.to_owned()
                        })
                    )
            )
            .collect())
    }

    fn banlist_date_file(name: &str) -> Option<&str> {
        if name.ends_with(".name.json") {
            return None
        }

        let prefix = name.split(".").next();

        if let Some(date) = prefix {
            return chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").is_ok().then_some(date)
        }

        return None
    }

    pub async fn get_banlist_content(&self, ban_list_name: BanListName) -> Result<BanListVector, reqwest::Error> {
        self.client
            .get(format!("https://api.github.com/repos/DawnbrandBots/yaml-yugi-limit-regulation/contents/data/master-duel/{}.vector.json", ban_list_name.effective_date))
            .header(
                reqwest::header::ACCEPT, 
                "application/vnd.github.raw+json"
            )
            .send()
            .await?
            .json::<BanListVector>()
            .await
    }
}

impl std::fmt::Display for BanListName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.effective_date)
    }
}

impl std::str::FromStr for BanListName {
    type Err = ParseBanListNameErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .is_ok()
            .then_some(Self {
                effective_date: s.to_owned(),
            })
            .ok_or(ParseBanListNameErr)
    }
}

impl std::error::Error for ParseBanListNameErr {}

impl std::fmt::Display for ParseBanListNameErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid ban list name")
    }
}
