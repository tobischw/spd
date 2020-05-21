extern crate spd;

use flate2::Compression;

#[test]
fn create_doc() {
    let doc = spd::Document::new();

    assert_eq!(doc.head.min_version, 0x01)
}

#[test]
fn test_asset_builder() {
    let adb = spd::AssetDatabaseBuilder::new(Compression::best());

    let testDatabase = adb
        .addFile(String::from("./index.html"))
        .finish();

    
}