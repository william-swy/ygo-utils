use clap::{Args, Parser, ValueEnum};
use futures::StreamExt;
use std::str::FromStr;

mod ban_list_writer;
mod throttle;
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

        let content = ban_list_client.get_banlist_content(&name).await?;

        let mut ban_list_entries = Vec::<ban_list_writer::EDOProBanListEntry>::new();

        let client = ygo_pro_deck::Client::new()?;

        let results = futures::stream::iter(content.regulation)
            .map(|(konami_id, count)| {
                let client = &client;
                async move {
                    println!("Start get");
                    let card = client.get_card_by_konami_id(&konami_id).await?;

                    println!("Done get");
                    Ok::<(ygo_pro_deck::YGOProDeckCardResponse, i32), ygo_pro_deck::YGOProDeckErr>(
                        (card, count),
                    )
                }
            })
            .buffer_unordered(2)
            .fuse();

        tokio::pin!(results);

        while let Some(res) = results.next().await {
            let (card, count) = res?;

            for card_art in &card.card_images {
                ban_list_entries.push(ban_list_writer::EDOProBanListEntry {
                    ygo_pro_id: card_art.id,
                    count: count,
                    card_name: card.name.to_owned(),
                });
            }
        }

        ban_list_writer::write(name.as_str(), ban_list_entries).unwrap();
    }

    Ok(())
}
