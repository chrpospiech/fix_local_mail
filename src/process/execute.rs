// Copyright 2026 fix_local_mail C. Pospiech
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::Result;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub fn ensure_writable_directory(dir: String) -> Result<()> {
    let path = Path::new(&dir);

    // Attempt to create the directory if it doesn't exist
    fs::create_dir_all(path)
        .map_err(|e| anyhow::anyhow!("Failed to create directory {}: {}", dir, e))?;

    // Check write permissions by attempting to get metadata
    let metadata = fs::metadata(path)?;
    let permissions = metadata.permissions();

    // On Unix, ensure owner has write permission
    #[cfg(unix)]
    {
        let mode = permissions.mode();
        if mode & 0o200 == 0 {
            let mut new_permissions = permissions.clone();
            new_permissions.set_mode(mode | 0o200);
            fs::set_permissions(path, new_permissions)?;
        }
    }
    Ok(())
}

pub fn move_file(source: &str, target: &str) -> Result<()> {
    // Ensure both, source and target directory are writable
    if let Some(parent) = std::path::Path::new(&source).parent() {
        ensure_writable_directory(parent.to_string_lossy().to_string())?;
    }
    if let Some(parent) = std::path::Path::new(&target).parent() {
        ensure_writable_directory(parent.to_string_lossy().to_string())?;
    }
    // Move the file
    std::fs::rename(source, target)?;
    Ok(())
}

pub fn delete_file(path: &str) -> Result<()> {
    std::fs::remove_file(path)?;
    Ok(())
}

pub async fn update_akonadi_db(pool: sqlx::Pool<sqlx::MySql>, id: i64) -> Result<()> {
    sqlx::query("DELETE FROM pimitemtable WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(())
}

pub async fn trigger_akonadi_sync() -> Result<()> {
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

pub async fn trigger_kmail_quit() -> Result<()> {
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

pub async fn trigger_akonadi_stop() -> Result<()> {
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

pub async fn clean_up(stop_akonadi: bool, stop_kmail: bool) -> Result<()> {
    // Clean up Akonadi and KMail

    // Trigger Akonadi to synchronize changes
    if let Err(e) = trigger_akonadi_sync().await {
        eprintln!("Failed to trigger Akonadi synchronization: {}", e);
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
    Ok(())
}
