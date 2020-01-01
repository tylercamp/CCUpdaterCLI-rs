#[serde(Deserialize)]
pub struct ModDB {
    pub mods: HashMap<&str, RemoteModPackage>
}

#[serde(Deserialize)]
pub enum ModHash {
    #[serde(rename = "sha256")]
    Sha256(String)
}

#[serde(Deserialize)]
pub struct ModHomepage {
    pub name: String,
    pub url: String
}

#[serde(Deserialize)]
pub struct RemoteModPackage {
    pub name: String,
    pub description: String,
    pub license: String,
    pub page: [ModHomepage],
    pub archive_link: String,
    pub hash: ModHash,
    pub version: String
}

pub struct CrossCodeInstallation {
    pub base_path: String,

    pub game_version: String,
    pub loader_version: Option<String>,

    pub installed_mods: HashMap<&str, InstalledModPackage>,
}

pub struct InstalledModPackage {
    pub name: String,
    pub base_path: String,
    pub version: String,
    pub dependencies: HashMap<&str, String>
}
