use tauri::command;

#[cfg(target_os = "macos")]
use {
    objc2::{class, msg_send, runtime::Bool},
    objc2_foundation::NSString,
};

/// Check camera permission.
///
/// # Returns
/// - `bool`: `true` if camera permission is granted, `false` otherwise.
///
/// # Example
/// ```
/// use tauri_plugin_macos_permissions::check_camera_permission;
///
/// let authorized = check_camera_permission().await;
/// println!("Authorized: {}", authorized); // false
/// ```
#[command]
pub async fn check_camera_permission() -> bool {
    #[cfg(target_os = "macos")]
    unsafe {
        let av_media_type = NSString::from_str("vide");
        let status: i32 = msg_send![
            class!(AVCaptureDevice),
            authorizationStatusForMediaType: &*av_media_type
        ];

        status == 3
    }

    #[cfg(not(target_os = "macos"))]
    return true;
}

/// Request camera permission.
///
/// # Example
/// ```
/// use tauri_plugin_macos_permissions::request_camera_permission;
///
/// request_camera_permission().await;
/// ```
#[command]
pub async fn request_camera_permission() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    unsafe {
        let av_media_type = NSString::from_str("vide");
        type CompletionBlock = Option<extern "C" fn(Bool)>;
        let completion_block: CompletionBlock = None;
        let _: () = msg_send![
            class!(AVCaptureDevice),
            requestAccessForMediaType: &*av_media_type,
            completionHandler: completion_block
        ];
    }

    Ok(())
}
