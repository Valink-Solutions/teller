use std::{fs, path::PathBuf};

use crate::{handlers::backup::grab_backup_metadata, types::world::WorldData};

fn get_backups_from_path(directory_path: &str) -> Result<Vec<std::fs::DirEntry>, std::io::Error> {
    let entries = fs::read_dir(directory_path)?;
    let files: Vec<std::fs::DirEntry> = entries.filter_map(|entry| entry.ok()).collect();
    Ok(files)
}

fn find_newest_backup(files: &[std::fs::DirEntry]) -> Option<PathBuf> {
    let mut newest_file: Option<PathBuf> = None;
    let mut newest_time = i64::MIN;

    for file in files {
        if let Some(file_name) = file.file_name().to_str() {
            let file_name = file_name.replace(".chunkvault-snapshot", "");

            if let Ok(time) = file_name.parse::<i64>() {
                if time > newest_time {
                    newest_time = time;
                    newest_file = Some(file.path());
                }
            }
        }
    }

    newest_file
}

pub fn fetch_backups_list(local_backups_path: PathBuf) -> Result<Vec<WorldData>, String> {
    let backup_entries = fs::read_dir(local_backups_path)
        .map_err(|e| format!("Failed to read backups directory: {}", e))?;

    let mut backups: Vec<WorldData> = Vec::new();

    for entry in backup_entries {
        if entry.is_ok() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                let all_backups = get_backups_from_path(path.to_str().unwrap())
                    .map_err(|e| format!("Failed to read backups directory: {}", e))?;

                let newest_backup = find_newest_backup(&all_backups);

                if let Some(newest_backup) = newest_backup {
                    let metadata = grab_backup_metadata(newest_backup);
                    if metadata.is_ok() {
                        let world_data = metadata.unwrap();
                        backups.push(world_data.entry);
                    } else {
                        continue;
                    }
                }
            }
        } else {
            continue;
        }
    }

    Ok(backups)
}
