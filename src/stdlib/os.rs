//! Python os module implementation
//! 
//! This module provides Python's os module functionality for
//! operating system interface functions.
//!
//! Note: This module is only available with the `std` feature enabled,
//! as it requires operating system functionality.

use crate::{PyException, AsStrLike, AsPathLike};
use std::collections::HashMap;
use std::sync::LazyLock;

/// os.execv - execute a program (generic version using traits)
/// 
/// # Arguments
/// * `program` - Path to the program to execute (any string-like type)
/// * `args` - Arguments to pass to the program (any collection of string-like types)
/// 
/// # Note
/// This function replaces the current process with the new program.
pub fn execv_mixed<P, A, S>(program: P, args: A) -> Result<(), PyException> 
where
    P: AsPathLike,
    A: IntoIterator<Item = S>,
    S: AsStrLike,
{
    // Convert to owned strings first to avoid lifetime issues
    let owned_args: Vec<String> = args.into_iter().map(|s| s.as_str_like().to_string()).collect();
    let str_args: Vec<&str> = owned_args.iter().map(|s| s.as_str()).collect();
    execv(program.as_path_like(), str_args)
}

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
pub fn execv<P: AsRef<str>, A: AsRef<str>>(program: P, args: Vec<A>) -> Result<(), PyException> {
    use std::ffi::CString;
    
    // Convert program path and arguments to C strings
    let program_c = CString::new(program.as_ref())
        .map_err(|_| crate::value_error("Invalid program path"))?;
    
    let mut args_c: Vec<CString> = Vec::new();
    for arg in &args {
        args_c.push(CString::new(arg.as_ref())
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
    Err(crate::runtime_error(format!("execv failed for program: {}", program.as_ref())))
}

/// os.execv - execute a program (Windows implementation)
#[cfg(windows)]
pub fn execv<P: AsRef<str>, A: AsRef<str>>(program: P, args: Vec<A>) -> Result<(), PyException> {
    use std::process::Command;
    
    // On Windows, we simulate execv using process spawn + exit
    let mut cmd = Command::new(program.as_ref());
    cmd.args(args.iter().map(|a| a.as_ref()));
    
    match cmd.status() {
        Ok(status) => {
            std::process::exit(status.code().unwrap_or(1));
        }
        Err(e) => Err(crate::runtime_error(format!("Failed to execute program {}: {}", program.as_ref(), e)))
    }
}

/// os.getenv - get environment variable (generic version)
/// 
/// # Arguments
/// * `key` - Environment variable name (any string-like type)
/// 
/// # Returns
/// The value of the environment variable in the requested type, or None if not found
pub fn getenv<K, R>(key: K) -> Option<R>
where
    K: AsStrLike,
    R: From<String>,
{
    std::env::var(key.as_str_like()).ok().map(R::from)
}

/// os.setenv - set environment variable (generic version)
/// 
/// # Arguments
/// * `key` - Environment variable name (any string-like type)
/// * `value` - Environment variable value (any string-like type)
pub fn setenv<K, V>(key: K, value: V)
where
    K: AsStrLike,
    V: AsStrLike,
{
    unsafe {
        std::env::set_var(key.as_str_like(), value.as_str_like());
    }
}

/// os.getcwd - get current working directory (generic version)
/// 
/// # Returns
/// The current working directory path in the requested type
pub fn getcwd<R>() -> Result<R, PyException>
where
    R: From<String>,
{
    std::env::current_dir()
        .map(|p| R::from(p.to_string_lossy().to_string()))
        .map_err(|e| crate::runtime_error(&format!("Failed to get current directory: {}", e)))
}

/// os.chdir - change current working directory (generic version)
/// 
/// # Arguments
/// * `path` - New working directory path (any path-like type)
pub fn chdir<P>(path: P) -> Result<(), PyException>
where
    P: AsPathLike,
{
    std::env::set_current_dir(path.as_path_like())
        .map_err(|e| crate::runtime_error(&format!("Failed to change directory to {}: {}", path.as_path_like(), e)))
}

/// os.environ - environment variables dictionary
/// 
/// This provides access to the current environment variables.
/// Note: This uses lazy evaluation to get the actual environment at runtime.
pub static environ: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    std::env::vars().collect()
});

/// os.name - operating system name
/// 
/// This provides the name of the operating system, similar to Python's os.name.
pub static name: LazyLock<&'static str> = LazyLock::new(|| {
    if cfg!(target_os = "windows") {
        "nt"
    } else {
        "posix"
    }
});

// Compatibility aliases for generated code
// Note: Functions are already public in this module, no need to re-export

/// os.path submodule
pub mod path {
    //! Python os.path module implementation
    //! 
    //! This submodule provides path manipulation functions using Rust's std::path.

    use std::path::{Path, PathBuf};
    use crate::{PyException, AsPathLike};
    
    /// os.path.sep - path separator for the current platform
    pub static sep: &str = if cfg!(target_os = "windows") { "\\" } else { "/" };

    /// os.path.dirname - return directory name of pathname (generic version)
    /// 
    /// # Arguments
    /// * `path` - The path to get the directory name of (any path-like type)
    /// 
    /// # Returns
    /// The directory portion of the path in the requested type
    /// 
    /// # Example
    /// ```rust
    /// use stdpython::os::path;
    /// let result: String = path::dirname("/home/user/file.txt");
    /// assert_eq!(result, "/home/user");
    /// ```
    pub fn dirname<P, R>(path: P) -> R
    where
        P: AsPathLike,
        R: From<String>,
    {
        let dir = Path::new(path.as_path_like())
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string());
        R::from(dir)
    }
    
    /// os.path.basename - return base name of pathname (generic version)
    /// 
    /// # Arguments
    /// * `path` - The path to get the base name of (any path-like type)
    /// 
    /// # Returns
    /// The base name portion of the path in the requested type
    pub fn basename<P, R>(path: P) -> R
    where
        P: AsPathLike,
        R: From<String>,
    {
        let name = Path::new(path.as_path_like())
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| String::new());
        R::from(name)
    }
    
    /// os.path.join - join path components (generic version)
    /// 
    /// # Arguments
    /// * `components` - Path components to join (any iterable of path-like types)
    /// 
    /// # Returns
    /// The joined path in the requested type
    pub fn join<I, P, R>(components: I) -> R
    where
        I: IntoIterator<Item = P>,
        P: AsPathLike,
        R: From<String>,
    {
        let mut path = PathBuf::new();
        for component in components {
            path.push(component.as_path_like());
        }
        R::from(path.to_string_lossy().to_string())
    }
    
    /// os.path.join - variadic version for compatibility with Python's os.path.join
    /// 
    /// This function accepts individual arguments like Python's os.path.join(a, b, c, ...)
    /// 
    /// # Arguments
    /// * `first` - First path component
    /// * `rest` - Additional path components (variadic)
    /// 
    /// # Returns
    /// The joined path as a String
    pub fn join_paths<P: AsPathLike>(first: P, rest: &[P]) -> String {
        let mut path = PathBuf::from(first.as_path_like());
        for component in rest {
            path.push(component.as_path_like());
        }
        path.to_string_lossy().to_string()
    }
    
    /// os.path.exists - check if path exists (generic version)
    /// 
    /// # Arguments
    /// * `path` - Path to check (any path-like type)
    /// 
    /// # Returns
    /// true if the path exists, false otherwise
    pub fn exists<P>(path: P) -> bool
    where
        P: AsPathLike,
    {
        Path::new(path.as_path_like()).exists()
    }
    
    /// os.path.isfile - check if path is a regular file
    /// 
    /// # Arguments
    /// * `path` - Path to check
    /// 
    /// # Returns
    /// true if the path is a regular file, false otherwise
    pub fn isfile<P: AsRef<str>>(path: P) -> bool {
        Path::new(path.as_ref()).is_file()
    }
    
    /// os.path.isdir - check if path is a directory
    /// 
    /// # Arguments
    /// * `path` - Path to check
    /// 
    /// # Returns
    /// true if the path is a directory, false otherwise
    pub fn isdir<P: AsRef<str>>(path: P) -> bool {
        Path::new(path.as_ref()).is_dir()
    }
    
    /// os.path.abspath - return absolute path
    /// 
    /// # Arguments
    /// * `path` - Path to make absolute
    /// 
    /// # Returns
    /// The absolute path
    pub fn abspath<P: AsRef<str>>(path: P) -> Result<String, PyException> {
        std::fs::canonicalize(path.as_ref())
            .map(|p| p.to_string_lossy().to_string())
            .map_err(|e| crate::runtime_error(format!("Failed to get absolute path for {}: {}", path.as_ref(), e)))
    }
    
    /// os.path.relpath - return relative path
    /// 
    /// # Arguments
    /// * `path` - Path to make relative
    /// * `start` - Start directory for relative path calculation (optional)
    /// 
    /// # Returns
    /// The relative path
    pub fn relpath<P: AsRef<str>>(path: P, start: Option<&str>) -> Result<String, PyException> {
        let path_buf = Path::new(path.as_ref());
        let start_path = match start {
            Some(s) => Path::new(s),
            None => Path::new("."),
        };
        
        match path_buf.strip_prefix(start_path) {
            Ok(relative) => Ok(relative.to_string_lossy().to_string()),
            Err(_) => {
                // If stripping prefix fails, try to calculate relative path manually
                // This is a simplified version - a full implementation would handle more cases
                Ok(path.as_ref().to_string())
            }
        }
    }
}