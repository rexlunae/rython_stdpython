//! Python sys module implementation
//! 
//! This module provides Python's sys module functionality including
//! system-specific parameters and functions. Uses generic traits for
//! maximum flexibility and reusability.



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

/// sys.exit - exit the program (generic function)
/// 
/// # Arguments
/// * `code` - Exit status code (anything convertible to i32)
/// 
/// # Example
/// ```rust
/// use stdpython::sys;
/// // sys::exit(0i32); // This would exit the program successfully
/// // sys::exit(1u8);  // This would exit with error code 1
/// ```
#[cfg(feature = "std")]
pub fn exit<T>(code: T) -> ! 
where
    T: Into<i32>,
{
    std::process::exit(code.into());
}

/// sys.exit - no-std version (panics instead of exiting)
/// 
/// In no-std environments, we cannot actually exit the process,
/// so we panic with the exit code information instead.
#[cfg(not(feature = "std"))]
pub fn exit<T>(code: T) -> ! 
where
    T: Into<i32> + core::fmt::Display,
{
    panic!("sys.exit called with code: {}", code);
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

/// Generic helper: Get executable path as any string-like type
/// 
/// This allows callers to get the executable path in their preferred string format
pub fn get_executable<T>() -> T
where
    T: From<String>,
{
    #[cfg(feature = "std")]
    {
        T::from(executable.clone())
    }
    #[cfg(not(feature = "std"))]
    {
        T::from("python".to_string())
    }
}

/// Generic helper: Get command line arguments as any collection type
/// 
/// This allows callers to get argv in their preferred collection format
pub fn get_argv<T>() -> T
where
    T: FromIterator<String>,
{
    #[cfg(feature = "std")]
    {
        argv.iter().cloned().collect()
    }
    #[cfg(not(feature = "std"))]
    {
        vec!["python".to_string()].into_iter().collect()
    }
}

/// Generic helper: Get platform identifier as any string-like type
pub fn get_platform<T>() -> T
where
    T: From<&'static str>,
{
    T::from(platform())
}