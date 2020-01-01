use std::collections::HashMap;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct ModDB {
    pub mods: HashMap<String, RemoteModPackage>
}

#[derive(Deserialize)]
pub enum ModHash {
    Sha256(String)
}

#[derive(Deserialize)]
pub struct ModHomepage {
    pub name: String,
    pub url: String
}

#[derive(Deserialize)]
pub struct RemoteModPackage {
    pub name: String,
    pub description: String,
    pub license: String,
    pub page: Vec<ModHomepage>,
    pub archive_link: String,
    pub hash: ModHash,
    pub version: String
}

pub struct CrossCodeInstallation {
    pub base_path: String,

    pub game_version: String,
    pub loader_version: Option<String>,
}

pub struct InstalledModPackage {
    pub name: String,
    pub base_path: String,
    pub version: String,
    pub dependencies: HashMap<String, String>
}
