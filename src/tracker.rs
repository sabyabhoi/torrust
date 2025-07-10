pub struct TrackerRequest {
    info_hash: String,
    peer_id: String,
    ip: String,
    port: u16,
    uploaded: u64,
    downloaded: u64,
    left: u64,
    event: Option<Event>,
}

pub enum TrackerResponse {
    Success { interval: u32, peers: Vec<Peer> },
    Failure { reason: String },
}

pub struct Peer {
    id: String,
    ip: String,
    port: u16,
}

pub enum Event {
    Started,
    Completed,
    Stopped,
}
