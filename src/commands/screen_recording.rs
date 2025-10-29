use tauri::command;

#[cfg(target_os = "macos")]
#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGPreflightScreenCaptureAccess() -> bool;
    fn CGRequestScreenCaptureAccess() -> bool;
}

/// Check screen recording permission.
///
/// # Returns
/// - `bool`: `true` if screen recording permission are granted, `false` otherwise.
///
/// # Example
/// ```
/// use tauri_plugin_macos_permissions::check_screen_recording_permission;
///
/// let authorized = check_screen_recording_permission().await;
/// println!("Authorized: {}", authorized); // false
/// ```
#[command]
pub async fn check_screen_recording_permission() -> bool {
    #[cfg(target_os = "macos")]
    unsafe {
        CGPreflightScreenCaptureAccess()
    }

    #[cfg(not(target_os = "macos"))]
    return true;
}

/// Request screen recording permission.
///
/// # Example
/// ```
/// use tauri_plugin_macos_permissions::request_screen_recording_permission;
///
/// request_screen_recording_permission().await;
/// ```
#[command]
pub async fn request_screen_recording_permission() {
    #[cfg(target_os = "macos")]
    unsafe {
        CGRequestScreenCaptureAccess();
    }
}
