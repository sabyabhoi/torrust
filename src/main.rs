use std::fs;

use eyre::Result;
use torrust::metainfo;

fn main() -> Result<()> {
    color_eyre::install()?;

    let file_path = "tests/test_data/sample.torrent";
    let file_bytes = fs::read(file_path).expect("Failed to read sample.torrent file");

    let _ = metainfo::decode_metainfo(&file_bytes)?;

    Ok(())
}
