extern crate spd;

use flate2::Compression;

#[test]
fn create_doc() {
    let doc = spd::Document::new();

    assert_eq!(doc.head.min_version, 0x01)
}

#[test]
fn test_asset_builder() {
    let mut doc = spd::Document::new();
    let adb = spd::AssetDatabaseBuilder::new(Compression::best());

    let test_database = adb
         .add_file(String::from("index.html"), String::from("./demo/index.html"))
         .add_file(String::from("logo.html"), String::from("./demo/logo.png"))
        .finish();

    doc.with_assets(test_database); 
    doc.save(String::from("hello"));
}