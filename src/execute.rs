use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::akonadi_ffi;

pub fn ensure_writable_directory(dir: String) {
    let path = Path::new(&dir);

    // Attempt to create the directory if it doesn't exist
    let msg = format!("Failed to create directory {}", dir);
    fs::create_dir_all(path).expect(&msg);

    // Check write permissions by attempting to get metadata
    let metadata = fs::metadata(path).expect("Failed to get metadata");
    let permissions = metadata.permissions();

    // On Unix, ensure owner has write permission
    #[cfg(unix)]
    {
        let mode = permissions.mode();
        if mode & 0o200 == 0 {
            let mut new_permissions = permissions.clone();
            new_permissions.set_mode(mode | 0o200);
            let error = format!("Failed to set write permissions on {}", dir);
            fs::set_permissions(path, new_permissions).expect(&error);
        }
    }
}

pub fn remove_temp_file(file_path: &str) {
    if file_path.contains("/file_db_data/tmp") {
        if let Err(e) = fs::remove_file(file_path) {
            eprintln!("Failed to remove temp file {}: {}", file_path, e);
        }
    }
}

pub fn move_file(source: &str, target: &str) {
    // Ensure both, source and target directory are writable
    if let Some(parent) = std::path::Path::new(&source).parent() {
        ensure_writable_directory(parent.to_string_lossy().to_string());
    }
    if let Some(parent) = std::path::Path::new(&target).parent() {
        ensure_writable_directory(parent.to_string_lossy().to_string());
    }
    // Move the file
    if let Err(e) = std::fs::rename(source, target) {
        eprintln!("Failed to move {} to {}: {}", source, target, e);
    }
}

pub fn update_akonadi_via_lib(item_id: i64, target_path: &str) -> Result<(), String> {
    // Extract remote ID from target path
    let remote_id = std::path::Path::new(target_path)
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or("Failed to extract remote ID from target path")?;

    akonadi_ffi::modify_item(item_id, remote_id)?;

    let some = false;

    // Example condition to delete an item
    if some {
        akonadi_ffi::delete_item(item_id)?;
    }
    Ok(())
}
