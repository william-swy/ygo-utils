use reqwest::blocking::Client;
use std::{collections::HashMap};

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
    card_images: Vec<YGOProDeckCardImageResponse>
}

#[derive(serde::Deserialize, Debug)]
struct YGOProDeckCardImageResponse {
    id: i32,
    image_url: String,
    image_url_small: String,
    image_url_cropped: String,
}

struct EDOProBanListEntry {
    // Different than konami id
    pub ygo_pro_id: i32,
    pub count: i32,
    pub card_name: String,
}

impl EDOProBanListEntry {
    fn to_line(&self) -> String {
        return format!("{} {} --{}", self.ygo_pro_id, self.count, self.card_name)
    }
}

fn write_ban_list(date: &str, ban_list_entries: Vec<EDOProBanListEntry>) -> Result<(), std::io::Error> {
    let mut contents = Vec::<String>::new();

    let file_name = format!("{}_master_duel.lflist.conf", date);

    contents.push(format!("#[{} Master Duel]", date));
    contents.push(format!("!{} Master Duel", date));

    contents.push(String::from("#Forbidden"));
    contents.extend(ban_list_entries.iter()
        .filter(|entry| entry.count == 0)
        .map(|entry| entry.to_line()));

    contents.push(String::from("#Limited"));
    contents.extend(ban_list_entries.iter()
        .filter(|entry| entry.count == 1)
        .map(|entry| entry.to_line()));

    contents.push(String::from("Semi-limited"));
    contents.extend(ban_list_entries.iter()
        .filter(|entry| entry.count == 2)
        .map(|entry| entry.to_line()));

    let content = contents.join("\n");

    std::fs::write(file_name, content)
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

    // TODO Need to add header
    // #[<Date> Master Duel]
    // !<Date> Master Duel
    // For easier reading separate into sections
    // #Forbidden
    // #Limited
    // #Semi-limited

    let mut ban_list_entries = Vec::<EDOProBanListEntry>::new();

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
            ban_list_entries.push(
                EDOProBanListEntry { 
                    ygo_pro_id: card_art.id, 
                    count: *count, 
                    card_name: card[0].name.clone()
                }
            );
        }
    }

    write_ban_list("2026-10-06", ban_list_entries).unwrap();

}
