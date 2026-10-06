use clap::{Args, Parser, ValueEnum};
use std::{println, str::FromStr};

mod yaml_yugi;
mod ygo_pro_deck;

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
        let name = yaml_yugi::BanListName::from_str(&version)?;

        let content = ban_list_client
            .get_banlist_content(&name)
            .await?;

        let mut ban_list_entries = Vec::<EDOProBanListEntry>::new();

        let client = ygo_pro_deck::Client::new()?;
        
        for (konami_id, count) in content.regulation.iter() {
            let card = client.get_card_by_konami_id(konami_id).await?;

            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

            for card_art in &card.card_images {
                ban_list_entries.push(
                    EDOProBanListEntry { 
                        ygo_pro_id: card_art.id, 
                        count: *count, 
                        card_name: card.name.to_owned(),
                    }
                );
            }
        }

        write_ban_list(name.as_str(), ban_list_entries).unwrap();
    }

    Ok(())
}
