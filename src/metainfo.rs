#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MetaInfo {
    announce: String,
    info: Info,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Info {
    name: String,
    piece_length: u32,
    pieces: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FileInfo {
    SingleFile { length: i64 },
    MultiFile { files: Vec<FileEntry> },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileEntry {
    length: i64,
    path: Vec<String>,
}
