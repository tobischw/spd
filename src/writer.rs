use super::asset::{Asset, AssetDatabase};
use super::Document;

impl Document {
    #[allow(dead_code)]
    pub fn save(&mut self, path: String) {
        println!("{}", path);
    }
}