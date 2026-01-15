//! # utilities
//!
//! utilities functions
use std::env;

/// what_panic logs the panic information, use `cat /tmp/what_panic.log` to check
/// # Returns
///
/// * Write panic information to /tmp/what_panic.log
///
/// # Examples
/// ```rust,ignore
/// what_panic();
/// ```
///
pub fn what_panic() {
    std::panic::set_hook(Box::new(|info| {
        use std::io::Write;
        let _ = writeln!(
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("/tmp/what_panic.log")
                .unwrap(),
            "PANIC: {}",
            info
        );
    }))
}
/// format counter to second;
/// # Arguments
///
/// * `total_seconds`:f32 - the total second
///
/// # Returns
///
/// *  String: format to ss.ms
///
/// # Examples
/// ```
/// let time_string = time_format_to_s_m(10.1);
/// assert_eq!(time_string, "10.1");
/// ```
///
pub fn time_format_to_s_m(total_seconds: f32) -> String {
    let seconds = total_seconds as u32;
    let tenths_seconds = ((total_seconds * 10.0) % 10.0) as u32;
    format!("{:02}.{}", seconds, tenths_seconds)
}

/// Get absolute path of the resource files
/// Place assets to Workspace/Contents/Resources
/// Keep consistent in debug mode, release mode and macOS app package
/// Read from ./Contents/Resources
///
/// # Arguments
///
/// * `input_path`:&str - the target file path
///
/// # Returns
///
/// *  String - patch to absolute path
///
/// # Examples
/// ```rust,no_run
/// let path = &get_resource_path_str("assets/images/logo.png");
/// ```
///
pub fn get_resource_path_str(relative_path: &str) -> String {
    let exe_path = env::current_exe().expect("Read failed!");
    // get the app root
    let app_root = exe_path
        .ancestors()
        .nth(3)
        .expect("Failed to find .app root");
    // set Contents/Resources
    let resources_dir = app_root.join("Contents").join("Resources");

    // patch the input
    let full_path = resources_dir.join(relative_path);
    full_path
        .canonicalize()
        .unwrap_or(full_path)
        .to_str()
        .expect("Not UTF-8")
        .to_string()
}
