use std::fs;

use eyre::Result;
use torrust::{metainfo, tracker};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // let file_path = "tests/test_data/sample.torrent";
    // let file_bytes = fs::read(file_path).expect("Failed to read sample.torrent file");
    //
    // metainfo::decode_metainfo(&file_bytes)?;

    let file_path = "tests/test_data/magnet_link.txt";
    let magnet_uri = fs::read_to_string(file_path)?;

    tracker::parse_magnet_link(&magnet_uri)?;

    Ok(())
}
