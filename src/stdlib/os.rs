//! Python os module implementation
//! 
//! This module provides Python's os module functionality for
//! operating system interface functions.

use crate::PyException;

/// os.execv - execute a program
/// 
/// # Arguments
/// * `program` - Path to the program to execute
/// * `args` - Arguments to pass to the program
/// 
/// # Note
/// This function replaces the current process with the new program.
/// On Unix systems, this uses the actual execv system call.
/// On Windows, this is simulated using process spawn and exit.
#[cfg(unix)]
pub fn execv(program: &str, args: Vec<&str>) -> Result<(), PyException> {
    use std::ffi::CString;
    
    // Convert program path and arguments to C strings
    let program_c = CString::new(program)
        .map_err(|_| crate::value_error("Invalid program path"))?;
    
    let mut args_c: Vec<CString> = Vec::new();
    for arg in args {
        args_c.push(CString::new(arg)
            .map_err(|_| crate::value_error("Invalid argument"))?);
    }
    
    // Convert to raw pointers for execv
    let mut args_ptr: Vec<*const libc::c_char> = args_c.iter()
        .map(|s| s.as_ptr())
        .collect();
    args_ptr.push(std::ptr::null()); // execv expects null-terminated array
    
    // Call execv - this replaces the current process
    unsafe {
        libc::execv(program_c.as_ptr(), args_ptr.as_ptr());
    }
    
    // If we reach here, execv failed
    Err(crate::runtime_error(&format!("execv failed for program: {}", program)))
}

/// os.execv - execute a program (Windows implementation)
#[cfg(windows)]
pub fn execv(program: &str, args: Vec<&str>) -> Result<(), PyException> {
    use std::process::Command;
    
    // On Windows, we simulate execv using process spawn + exit
    let mut cmd = Command::new(program);
    cmd.args(&args);
    
    match cmd.status() {
        Ok(status) => {
            std::process::exit(status.code().unwrap_or(1));
        }
        Err(e) => Err(crate::runtime_error(&format!("Failed to execute program {}: {}", program, e)))
    }
}

/// os.getenv - get environment variable
/// 
/// # Arguments
/// * `key` - Environment variable name
/// 
/// # Returns
/// The value of the environment variable, or None if not found
pub fn getenv(key: &str) -> Option<String> {
    std::env::var(key).ok()
}

/// os.setenv - set environment variable
/// 
/// # Arguments
/// * `key` - Environment variable name
/// * `value` - Environment variable value
pub fn setenv(key: &str, value: &str) {
    unsafe {
        std::env::set_var(key, value);
    }
}

/// os.getcwd - get current working directory
/// 
/// # Returns
/// The current working directory path
pub fn getcwd() -> Result<String, PyException> {
    std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| crate::runtime_error(&format!("Failed to get current directory: {}", e)))
}

/// os.chdir - change current working directory
/// 
/// # Arguments
/// * `path` - New working directory path
pub fn chdir(path: &str) -> Result<(), PyException> {
    std::env::set_current_dir(path)
        .map_err(|e| crate::runtime_error(&format!("Failed to change directory to {}: {}", path, e)))
}

/// os.path submodule
pub mod path {
    //! Python os.path module implementation
    //! 
    //! This submodule provides path manipulation functions using Rust's std::path.

    use std::path::{Path, PathBuf};
    use crate::PyException;

    /// os.path.dirname - return directory name of pathname
    /// 
    /// # Arguments
    /// * `path` - The path to get the directory name of
    /// 
    /// # Returns
    /// The directory portion of the path
    /// 
    /// # Example
    /// ```rust
    /// use stdpython::os::path;
    /// assert_eq!(path::dirname("/home/user/file.txt"), "/home/user");
    /// ```
    pub fn dirname(path: &str) -> String {
        Path::new(path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string())
    }
    
    /// os.path.basename - return base name of pathname
    /// 
    /// # Arguments
    /// * `path` - The path to get the base name of
    /// 
    /// # Returns
    /// The base name portion of the path
    pub fn basename(path: &str) -> String {
        Path::new(path)
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| String::new())
    }
    
    /// os.path.join - join path components
    /// 
    /// # Arguments
    /// * `components` - Path components to join
    /// 
    /// # Returns
    /// The joined path
    pub fn join(components: &[&str]) -> String {
        let mut path = PathBuf::new();
        for component in components {
            path.push(component);
        }
        path.to_string_lossy().to_string()
    }
    
    /// os.path.exists - check if path exists
    /// 
    /// # Arguments
    /// * `path` - Path to check
    /// 
    /// # Returns
    /// true if the path exists, false otherwise
    pub fn exists(path: &str) -> bool {
        Path::new(path).exists()
    }
    
    /// os.path.isfile - check if path is a regular file
    /// 
    /// # Arguments
    /// * `path` - Path to check
    /// 
    /// # Returns
    /// true if the path is a regular file, false otherwise
    pub fn isfile(path: &str) -> bool {
        Path::new(path).is_file()
    }
    
    /// os.path.isdir - check if path is a directory
    /// 
    /// # Arguments
    /// * `path` - Path to check
    /// 
    /// # Returns
    /// true if the path is a directory, false otherwise
    pub fn isdir(path: &str) -> bool {
        Path::new(path).is_dir()
    }
    
    /// os.path.abspath - return absolute path
    /// 
    /// # Arguments
    /// * `path` - Path to make absolute
    /// 
    /// # Returns
    /// The absolute path
    pub fn abspath(path: &str) -> Result<String, PyException> {
        std::fs::canonicalize(path)
            .map(|p| p.to_string_lossy().to_string())
            .map_err(|e| crate::runtime_error(&format!("Failed to get absolute path for {}: {}", path, e)))
    }
}