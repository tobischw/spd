use super::asset::{Asset, AssetDatabase};
use super::Document;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Result};

struct DocumentReader {

}

impl DocumentReader {
    pub fn read(&mut self, path: String) -> std::io::Result<()> {

    }
}
