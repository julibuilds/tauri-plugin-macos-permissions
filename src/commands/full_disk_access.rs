use tauri::{command, AppHandle, Runtime};

#[cfg(target_os = "macos")]
use {
    std::{fs::read_dir, process::Command},
    tauri::Manager,
};

/// Check full disk access permission.
///
/// # Returns
/// - `bool`: `true` if full disk access permission are granted, `false` otherwise.
///
/// # Example
/// ```
/// use tauri_plugin_macos_permissions::check_full_disk_access_permission;
///
/// let authorized = check_full_disk_access_permission(app_handle).await;
/// println!("Authorized: {}", authorized); // false
/// ```
#[command]
pub async fn check_full_disk_access_permission<R: Runtime>(app_handle: AppHandle<R>) -> bool {
    #[cfg(target_os = "macos")]
    {
        // Reference: https://github.com/inket/FullDiskAccess/blob/846e04ea2b84fce843f47d7e7f3421189221829c/Sources/FullDiskAccess/FullDiskAccess.swift#L46
        let check_dirs = vec!["Library/Containers/com.apple.stocks", "Library/Safari"];

        if let Ok(home_dir) = app_handle.path().home_dir() {
            for check_dir in check_dirs.iter() {
                if read_dir(&home_dir.join(check_dir)).is_ok() {
                    return true;
                }
            }
        }

        false
    }

    #[cfg(not(target_os = "macos"))]
    {
        let _ = app_handle;

        true
    }
}

/// Request full disk access permission.
///
/// # Example
/// ```
/// use tauri_plugin_macos_permissions::request_full_disk_access_permission;
///
/// request_full_disk_access_permission().await;
/// ```
#[command]
pub async fn request_full_disk_access_permission() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles")
            .output()
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}
