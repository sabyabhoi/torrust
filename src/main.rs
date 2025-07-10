use std::fs;

use eyre::Result;
use torrust::{bencode, metainfo};

fn main() -> Result<()> {
    color_eyre::install()?;

    let file_path = "tests/test_data/sample.torrent";
    let file_bytes = fs::read(file_path).expect("Failed to read sample.torrent file");

    // Convert bytes to string for processing
    // Torrent files contain binary data, so we need to handle it as raw bytes
    // let torrent_data = String::from_utf8_lossy(&file_bytes);
    // dbg!(&torrent_data);

    // Attempt to decode the metainfo
    // let _ = metainfo::decode_metainfo(&torrent_data)?;

    let bencode = bencode::decode(file_bytes)?;
    dbg!(&bencode);

    Ok(())
}
