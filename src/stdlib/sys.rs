//! Python sys module implementation
//! 
//! This module provides Python's sys module functionality including
//! system-specific parameters and functions. Uses generic traits for
//! maximum flexibility and reusability.

use crate::python_function;


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

/// sys.version_info - version information as a tuple-like structure
/// 
/// Python's version_info is a named tuple with major, minor, micro, etc.
/// For compiled code, we simulate Python version information.
pub static version_info: std::sync::LazyLock<Vec<i32>> = std::sync::LazyLock::new(|| {
    vec![3, 11, 0]  // Simulate Python 3.11.0
});

/// sys.prefix - installation prefix
/// 
/// In Python, this is the directory prefix where Python is installed.
/// For compiled code, we use the executable's directory.
pub static prefix: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(|p| p.to_string_lossy().to_string()))
        .unwrap_or_else(|| "/usr/local".to_string())
});

/// sys.base_prefix - base installation prefix
/// 
/// In Python, this is the base installation prefix (before virtual environments).
/// For simplicity, we make it the same as prefix.
pub static base_prefix: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    prefix.clone()
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

#[cfg(feature = "std")]
python_function! {
    /// sys.exit - exit the program
    pub fn exit<T>(code: T) -> !
    where [T: Into<ExitCode>]
    [signature: (code)]
    [concrete_types: (String) -> !]
    {
        let exit_code = code.into();
        match exit_code {
            ExitCode::Code(c) => std::process::exit(c),
            ExitCode::Message(msg) => {
                eprintln!("{}", msg);
                std::process::exit(1);
            }
        }
    }
}

/// Helper enum to handle both numeric exit codes and string messages
pub enum ExitCode {
    Code(i32),
    Message(String),
}

impl From<i32> for ExitCode {
    fn from(code: i32) -> Self {
        ExitCode::Code(code)
    }
}

impl From<&str> for ExitCode {
    fn from(message: &str) -> Self {
        ExitCode::Message(message.to_string())
    }
}

impl From<String> for ExitCode {
    fn from(message: String) -> Self {
        ExitCode::Message(message)
    }
}

// Add support for other common integer types
impl From<i8> for ExitCode {
    fn from(code: i8) -> Self {
        ExitCode::Code(code as i32)
    }
}

impl From<u8> for ExitCode {
    fn from(code: u8) -> Self {
        ExitCode::Code(code as i32)
    }
}

impl From<i16> for ExitCode {
    fn from(code: i16) -> Self {
        ExitCode::Code(code as i32)
    }
}

impl From<u16> for ExitCode {
    fn from(code: u16) -> Self {
        ExitCode::Code(code as i32)
    }
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

python_function! {
    /// sys.platform - platform identifier
    pub fn platform() -> &'static str
    [signature: ()]
    [concrete_types: () -> &'static str]
    {
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
}

python_function! {
    /// sys.version - version information
    pub fn version() -> String
    [signature: ()]
    [concrete_types: () -> String]
    {
        format!("Python-to-Rust compiled code (rustc {})", 
            option_env!("RUSTC_VERSION").unwrap_or("unknown"))
    }
}

python_function! {
    /// Generic helper: Get executable path
    pub fn get_executable() -> String
    [signature: ()]
    [concrete_types: () -> String]
    {
        #[cfg(feature = "std")]
        {
            executable.clone()
        }
        #[cfg(not(feature = "std"))]
        {
            "python".to_string()
        }
    }
}

python_function! {
    /// Generic helper: Get command line arguments
    pub fn get_argv() -> Vec<String>
    [signature: ()]
    [concrete_types: () -> Vec<String>]
    {
        #[cfg(feature = "std")]
        {
            argv.iter().cloned().collect()
        }
        #[cfg(not(feature = "std"))]
        {
            vec!["python".to_string()]
        }
    }
}

python_function! {
    /// Generic helper: Get platform identifier
    pub fn get_platform() -> &'static str
    [signature: ()]
    [concrete_types: () -> &'static str]
    {
        platform()
    }
}