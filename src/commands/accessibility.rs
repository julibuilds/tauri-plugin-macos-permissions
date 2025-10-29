use tauri::command;

#[cfg(target_os = "macos")]
use macos_accessibility_client::accessibility::{
    application_is_trusted, application_is_trusted_with_prompt,
};

/// Check accessibility permission.
///
/// # Returns
/// - `bool`: `true` if accessibility permission are granted, `false` otherwise.
///
/// # Example
/// ```
/// use tauri_plugin_macos_permissions::check_accessibility_permission;
///
/// let authorized = check_accessibility_permission().await;
/// println!("Authorized: {}", authorized); // false
/// ```
#[command]
pub async fn check_accessibility_permission() -> bool {
    #[cfg(target_os = "macos")]
    return application_is_trusted();

    #[cfg(not(target_os = "macos"))]
    return true;
}

/// Request accessibility permission.
///
/// # Example
/// ```
/// use tauri_plugin_macos_permissions::request_accessibility_permission;
///
/// request_accessibility_permission().await;
/// ```
#[command]
pub async fn request_accessibility_permission() {
    #[cfg(target_os = "macos")]
    application_is_trusted_with_prompt();
}
