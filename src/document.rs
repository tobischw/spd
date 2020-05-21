use super::asset::{Asset,  AssetDatabase};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ################################################

pub const MAGIC: [u8; 3] = [0x53, 0x50, 0x44];

pub const MAJ_VERSION: u32 = 0;

pub const MIN_VERSION: u32 = 0x01;

// ################################################

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Document {
    pub head: Head,
    pub database: AssetDatabase 
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Encryption {
    None,
    PBKDF2,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Head {
    pub magic: [u8; 3],
    pub maj_version: u32,
    pub min_version: u32,
    pub encryption: Encryption,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Meta {
    pub title: String,
    pub author: String,
    pub desc: String,
    pub tags: String,
    pub copyright: String,
}

impl Document {
    // Create a new SPD document.
    pub fn new() -> Document {
        Document {
            head: Head::new(),
            database: AssetDatabase {
                assets: BTreeMap::new()
            },
        }
    }
}

impl Head {
    pub fn new() -> Head {
        Head {
            magic: MAGIC,
            maj_version: MAJ_VERSION,
            min_version: MIN_VERSION,
            encryption: Encryption::None,
            meta: Meta::new(),
        }
    }
}

impl Meta {
    pub fn new() -> Meta {
        Meta {
            title: String::new(),
            author: String::new(),
            desc: String::new(),
            tags: String::new(),
            copyright: String::new(),
        }
    }
}
