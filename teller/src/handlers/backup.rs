use log::info;
use std::io::{Read, Seek, Write};
use std::path::Path;
use std::{fs::File, path::PathBuf};
use zip::write::FileOptions;
use zip::ZipWriter;

use serde_json::json;

use crate::handlers::config::{get_local_directories_config, get_minecraft_save_location};
use crate::handlers::world::get_vault_id;

use super::world::parse_world_entry_data;
use super::{
    config::get_config_folder, search::worlds::is_minecraft_world, world::process_world_data,
};

pub fn create_backup_from_id(world_id: &str, category: Option<&str>) -> Result<String, String> {
    let config_dir = get_config_folder();

    info!("Searching for world: {}", world_id);

    let mut paths: Vec<PathBuf> = Vec::new();

    match get_local_directories_config(&config_dir) {
        Ok(config) => {
            if let Some(category) = category {
                if category == "default" {
                    match get_minecraft_save_location() {
                        Some(path) => paths.push(path),
                        None => {}
                    };
                } else if let Some(vault_entries) = config.categories.get(category) {
                    for (_, path) in vault_entries.paths.iter() {
                        paths.push(path.clone());
                    }
                }
            }
        }
        Err(_e) => {}
    };

    for save_location in paths {
        let world_folders = match std::fs::read_dir(&save_location) {
            Ok(folders) => folders,
            Err(_) => continue,
        };

        for entry in world_folders {
            if let Ok(world_folder) = entry {
                let world_folder = world_folder.path();

                if !world_folder.is_dir() {
                    continue;
                }

                let vault_id = match get_vault_id(&world_folder) {
                    Ok(id) => id,
                    Err(_) => continue,
                };

                if vault_id == world_id {
                    info!("Found world: {world_id}");

                    match create_world_backup(world_folder.clone()) {
                        Ok(backup_path) => {
                            return Ok(backup_path.to_string_lossy().to_string());
                        }
                        Err(e) => {
                            return Err(format!("Could not create backup: {:?}", e));
                        }
                    };
                }
            }
        }
    }

    Err("Could not find world".to_string())
}

pub fn create_world_backup(world_path: PathBuf) -> Result<PathBuf, String> {
    let default_vault = get_default_vault();

    let temp_dir = match default_vault.join("temp").exists() {
        true => default_vault.join("temp"),
        false => {
            let _ = std::fs::create_dir_all(default_vault.join("temp"));
            default_vault.join("temp")
        }
    };

    let mut world_entry_data = parse_world_entry_data(world_path.clone())?;

    world_entry_data.path = "".to_string();

    let game_type = is_minecraft_world(&world_path);

    let world_data = match process_world_data(&world_path, game_type) {
        Ok(data) => data,
        Err(e) => {
            return Err(format!("Could not process world data: {:?}", e));
        }
    };

    let metadata = json!({
        "entry": world_entry_data,
        "data": world_data,
    });

    let backup_id = format!(
        "{}-{}.chunkvault-snapshot",
        world_entry_data.id,
        chrono::Utc::now().timestamp()
    );
    let backup_path = temp_dir.join(backup_id);

    let world_zip_path = temp_dir.join("world_data.zip");
    let mut world_zip = zip::ZipWriter::new(std::fs::File::create(&world_zip_path).unwrap());
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);
    add_directory_to_zip(&mut world_zip, &world_path, &world_path, &options).unwrap();
    world_zip.finish().unwrap();

    let mut zip = zip::ZipWriter::new(std::fs::File::create(backup_path.clone()).unwrap());
    zip.start_file(
        "metadata.json",
        FileOptions::default().compression_method(zip::CompressionMethod::Stored),
    )
    .unwrap();
    zip.write_all(metadata.to_string().as_bytes()).unwrap();

    let mut world_zip_file = File::open(&world_zip_path).unwrap();
    let mut buffer = Vec::new();
    world_zip_file.read_to_end(&mut buffer).unwrap();
    zip.start_file(
        "world_data.zip",
        FileOptions::default().compression_method(zip::CompressionMethod::Stored),
    )
    .unwrap();
    zip.write_all(&buffer).unwrap();

    zip.finish().unwrap();

    std::fs::remove_file(&world_zip_path).unwrap();

    Ok(backup_path)
}

fn add_directory_to_zip<W: Write + Seek>(
    zip_writer: &mut ZipWriter<W>,
    directory: &Path,
    prefix: &Path,
    options: &FileOptions,
) -> zip::result::ZipResult<()> {
    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();
        if path.is_file() {
            zip_writer.start_file(name.to_string_lossy().as_ref(), *options)?;
            let mut f = File::open(&path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            zip_writer.write_all(&buffer)?;
        } else if path.is_dir() {
            add_directory_to_zip(zip_writer, &path, prefix, options)?;
        }
    }
    Ok(())
}

fn get_default_vault() -> PathBuf {
    let config_dir = get_config_folder();

    let vault_dir = config_dir.join("vault");

    match vault_dir.exists() {
        true => vault_dir,
        false => {
            let _ = std::fs::create_dir_all(vault_dir.clone());
            vault_dir
        }
    }
}
