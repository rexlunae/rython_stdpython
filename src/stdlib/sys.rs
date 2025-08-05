//! Python sys module implementation
//! 
//! This module provides Python's sys module functionality including
//! system-specific parameters and functions.


/// sys.executable - path to the Python executable (property)
/// 
/// In a real Python environment, this would be the path to the Python interpreter.
/// For Rust-compiled Python code, we use the current executable path.
/// 
/// Note: This uses lazy evaluation to get the actual executable path at runtime.
pub static executable: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    std::env::current_exe()
        .map(|path| path.to_string_lossy().to_string())
        .unwrap_or_else(|_| "python".to_string())
});

/// sys.argv - command line arguments (property)
/// 
/// Returns the command line arguments passed to the program.
/// This reflects the actual command line arguments, just like Python's sys.argv.
/// 
/// Note: This uses lazy evaluation to get the actual command line arguments at runtime.
pub static argv: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| {
    std::env::args().collect()
});

/// sys.exit - exit the program (function)
/// 
/// # Arguments
/// * `code` - Exit status code (0 for success, non-zero for error)
/// 
/// # Example
/// ```rust
/// use stdpython::sys;
/// // sys::exit(0); // This would exit the program successfully
/// // sys::exit(1); // This would exit with error code 1
/// ```
pub fn exit(code: i32) -> ! {
    std::process::exit(code);
}

/// sys.platform - platform identifier
/// 
/// Returns a string identifying the platform on which Python is running.
pub fn platform() -> &'static str {
    if cfg!(target_os = "windows") {
        "win32"
    } else if cfg!(target_os = "macos") {
        "darwin"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(unix) {
        "unix"
    } else {
        "unknown"
    }
}

/// sys.version - version information
/// 
/// Returns version information about the Python interpreter.
/// Since this is compiled Python code, we return information about the Rust compiler.
pub fn version() -> String {
    format!("Python-to-Rust compiled code (rustc {})", 
        option_env!("RUSTC_VERSION").unwrap_or("unknown"))
}