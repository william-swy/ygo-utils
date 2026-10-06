use clap::{Args, Parser, ValueEnum};
use std::str::FromStr;

use crate::ban_list_writer::EDOProBanListEntry;

mod ban_list_writer;
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let ban_list_client = yaml_yugi::Client::new()?;

    let ban_lists = ban_list_client.get_banlists().await?;

    if let Some(show) = cli.operation.show {
        for name in ban_lists {
            std::println!("{}", name);
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

        ban_list_writer::write(name.as_str(), ban_list_entries).unwrap();
    }

    Ok(())
}
