use super::asset::{Asset, AssetDatabase};
use super::Document;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Result};

impl Document {
    pub fn save(&mut self, path: String) -> std::io::Result<()> {
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        let mut buffer = File::create(path)?;

        buffer.write_all(&encoded).unwrap();

        Ok(())
    }
}
