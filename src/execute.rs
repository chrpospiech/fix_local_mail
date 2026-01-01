use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

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

pub async fn update_akonadi_db(pool: sqlx::Pool<sqlx::MySql>, id: i64) {
    sqlx::query("DELETE FROM pimitemtable WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();
}

pub async fn trigger_akonadi_sync() -> Result<(), Box<dyn std::error::Error>> {
    let conn = zbus::Connection::session().await?;

    conn.call_method(
        Some("org.freedesktop.Akonadi.Resource.akonadi_maildir_resource_0"),
        "/",
        Some("org.freedesktop.Akonadi.Resource"),
        "synchronize",
        &(),
    )
    .await?;

    Ok(())
}

pub async fn trigger_kmail_quit() -> Result<(), Box<dyn std::error::Error>> {
    let conn = zbus::Connection::session().await?;

    // Quit KMail to force it to reload everything from Akonadi
    // This is the only reliable way to clear KMail's view cache
    match conn
        .call_method(
            Some("org.kde.kmail"),
            "/kmail2/kmail_mainwindow_1",
            Some("org.qtproject.Qt.QWidget"),
            "close",
            &(),
        )
        .await
    {
        Ok(_) => {
            println!("KMail closed successfully. Please restart it manually.");
        }
        Err(e) => {
            eprintln!(
                "Failed to close KMail: {}. You may need to restart KMail manually.",
                e
            );
        }
    }

    Ok(())
}

pub async fn trigger_akonadi_stop() -> Result<(), Box<dyn std::error::Error>> {
    let conn = zbus::Connection::session().await?;

    match conn
        .call_method(
            Some("org.freedesktop.Akonadi.Control"),
            "/ControlManager",
            Some("org.freedesktop.Akonadi.ControlManager"),
            "shutdown",
            &(),
        )
        .await
    {
        Ok(_) => {
            println!("Akonadi server stopped successfully.");
        }
        Err(e) => {
            eprintln!("Failed to stop Akonadi via D-Bus: {}", e);
            eprintln!("You may need to run 'akonadictl stop' manually.");
        }
    }

    Ok(())
}

pub async fn clean_up(stop_akonadi: bool, stop_kmail: bool) {
    // Clean up Akonadi and KMail

    // Trigger Akonadi to synchronize changes
    if let Err(e) = trigger_akonadi_sync().await {
        eprintln!("Failed to trigger Akonadi sync: {}", e);
    }

    if stop_akonadi || stop_kmail {
        // Trigger KMail to refresh all views
        if let Err(e) = trigger_kmail_quit().await {
            eprintln!("Failed to trigger KMail refresh: {}", e);
        }
    }

    if stop_akonadi {
        // Stop Akonadi server
        if let Err(e) = trigger_akonadi_stop().await {
            eprintln!("Failed to stop Akonadi: {}", e);
        }
    }
}
