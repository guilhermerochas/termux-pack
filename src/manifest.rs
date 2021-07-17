use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestFile<'a> {
    pub name: String,
    pub version: String,
    pub arch: String,
    pub maintainer: Option<&'a str>,
    #[serde(default = "description_default")]
    pub description: String,
    pub homepage: Option<&'a str>,
    #[serde(default)]
    pub depends: Vec<&'a str>,
    #[serde(default)]
    pub recommends: Vec<&'a str>,
    #[serde(default)]
    pub suggests: Vec<&'a str>,
    #[serde(default)]
    pub provides: Vec<&'a str>,
    #[serde(default)]
    pub conflicts: Vec<&'a str>,
    pub files: HashMap<String, String>,
}

impl<'a> ManifestFile<'a> {
    pub fn from_json(json: &'a str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}

fn description_default() -> String {
    String::from("No description")
}
