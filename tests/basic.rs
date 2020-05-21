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



fn test_asset_builder() {
    let adb = spd::AssetDatabaseBuilder::new(Compression::best());

    let testDatabase = adb
        .addFile(String::from("./index.html"))
        .finish();

    
}
