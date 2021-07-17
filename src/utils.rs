use core::str;

use crate::manifest::ManifestFile;

pub fn captalize_str(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

pub fn validate_manifest(manifest: &ManifestFile) {
    // Validate that the package manifest makes sense.
    if !["all", "arm", "i686", "aarch64", "x86_64"]
        .iter()
        .any(|arch| arch.to_string() == manifest.arch)
    {
        eprintln!(r#"'Invalid "arch" - must be one of all/arm/i686/aarch64/x86_64'"#);
        std::process::exit(1);
    }
}

pub(crate) fn vec_to_string(vector: &Vec<&str>, property_name: &str) -> Option<String> {
    if vector.is_empty() {
        return None;
    }

    let mut vector_str = vector
        .iter()
        .map(|item| format!("{},", &item))
        .collect::<String>();
    vector_str = vector_str.as_str()[1..vector_str.as_str().len() - 1].to_string();

    let property = format!("{}: {}\n", captalize_str(property_name), &vector_str);

    Some(property)
}
