use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Cursor;
use std::io::Read;
use std::io::{self, Result};

type AssetTreeMap = BTreeMap<String, Asset>;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Asset {
    asset_type: AssetType,
    compressed_blob: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AssetDatabase {
    pub assets: AssetTreeMap,
}

pub struct AssetDatabaseBuilder {
    compression: Compression,
    assets: AssetTreeMap,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
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

impl AssetDatabaseBuilder {
    pub fn new(compression: Compression) -> AssetDatabaseBuilder {
        AssetDatabaseBuilder {
            compression: compression,
            assets: BTreeMap::new(),
        }
    }

    pub fn addBlob(mut self, blob: Vec<u8>, asset_type: AssetType) -> AssetDatabaseBuilder {
        let mut encoder = GzEncoder::new(Vec::new(), self.compression);
        let mut buff = Cursor::new(blob);

        io::copy(&mut buff, &mut encoder).unwrap();

        let result = encoder.finish().unwrap();
        let asset = Asset {
            asset_type: asset_type,
            compressed_blob: result,
        };

        self.assets.insert(String::from("_file_identifier_"), asset);

        self
    }

    pub fn addFile(mut self, path: String) -> AssetDatabaseBuilder {
        Self::addBlob(self, std::fs::read(path).unwrap(), AssetType::_ILLEGAL)
    }

    pub fn finish(self) -> AssetDatabase {
        AssetDatabase {
            assets: self.assets,
        }
    }
}
