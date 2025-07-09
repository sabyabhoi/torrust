use eyre::Result;

enum Bencode {
    Integer(i64),
    String(String),
    List(Vec<Bencode>),
    Dictionary(std::collections::HashMap<String, Bencode>),
}

fn decode_string(s: String) -> Result<(Bencode, String)> {
    unimplemented!()
}
