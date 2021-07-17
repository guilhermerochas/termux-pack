use serde_json::Value;
use tempfile::TempDir;

use crate::{manifest::ManifestFile, utils};
use std::{fs::File, io::Write, path::PathBuf, str};

pub(crate) fn write_control_tar(
    tar_path: &TempDir,
    manifest: &ManifestFile,
    subscriptios: &Vec<PathBuf>,
) -> std::io::Result<()> {
    let maintainer = manifest.maintainer.unwrap_or("None");

    let mut contents = format!(
        "Package: {}\nVersion: {}\nArchitecture: {}\nMaintainer: {}\nDescription: {}\n",
        &manifest.name, &manifest.version, &manifest.arch, maintainer, &manifest.description
    );

    if !manifest.homepage.is_none() {
        let homepage = format!("Homepage: {}\n", manifest.homepage.unwrap());
        contents.push_str(&homepage);
    }

    let json_value: Value = serde_json::value::to_value(manifest).unwrap();

    for property in ["depends", "recommends", "suggests", "provides", "conflicts"].iter() {
        if !json_value[property].is_null() {
            let arr: Vec<&str> = json_value[property]
                .as_array()
                .unwrap()
                .iter()
                .map(|i| i.as_str().unwrap())
                .collect::<Vec<&str>>();

            match utils::vec_to_string(&arr, &property) {
                Some(property) => contents.push_str(&property),
                None => (),
            }
        }
    }

    let mut control_file = File::create("./control")?;
    control_file.write_all(&contents.as_bytes())?;

    let mut tar = tar::Builder::new(File::create("control.tar")?);
    tar.append_file(".", &mut control_file)?;

    Ok(())
}
