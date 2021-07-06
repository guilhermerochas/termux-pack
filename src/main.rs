use core::str;
use manifest::ManifestFile;
use serde_json::Value;
use std::io::Write;
use std::path::Path;
use tempfile::TempDir;

mod manifest;

static DESCRIPTION: &str = r#"Create a Termux package from a JSON manifest file. Example of manifest:

{
  "name": "mypackage",
  "version": "0.1",
  "arch": "all",
  "maintainer": "@MyGithubNick",
  "description": "This is a hello world package",
  "homepage": "https://example.com",
  "depends": ["python"],
  "recommends": ["vim"],
  "suggests": ["vim-python"],
  "provides": ["vi"],
  "conflicts": ["vim-python-git"],
  "files" : {
    "hello-world.py": "bin/hello-world",
    "hello-world.1": "share/man/man1/hello-world.1"
  }
}

The "maintainer", "description", "homepage", "depends", "recommends", "suggests", "provides" and "conflicts" fields are all optional.

The "depends", "recommends", and "suggests" fields should be a comma-separated list of packages that this package may depends on. Unlike "suggests", "depends" and "recommends" will be installed automatically when this package is installed using apt.

The "arch" field defaults to "all" (that is, a platform-independent package not containing native code) and can be any of arm/i686/aarch64/x86_64.  Run "uname -m" to find out arch name if building native code inside Termux.

The "files" map is keyed from paths to files to include (relative to the current directory) while the values contains the paths where the files should end up after installation (relative to $PREFIX).

Optional debscripts named "preinst", "postinst", "prerm", and "postrm" will be automatically included and executed upon installation and removing. They should exist within the same directory as the manifest.

The resulting .deb file can be installed by Termux users with:
  apt install ./package-file.deb
or by hosting it in an apt repository using the termux-apt-repo tool."#;

fn validate_manifest(manifest: &ManifestFile) -> Result<Value, Box<dyn std::error::Error>> {
    // Validate that the package manifest makes sense.

    let json: Value = serde_json::from_str(
        serde_json::to_string(&manifest)
            .expect(
                "Not able to convert an Manifest Object to String while validating the manifest",
            )
            .as_str(),
    )
    .expect("Not able to convert an string to JSON while validating the manifest");

    for property in ["name", "version", "files"].iter() {
        match json.get(property) {
            Some(value) => {
                if value.is_null() {
                    eprintln!("Missing mandatory {} property", property);
                    std::process::exit(1);
                }
            }
            None => unreachable!(),
        }
    }

    if !["all", "arm", "i686", "aarch64", "x86_64"]
        .iter()
        .any(|value| value.to_owned() == json["arch"].as_str().unwrap().replace("\"", ""))
    {
        println!("{}", &json["arch"].as_str().expect("nothing"));

        eprintln!(r#"'Invalid "arch" - must be one of all/arm/i686/aarch64/x86_64'"#);
        std::process::exit(1);
    }

    return Ok(json);
}

#[cfg(not(target_os = "windows"))]
fn main() -> std::io::Result<()> {
    // Generates a DEB file from a JSON manifest.
    let mut install_prefix: String = String::from("/data/data/com.termux/files/usr/");

    let argparser = clap::App::new("termux-pack")
        .author("guilherme-rochas")
        .version("0.1.0")
        .about(DESCRIPTION)
        .arg(
            clap::Arg::with_name("manifest")
                .long("manifest")
                .help("A JSON manifest file describing the package")
                .takes_value(true)
                .required(true)
                .max_values(1)
                .min_values(1),
        )
        .arg(
            clap::Arg::with_name("prefix")
                .long("--prefix")
                .help(format!("Set prefix dir (default: {})", install_prefix).as_str())
                .takes_value(true)
                .max_values(1)
                .required(false)
                .min_values(1),
        )
        .get_matches();

    match argparser.value_of("prefix") {
        Some(prefix) => install_prefix = String::from(prefix),
        None => (),
    }

    let manifest_path: &str = argparser
        .value_of("manifest")
        .expect("No MANIFEST file was provided");

    if !Path::new(manifest_path).exists() {
        eprintln!("The path to the manifest file was not found");
        std::process::exit(1);
    }

    let manifest_file: String = std::fs::read_to_string(manifest_path)
        .expect("Not ableto read the Manifest File")
        .to_owned();

    let manifest: ManifestFile = ManifestFile::from_json(&manifest_file.as_str());

    let manifest_json = validate_manifest(&manifest).expect("Not able to validate the manifest");

    let package_name = manifest_json["name"].as_str().unwrap();
    let package_version = manifest_json["version"].as_str().unwrap();
    //let package_files = &manifest_json["files"];
    let output_debfile_name = format!(
        "{}_{}_{}.deb",
        &package_name.replace("\"", ""),
        &package_version.replace("\"", ""),
        &manifest_json["arch"].as_str().unwrap().replace("\"", "")
    );

    let package_temp_directory = TempDir::new_in(".")?
        .path()
        .to_owned()
        .join("debian-binary");
    let mut tmp_file = std::fs::File::create(package_temp_directory)?;
    writeln!(tmp_file, "2.0\n")?;

    println!("Building {}", output_debfile_name);

    Ok(())
}
