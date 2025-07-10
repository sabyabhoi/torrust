use eyre::Result;

use crate::bencode::{self, Bencode};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MetaInfo {
    pub announce: String,
    pub info: Info,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Info {
    pub name: String,
    pub piece_length: u32,
    pub pieces: Vec<Vec<u8>>,
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

pub fn decode_metainfo(metainfo: &[u8]) -> Result<MetaInfo> {
    let bencode = bencode::decode(metainfo.to_vec())?;
    match bencode {
        Bencode::Dictionary(dict) => {
            let announce = match dict.get("announce") {
                Some(Bencode::String(s)) => String::from_utf8(s.clone())
                    .map_err(|_| eyre::eyre!("Announce URL must be valid UTF-8"))?,
                _ => return Err(eyre::eyre!("MetaInfo must have 'announce' field as string")),
            };

            let info = match dict.get("info") {
                Some(info_bencode) => decode_info(info_bencode)?,
                _ => return Err(eyre::eyre!("MetaInfo must have 'info' field")),
            };

            Ok(MetaInfo { announce, info })
        }
        _ => return Err(eyre::eyre!("Invalid metainfo format")),
    }
}

pub fn decode_info(info: &Bencode) -> Result<Info> {
    match info {
        Bencode::Dictionary(dict) => {
            let name = match dict.get("name") {
                Some(Bencode::String(s)) => String::from_utf8(s.clone())
                    .map_err(|_| eyre::eyre!("Name must be valid UTF-8"))?,
                _ => {
                    return Err(eyre::eyre!(
                        "Info dictionary must have 'name' field as string"
                    ))
                }
            };

            let piece_length = match dict.get("piece length") {
                Some(Bencode::Integer(i)) => {
                    if *i < 0 || *i > u32::MAX as i64 {
                        return Err(eyre::eyre!("Piece length must be between 0 and 2^32 - 1"));
                    }
                    *i as u32
                }
                _ => {
                    return Err(eyre::eyre!(
                        "Info dictionary must have 'piece length' field as integer"
                    ))
                }
            };

            let pieces = match dict.get("pieces") {
                Some(Bencode::String(s)) => s.clone(),
                _ => {
                    return Err(eyre::eyre!(
                        "Info dictionary must have 'pieces' field as string"
                    ));
                }
            };
            if pieces.len() % 20 != 0 {
                return Err(eyre::eyre!("Pieces must be a multiple of 20 bytes"));
            }

            let mut pieces_vec = Vec::new();
            for i in (0..pieces.len()).step_by(20) {
                pieces_vec.push(pieces[i..i + 20].to_vec());
            }

            Ok(Info {
                name,
                piece_length,
                pieces: pieces_vec,
            })
        }
        _ => return Err(eyre::eyre!("Invalid info format")),
    }
}
