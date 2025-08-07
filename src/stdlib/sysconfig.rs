//! Python sysconfig module placeholder implementation
//! 
//! This module provides placeholders for Python's sysconfig module functionality.
//! The sysconfig module provides access to Python's configuration information.
//!
//! Note: This module is only available with the `std` feature enabled.

use crate::{PyException, python_function};
use std::collections::HashMap;

/// sysconfig.get_config_vars - get configuration variables
/// 
/// This is a placeholder implementation that returns basic configuration variables.
/// In a real Python environment, this would return the actual build configuration.
pub fn get_config_vars() -> HashMap<String, String> {
    let mut config = HashMap::new();
    
    // Basic Python configuration simulation
    config.insert("LIBDIR".to_string(), "/usr/local/lib".to_string());
    config.insert("INCLUDEDIR".to_string(), "/usr/local/include".to_string());
    config.insert("SO".to_string(), ".so".to_string());
    config.insert("EXT_SUFFIX".to_string(), ".so".to_string());
    config.insert("CC".to_string(), "gcc".to_string());
    config.insert("CXX".to_string(), "g++".to_string());
    config.insert("OPT".to_string(), "-DNDEBUG -g -fwrapv -O3 -Wall".to_string());
    config.insert("CFLAGS".to_string(), "-Wno-unused-result -Wsign-compare".to_string());
    config.insert("CCSHARED".to_string(), "-fPIC".to_string());
    config.insert("LDSHARED".to_string(), "gcc -shared".to_string());
    config.insert("SHLIB_SUFFIX".to_string(), ".so".to_string());
    config.insert("AR".to_string(), "ar".to_string());
    config.insert("ARFLAGS".to_string(), "rcs".to_string());
    config.insert("LINKFORSHARED".to_string(), "-Xlinker -export-dynamic".to_string());
    
    config
}

/// sysconfig.get_config_var - get a single configuration variable
/// 
/// # Arguments
/// * `name` - Name of the configuration variable
/// 
/// # Returns
/// The configuration variable value, or None if not found
pub fn get_config_var<N: AsRef<str>>(name: N) -> Option<String> {
    get_config_vars().get(name.as_ref()).cloned()
}

/// sysconfig.get_path - get a path by name
/// 
/// This is a placeholder implementation for common Python paths.
/// 
/// # Arguments
/// * `name` - Path name (e.g., "stdlib", "platstdlib", "purelib", etc.)
/// 
/// # Returns
/// The path string
pub fn get_path<N: AsRef<str>>(name: N) -> Result<String, PyException> {
    match name.as_ref() {
        "stdlib" => Ok("/usr/local/lib/python3.11".to_string()),
        "platstdlib" => Ok("/usr/local/lib/python3.11".to_string()),
        "purelib" => Ok("/usr/local/lib/python3.11/site-packages".to_string()),
        "platlib" => Ok("/usr/local/lib/python3.11/site-packages".to_string()),
        "include" => Ok("/usr/local/include/python3.11".to_string()),
        "data" => Ok("/usr/local".to_string()),
        "scripts" => Ok("/usr/local/bin".to_string()),
        _ => Err(crate::key_error(format!("Unknown path name: {}", name.as_ref()))),
    }
}

/// sysconfig.get_paths - get all paths
/// 
/// Returns a dictionary of all known paths.
pub fn get_paths() -> HashMap<String, String> {
    let mut paths = HashMap::new();
    
    paths.insert("stdlib".to_string(), "/usr/local/lib/python3.11".to_string());
    paths.insert("platstdlib".to_string(), "/usr/local/lib/python3.11".to_string());
    paths.insert("purelib".to_string(), "/usr/local/lib/python3.11/site-packages".to_string());
    paths.insert("platlib".to_string(), "/usr/local/lib/python3.11/site-packages".to_string());
    paths.insert("include".to_string(), "/usr/local/include/python3.11".to_string());
    paths.insert("data".to_string(), "/usr/local".to_string());
    paths.insert("scripts".to_string(), "/usr/local/bin".to_string());
    
    paths
}

/// sysconfig.get_scheme_names - get available scheme names
/// 
/// Returns a list of available installation scheme names.
pub fn get_scheme_names() -> Vec<String> {
    vec![
        "posix_prefix".to_string(),
        "posix_home".to_string(),
        "posix_user".to_string(),
        "nt".to_string(),
        "nt_user".to_string(),
        "osx_framework_user".to_string(),
    ]
}

/// sysconfig.get_platform - get the platform string
/// 
/// Returns a platform identifier string.
pub fn get_platform() -> String {
    if cfg!(target_os = "windows") {
        if cfg!(target_arch = "x86_64") {
            "win-amd64".to_string()
        } else if cfg!(target_arch = "x86") {
            "win32".to_string()
        } else {
            "windows-unknown".to_string()
        }
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "x86_64") {
            "macosx-10.9-x86_64".to_string()
        } else if cfg!(target_arch = "aarch64") {
            "macosx-11.0-arm64".to_string()
        } else {
            "darwin-unknown".to_string()
        }
    } else if cfg!(target_os = "linux") {
        if cfg!(target_arch = "x86_64") {
            "linux-x86_64".to_string()
        } else if cfg!(target_arch = "aarch64") {
            "linux-aarch64".to_string()
        } else {
            "linux-unknown".to_string()
        }
    } else {
        "unknown-platform".to_string()
    }
}

python_function! {
    /// sysconfig.is_python_build - check if this is a Python build
    pub fn is_python_build() -> bool
    [signature: ()]
    [concrete_types: () -> bool]
    {
        // For compiled Python code, this is not a Python build
        // Return false as the code has been compiled to Rust
        false
    }
}