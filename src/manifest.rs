use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestFile {
    name: Option<String>,
    version: Option<String>,
    arch: String,
    mantainer: String,
    description: String,
    homepage: Option<String>,
    depends: Vec<String>,
    recomends: Vec<String>,
    suggests: Vec<String>,
    provides: Vec<String>,
    conflicts: Vec<String>,
    files: Option<Map<String, Value>>,
}

impl ManifestFile {
    pub fn from_json(json: &str) -> Self {
        let value: Value =
            serde_json::from_str(json).expect("Not able to convert manifest file to json");

        ManifestFile {
            name: if value["name"].is_null() {
                None
            } else {
                Some(value["name"].to_string())
            },
            version: if value["version"].is_null() {
                None
            } else {
                Some(value["version"].to_string())
            },
            arch: if value["arch"].is_null() {
                "arch".to_string()
            } else {
                value["arch"].to_string()
            },
            mantainer: if value["manteiner"].is_null() {
                "None".to_string()
            } else {
                value["manteiner"].to_string()
            },
            description: if value["description"].is_null() {
                "No Description".to_string()
            } else {
                value["description"].to_string()
            },
            homepage: if value["homepage"].is_null() {
                None
            } else {
                Some(value["homepage"].to_string())
            },
            depends: if value["depends"].is_null() {
                Vec::new()
            } else {
                value["depends"]
                    .as_array()
                    .expect("Not able to contect property `depends` to array")
                    .iter()
                    .map(|i| i.to_string())
                    .collect()
            },
            recomends: if value["recomends"].is_null() {
                Vec::new()
            } else {
                value["recomends"]
                    .as_array()
                    .expect("Not able to contect property `depends` to array")
                    .iter()
                    .map(|i| i.to_string())
                    .collect()
            },
            suggests: if value["suggests"].is_null() {
                Vec::new()
            } else {
                value["suggests"]
                    .as_array()
                    .expect("Not able to contect property `depends` to array")
                    .iter()
                    .map(|i| i.to_string())
                    .collect()
            },
            provides: if value["provides"].is_null() {
                Vec::new()
            } else {
                value["provides"]
                    .as_array()
                    .expect("Not able to contect property `depends` to array")
                    .iter()
                    .map(|i| i.to_string())
                    .collect()
            },
            conflicts: if value["conflicts"].is_null() {
                Vec::new()
            } else {
                value["conflicts"]
                    .as_array()
                    .expect("Not able to contect property `depends` to array")
                    .iter()
                    .map(|i| i.to_string())
                    .collect()
            },
            files: if value["files"].is_null() {
                None
            } else {
                Some(
                    value["files"]
                        .as_object()
                        .expect("Not able to contect property `files` to hashmap object")
                        .to_owned(),
                )
            },
        }
    }
}
