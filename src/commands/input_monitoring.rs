use tauri::command;

#[cfg(target_os = "macos")]
use std::process::Command;

#[cfg(target_os = "macos")]
#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOHIDCheckAccess(request: u32) -> u32;
}

/// Check input monitoring permission.
///
/// # Returns
/// - `bool`: `true` if input monitoring permission is granted, `false` otherwise.
///
/// # Example
/// ```
/// use tauri_plugin_macos_permissions::check_input_monitoring_permission;
///
/// let authorized = check_input_monitoring_permission().await;
/// println!("Authorized: {}", authorized); // false
/// ```
#[command]
pub async fn check_input_monitoring_permission() -> bool {
    #[cfg(target_os = "macos")]
    unsafe {
        let status = IOHIDCheckAccess(1);

        status == 0
    }

    #[cfg(not(target_os = "macos"))]
    return true;
}

/// Request input monitoring permission.
///
/// # Example
/// ```
/// use tauri_plugin_macos_permissions::request_input_monitoring_permission;
///
/// request_input_monitoring_permission().await;
/// ```
#[command]
pub async fn request_input_monitoring_permission() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_ListenEvent")
            .output()
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}
