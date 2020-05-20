use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::{self, Result};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Asset {
    asset_type: AssetType,
    compressed_blob: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AssetDatabase {
    assets: BTreeMap<String, Asset>
}

pub struct AssetBuilder {
    compression: Compression,
    asset_type: AssetType,
    uncompressed_blob: Vec<u8>,
}

pub struct AssetDatabaseBuilder {
    asset_builder: AssetBuilder,
    assets: Vec<Asset>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum AssetType {
    Content,
    Style,
    Font,
    Image,
    _ILLEGAL,
}

impl Asset {
    pub fn new() -> Asset {
        Asset {
            asset_type: AssetType::_ILLEGAL,
            compressed_blob: Vec::new(),
        }
    }
}

impl AssetBuilder {
    pub fn new() -> AssetBuilder {
        AssetBuilder {
            compression: Compression::best(),
            asset_type: AssetType::_ILLEGAL,
            uncompressed_blob: Vec::new(),
        }
    }

    pub fn withBlob(mut self, blob: Vec<u8>, asset_type: AssetType) {
        self.uncompressed_blob = blob;
        self.asset_type = asset_type;
    }

    pub fn withFile(mut self, file: File) {

    }

    fn finish(&self) -> Asset {
        let mut encoder = GzEncoder::new(Vec::new(), self.compression);

        io::copy(&mut self.uncompressed_blob, &mut encoder).unwrap();

        return Asset {
            asset_type: self.asset_type, 
            compressed_blob: encoder.finish().unwrap(),
        };
    }
}

impl AssetDatabaseBuilder {
    pub fn new(asset_builder : AssetBuilder) -> AssetDatabaseBuilder {
        AssetDatabaseBuilder {
            asset_builder : asset_builder,
            assets : Vec::new()
        }
    }



    fn finish(&self) -> Asset {

    }
}