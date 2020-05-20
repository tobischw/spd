extern crate spd;

#[test]
fn create_doc() {
    let doc = spd::Document::new();

    assert_eq!(doc.head.min_version, 0x01)
}

fn create_doc_with_asset() {
    
}