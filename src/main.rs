mod config;
mod data;

use crate::{
    config::Config,
    data::{PieceCleaned, PieceOriginal},
};
use calamine::{open_workbook, Error, RangeDeserializerBuilder, Reader, Xlsx};
use meilisearch_sdk::Client;
use std::{convert::TryFrom, ops::Deref, time::Duration};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_excel_to_meilisearch=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();

    tracing::debug!("Fetching spreadsheet");

    // Fetch the spreadsheet
    let file_name = "spreadsheet.xlsx";
    let res = reqwest::get(&config.lute_music_url).await?.bytes().await?;
    let mut data = res.deref();

    tracing::debug!("Writing to file");

    // Save into a file (can't figure out how to keep in memory)
    let mut f = std::fs::File::create(file_name)?;
    std::io::copy(&mut data, &mut f)?;

    tracing::debug!("Iterating over spreadsheet rows");
    let mut workbook: Xlsx<_> = open_workbook(file_name)?;

    let range = workbook
        .worksheet_range("dft")
        .ok_or(Error::Msg("Cannot find 'dft'"))??;

    let iter = RangeDeserializerBuilder::new().from_range(&range)?;

    let mut parsed_rows = vec![];

    for piece in iter {
        match piece {
            Ok(piece) => {
                let piece: PieceOriginal = piece;
                match PieceCleaned::try_from(piece.clone()) {
                    Ok(parsed) => {
                        parsed_rows.push(parsed);
                    }
                    Err(e) => tracing::error!("{e}"),
                }
            }
            Err(e) => {
                tracing::error!("Error parsing row {e}");
            }
        };
    }

    tracing::debug!("Successfully parsed {} rows", parsed_rows.len());

    tracing::debug!("Updating meilisearch index");

    let meili_client = Client::new(&config.meili_host, Some(config.meili_key));

    let timeout = Some(Duration::from_secs(120));
    let index = meili_client.index("lute");

    // Set filterable attributes so we can query with difficulty and date ranges
    index
        .set_filterable_attributes(["difficulty", "date"])
        .await?
        .wait_for_completion(&meili_client, None, timeout)
        .await?;

    // Load the documents in batches
    let tasks = index
        .add_documents_in_batches(&parsed_rows, Some(1000), Some("id"))
        .await?;

    // Wait for all the tasks to be finished, timeout after $timeout
    meili_client
        .wait_for_task(tasks.last().unwrap(), None, timeout)
        .await?;

    tracing::info!("Success! Index updated");

    Ok(())
}
