use std::fs;
use torrust::metainfo;

#[test]
fn test_decode_metainfo_sample_torrent() {
    // Read the sample torrent file as bytes
    let file_path = "tests/test_data/sample.torrent";
    let file_bytes = fs::read(file_path).expect("Failed to read sample.torrent file");

    let result = metainfo::decode_metainfo(&file_bytes);

    // Ensure the function doesn't throw any errors
    assert!(
        result.is_ok(),
        "decode_metainfo should not fail on valid torrent file"
    );

    // Additional verification that we got a valid MetaInfo struct
    let metainfo = result.unwrap();

    // Basic sanity checks on the decoded metainfo
    assert!(
        !metainfo.announce.is_empty(),
        "Announce URL should not be empty"
    );
    assert!(
        !metainfo.info.name.is_empty(),
        "Info name should not be empty"
    );
    assert!(
        metainfo.info.piece_length > 0,
        "Piece length should be positive"
    );
    assert!(
        !metainfo.info.pieces.is_empty(),
        "Pieces should not be empty"
    );
}

#[test]
fn test_decode_metainfo_with_invalid_data() {
    // Test with obviously invalid data to ensure proper error handling
    let invalid_data = "this is not a valid torrent file";
    let result = metainfo::decode_metainfo(invalid_data.as_bytes());

    // Should return an error for invalid data
    assert!(
        result.is_err(),
        "decode_metainfo should fail on invalid data"
    );
}

#[test]
fn test_decode_metainfo_with_empty_data() {
    // Test with empty data
    let empty_data = "";
    let result = metainfo::decode_metainfo(empty_data.as_bytes());

    // Should return an error for empty data
    assert!(result.is_err(), "decode_metainfo should fail on empty data");
}
