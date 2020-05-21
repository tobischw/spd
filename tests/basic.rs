extern crate spd;

use flate2::Compression;

#[test]
fn create_doc() {
    let doc = spd::Document::new();
    assert_eq!(doc.head.min_version, 0x01)
}

#[test]
fn create_fileio() {
    let y = spd::FileIO::new(String::from("yeet.txt"));
    assert_eq!(y.file, String::from("yeet.txt"))
}

#[test]
fn read_file(){
    let y = spd::FileIO::new(String::from("./res/index.html"));
    let d = y.read_file().unwrap();
    println!("{:?}", d)
}

#[test]
fn test_asset_builder() {
    let mut doc = spd::Document::new();
    let adb = spd::AssetDatabaseBuilder::new(Compression::default());

    let test_database = adb
         .add_file(String::from("index.html"), String::from("./demo/index.html"))
         .add_file(String::from("logo.png"), String::from("./demo/logo.png"))
         .add_file(String::from("amaranth.otf"), String::from("./demo/amaranth.otf"))
        .finish();

    doc.with_assets(test_database); 
    doc.save(String::from("./demo/demo.spd")).unwrap();
}
