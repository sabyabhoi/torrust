use std::collections::HashMap;

use eyre::Result;
use url::Url;

pub struct TrackerRequest {
    pub info_hash: String,
    pub peer_id: String,
    pub ip: String,
    pub port: u16,
    pub uploaded: u64,
    pub downloaded: u64,
    pub left: u64,
    pub event: Option<Event>,
}

pub enum TrackerResponse {
    Success { interval: u32, peers: Vec<Peer> },
    Failure { reason: String },
}

pub struct Peer {
    pub id: String,
    pub ip: String,
    pub port: u16,
}

pub enum Event {
    Started,
    Completed,
    Stopped,
}

pub fn parse_magnet_link(magnet_uri: &str) -> Result<()> {
    let url = Url::parse(magnet_uri)?;

    let query_pairs: HashMap<_, _> = url.query_pairs().collect();

    let response = query_pairs
        .get("xt")
        .and_then(|xt| xt.strip_prefix("urn:btih:"));

    if let Some(response_text) = response {
        println!("response text: {}", response_text);
    }

    if let Some(dn) = query_pairs.get("dn") {
        println!("display name: {}", dn);
    }

    Ok(())
}
