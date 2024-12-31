// created to help xls_handler

use std::fs;
use std::path::{ Path, PathBuf };
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};

// Constants for folder names
const TMP_FOLDER: &str = "tmp";
const BACKUP_FOLDER: &str = "backup";
const MAX_BACKUPS: usize = 3; // including ds store))

// Function to set up and clean the folder structure
pub fn prepare_folder_structure(base_path: &str, res_file:&str) -> io::Result<()> {

    let base_folder = Path::new(base_path);
    let tmp_folder = base_folder.join(TMP_FOLDER);
    let backup_folder = base_folder.join(BACKUP_FOLDER);

    // Ensure that the necessary folders exist
    ensure_folder_exists(&base_folder)?;
    ensure_folder_exists(&tmp_folder)?;
    ensure_folder_exists(&backup_folder)?;

    // Clear the tmp folder it also checks if folder exists
    clear_tmp_folder(&tmp_folder)?;

    // Manage the backup folder (delete old backups) it also checks if folder exists
    manage_backups(&backup_folder, &base_folder.join(res_file))?;

    Ok(())
}


// This one is AI generated
// returns path to latest backup
pub fn get_latest_backup(destination_path: &str) -> Option<PathBuf> {
    let backup_dir = Path::new(destination_path).join(BACKUP_FOLDER);

    // Read the directory and collect the latest file
    let latest_backup = fs::read_dir(backup_dir)
        .ok()?
        .filter_map(Result::ok) // Ignore any errors while reading directory entries
        .filter( |entry| {
            entry.file_type().map(|ft| ft.is_file()).unwrap_or(false)
            && entry.file_name().to_string_lossy().ends_with(".xlsx") // only look for .xlsx backups
        }) // Filter only files
        .filter_map(|entry| {
            let path = entry.path();
            let metadata = entry.metadata().ok()?;
            Some((path, metadata.modified().ok()?))
        })
        .max_by_key(|(_, modified_time)| *modified_time) // Get the entry with the latest modification time
        .map(|(path, _)| path); // Extract the path

    latest_backup
}

// Utility function to ensure a folder exists, creating it if necessary
fn ensure_folder_exists(folder: &Path) -> io::Result<()> {
    if !folder.exists() {
        fs::create_dir_all(folder)?;
    }
    Ok(())
}

// Function to clear all files in the tmp folder
fn clear_tmp_folder(tmp_folder: &Path) -> io::Result<()> {
    if tmp_folder.exists() {
        for entry in fs::read_dir(tmp_folder)? {
            let entry = entry?;
            if entry.path().is_file() {
                fs::remove_file(entry.path())?;
            }
        }
    }
    Ok(())
}

// Function to manage backup files, deleting the oldest if more than MAX_BACKUPS exist
fn manage_backups(backup_folder: &Path, old_file_path: &Path) -> io::Result<()> {

    ensure_folder_exists(backup_folder)?;

    // Check if the old file exists before proceeding with storing another backup
    if old_file_path.exists() {
        // Construct the backup filename with timestamp or unique identifier
        let timestamp = get_minimalistic_timestamp();
        let backup_file_name = format!("backup_{}.xlsx", timestamp);
        let backup_file_path = backup_folder.join(backup_file_name);

        // Copy the new main file to the backup folder
        match fs::copy(old_file_path, &backup_file_path) {
            Ok(_) => {
                // Only remove the old file if the copy was successful
                if let Err(e) = fs::remove_file(old_file_path) {
                    println!("Error removing old file: {}", e);
                    return Err(e);
                }
            },
            Err(e) => {
                println!("Error copying file to backup: {}", e);
                return Err(e);
            }
        }
    }

    // Collect all backup files
    let mut backup_files: Vec<_> = fs::read_dir(backup_folder)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file()
            && path.file_name().unwrap_or_default().to_string_lossy().ends_with(".xlsx") // count only .xlsx files
            )
        .collect();

    // Sort the files by modification time (oldest first)
    backup_files.sort_by_key(|path| {
        fs::metadata(path)
            .and_then(|metadata| metadata.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH)
    });

    // If there are more than MAX_BACKUPS, delete the oldest
    while backup_files.len() > MAX_BACKUPS {
        if let Some(oldest_backup) = backup_files.first() {
            fs::remove_file(oldest_backup)?;
            backup_files.remove(0);
        }
    }

    Ok(())
}


// Function to format the current time as a minimalistic timestamp
fn get_minimalistic_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let seconds = now.as_secs();
    let date_time: DateTime<Utc> = DateTime::from_timestamp(seconds as i64, 0).expect("DateTimeErr");
    date_time.format("%Y%m%d_%H%M%S").to_string()
}


// generated by AI
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs::{self, File};
    use std::io::Write;
    use std::time::Duration;

    // Test that folders are created correctly
    #[test]
    fn test_folder_creation() {
        // Create a temporary directory for the test
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path().to_str().unwrap();

        // Specify a dummy resource file name
        let resource_file_name = "res_file.xlsx";

        // Run the function to prepare the folder structure
        prepare_folder_structure(base_path, &resource_file_name).unwrap();

        // Check that the necessary folders exist
        assert!(temp_dir.path().join(TMP_FOLDER).exists());
        assert!(temp_dir.path().join(BACKUP_FOLDER).exists());
    }

    // Test that the tmp folder is cleaned correctly
    #[test]
    fn test_tmp_folder_cleaned() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path().to_str().unwrap();
        let tmp_folder = temp_dir.path().join(TMP_FOLDER);

        // Create tmp folder and add files to it
        fs::create_dir(&tmp_folder).unwrap();
        File::create(tmp_folder.join("file1.txt")).unwrap();
        File::create(tmp_folder.join("file2.txt")).unwrap();

        // Specify a dummy resource file name
        let resource_file_name = "res_file.xlsx";

        // Run the function
        prepare_folder_structure(base_path, &resource_file_name).unwrap();

        // Check that the tmp folder is empty
        let entries: Vec<_> = fs::read_dir(tmp_folder).unwrap().collect();
        assert_eq!(entries.len(), 0);
    }

    // Test that old backups are deleted if there are more than MAX_BACKUPS
    #[test]
    fn test_backup_management() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path().to_str().unwrap();
        let backup_folder = temp_dir.path().join(BACKUP_FOLDER);
        let file4_path = backup_folder.join("backup_4.xlsx");

        // Specify a dummy resource file name it is not going to be used in this test
        let resource_file_name = "res_file.xlsx";

        // Create backup folder and add 5 backup files
        fs::create_dir(&backup_folder).unwrap();
        for i in 0..5 {
            let mut file = File::create(backup_folder.join(format!("backup_{}.xlsx", i))).unwrap();
            file.set_len(1).unwrap();
            write!(file, "backup data").unwrap();
            std::thread::sleep(Duration::from_millis(200));
        }

        // Run the function
        prepare_folder_structure(base_path, &resource_file_name).unwrap();

        // Check that only the newest MAX_BACKUPS files remain
        let entries: Vec<_> = fs::read_dir(backup_folder).unwrap().collect();
        assert_eq!(entries.len(), MAX_BACKUPS);

        // Check that the remaining files are the newest ones
        let backup_files: Vec<_> = entries.into_iter().map(|entry| entry.unwrap().path()).collect();
        let filenames: Vec<_> = backup_files.iter().map(|path| path.file_name().unwrap().to_str().unwrap()).collect();
        assert!(filenames.contains(&"backup_2.xlsx"));
        assert!(filenames.contains(&"backup_3.xlsx"));
        assert!(filenames.contains(&"backup_4.xlsx"));

        // Call the function and get the latest backup file
        let latest_backup = get_latest_backup(temp_dir.path().to_str().unwrap());

        // Assert that the latest backup is indeed file3
        assert_eq!(latest_backup, Some(file4_path));
    }

    #[test]
    fn test_manage_backups_copying_main_to_backup() {
        let temp_dir = tempdir().unwrap();
        let backup_folder = temp_dir.path().join("backup");
        let old_file_path = temp_dir.path().join("old_file.xlsx");

        // Create a dummy old file
        File::create(&old_file_path).unwrap().write_all(b"Test data").unwrap();

        // Ensure the backup folder exists
        fs::create_dir(&backup_folder).unwrap();

        // Call manage_backups to create a backup
        let result = manage_backups(&backup_folder, &old_file_path);
        assert!(result.is_ok());

        // Check if the backup file was created
        let backup_files: Vec<_> = fs::read_dir(&backup_folder)
            .unwrap()
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .collect();

        assert!(!backup_files.is_empty(), "Backup files should exist.");

        // Ensure the old file has been deleted
        assert!(!old_file_path.exists(), "Old file should have been removed.");

        // Optionally, you can check the content of the backup file
        let backup_file_path = backup_folder.join(backup_files[0].file_name().unwrap());
        let content = fs::read_to_string(backup_file_path).unwrap();
        assert_eq!(content, "Test data", "Backup file content should match the old file.");
    }
}
