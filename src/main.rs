use clap::{Args, Parser, ValueEnum};
use std::{println, str::FromStr};

use crate::yaml_yugi::BanListName;

mod yaml_yugi;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    operation: Operations,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Operations {
    #[arg(short, long, value_enum)]
    show: Option<Show>,
    #[arg(short, long)]
    generate: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Show {
    All,
    Current,
    Next,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let ban_list_client = yaml_yugi::Client::new()?;

    let ban_lists = ban_list_client.get_banlists().await?;

    if let Some(show) = cli.operation.show {
        for name in ban_lists {
            println!("{}", name);
        }
    }

    if let Some(version) = cli.operation.generate {
        let name = BanListName::from_str(&version)?;

        let content = ban_list_client
            .get_banlist_content(name)
            .await?;

        let mut ban_list_entries = Vec::<EDOProBanListEntry>::new();

        let client = reqwest::Client::builder()
                .user_agent("ygo-utils")
                .build()?;
        
        for (konami_id, count) in content.regulation.iter() {
            let card = client
                .get(format!("https://db.ygoprodeck.com/api/v7/cardinfo.php?konami_id={}&misc=yes", konami_id))
                .send()
                .await?
                .json::<YGOProDeckResponse>()
                .await?
                .data;

            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

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



    // // TODO Need to add header
    // // #[<Date> Master Duel]
    // // !<Date> Master Duel
    // // For easier reading separate into sections
    // // #Forbidden
    // // #Limited
    // // #Semi-limited

    // let mut ban_list_entries = Vec::<EDOProBanListEntry>::new();

    // for (konami_id, count) in file.regulation.iter() {
    //     let card = client
    //         .get(format!("https://db.ygoprodeck.com/api/v7/cardinfo.php?konami_id={}&misc=yes", konami_id))
    //         .send()
    //         .unwrap()
    //         .json::<YGOProDeckResponse>()
    //         .unwrap()
    //         .data;

    //     println!("Sleeping");
    //     std::thread::sleep(std::time::Duration::from_millis(500));

    //     for card_art in &card[0].card_images {
    //         ban_list_entries.push(
    //             EDOProBanListEntry { 
    //                 ygo_pro_id: card_art.id, 
    //                 count: *count, 
    //                 card_name: card[0].name.clone()
    //             }
    //         );
    //     }
    // }

    // write_ban_list("2026-10-06", ban_list_entries).unwrap();
    Ok(())

}
