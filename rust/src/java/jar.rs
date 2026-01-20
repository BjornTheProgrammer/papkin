use std::{fs::File, path::PathBuf};

use glob::glob;
use j4rs::ClasspathEntry;
use zip::ZipArchive;

pub fn discover_jar_files(plugin_folder: &PathBuf) -> Vec<String> {
    let pattern = format!("{}/**/*.jar", plugin_folder.to_string_lossy());
    let mut entries = Vec::new();

    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        log::info!("jar found: {:?}", entry);
        match entry {
            Ok(inner_path) => match inner_path.canonicalize() {
                Ok(path) => match path.to_str() {
                    Some(path) => entries.push(path.to_string()),
                    None => log::error!("Couldn't convert '{}' into string", inner_path.display()),
                },
                Err(e) => log::error!("Failed to convert path to string: {:?}", e),
            },
            Err(e) => log::error!("Failed to canonicalize path: {:?}", e),
        }
    }

    entries
}

pub fn create_classpath_entries(jar_paths: &[String]) -> Vec<ClasspathEntry> {
    jar_paths
        .iter()
        .map(|entry| ClasspathEntry::new(entry))
        .collect()
}

#[allow(dead_code)]
pub async fn read_item_from_jar(jar_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(jar_path)?;
    let archive = ZipArchive::new(file)?;

    for file_name in archive.file_names() {
        println!("File name: {}", file_name);
    }

    Ok(String::new())
}
