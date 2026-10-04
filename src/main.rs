use reqwest::blocking::Client;
use std::{collections::HashMap, print};

#[derive(serde::Deserialize, Debug)]
struct GithubDirectory {
    pub download_url: String,
    pub git_url: String,
    pub html_url: String,
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: i32,
    #[serde(rename = "type")]
    pub content_type: String,
    pub url: String,
}

#[derive(serde::Deserialize, Debug)]
struct BanListVectorFormat {
    pub date: String,
    // String is Konami id, i32 is count
    pub regulation: HashMap<String, i32>
}

// From https://ygoprodeck.com/api-guide/
#[derive(serde::Deserialize, Debug)]
struct YGOProDeckResponse {
    data: Vec<YGOProDeckCardResponse>,
}

#[derive(serde::Deserialize, Debug)]
struct YGOProDeckCardResponse {
    id: i32,
    name: String,
    #[serde(rename = "type")]
    card_type: String,
    #[serde(rename = "frameType")]
    frame_type: String,
    desc: String,
    ygoprodeck_url: String,
    card_images: Vec<YGOProdDeckCardImageResponse>
}

#[derive(serde::Deserialize, Debug)]
struct YGOProdDeckCardImageResponse {
    id: i32,
    image_url: String,
    image_url_small: String,
    image_url_cropped: String,
}

fn main() {
    let client = Client::builder()
        .user_agent("ygo-utils")
        .build().unwrap();

    let body = client
    .get("https://api.github.com/repos/DawnbrandBots/yaml-yugi-limit-regulation/contents/data/master-duel")
    .send()
    .unwrap()
    .json::<Vec<GithubDirectory>>()
    .unwrap();

    for item in body {
        println!("Item = {:#?}", item)
    }

    let file = client
        .get("https://api.github.com/repos/DawnbrandBots/yaml-yugi-limit-regulation/contents/data/master-duel/2026-10-06.vector.json")
        .header(reqwest::header::ACCEPT, "application/vnd.github.raw+json")
        .send()
        .unwrap()
        .json::<BanListVectorFormat>()
        .unwrap();

    println!("Content = {:#?}", file);

    let mut contents = Vec::<String>::new();

    for (konami_id, count) in file.regulation.iter() {
        let card = client
            .get(format!("https://db.ygoprodeck.com/api/v7/cardinfo.php?konami_id={}&misc=yes", konami_id))
            .send()
            .unwrap()
            .json::<YGOProDeckResponse>()
            .unwrap()
            .data;

        println!("Sleeping");
        std::thread::sleep(std::time::Duration::from_millis(500));

        for card_art in &card[0].card_images {
            let id = card_art.id;
            let card_name = &card[0].name;
            // Format is
            // <id> <count> --<card name>
            contents.push(format!("{} {} --{}", id, count, card_name));
        }
    }

    let content = contents.join("\n");

    std::fs::write("master_duel.lflist.conf", content).unwrap();
}
