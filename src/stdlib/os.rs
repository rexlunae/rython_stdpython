//! Python os module implementation
//! 
//! This module provides Python's os module functionality for
//! operating system interface functions.
//!
//! Note: This module is only available with the `std` feature enabled,
//! as it requires operating system functionality.

use crate::{PyException, AsStrLike, AsPathLike, python_function};
use std::collections::HashMap;
use std::sync::LazyLock;

python_function! {
    /// os.execv - execute a program (generic version using traits)
    /// 
    /// # Arguments
    /// * `program` - Path to the program to execute (any string-like type)
    /// * `args` - Arguments to pass to the program (any collection of string-like types)
    /// 
    /// # Note
    /// This function replaces the current process with the new program.
    pub fn execv_mixed<P, A, S>(program: P, args: A) -> Result<(), PyException> 
    where [P: AsPathLike, A: IntoIterator<Item = S>, S: AsStrLike]
    [signature: (program, args)]
    [concrete_types: (String, Vec<String>) -> Result<(), crate::PyException>]
    {
        // Convert to owned strings first to avoid lifetime issues
        let owned_args: Vec<String> = args.into_iter().map(|s| s.as_str_like().to_string()).collect();
        let str_args: Vec<&str> = owned_args.iter().map(|s| s.as_str()).collect();
        execv(program.as_path_like(), str_args)
    }
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

python_function! {
    /// os.getenv - get environment variable
    pub fn getenv<K>(key: K) -> Option<String>
    where [K: AsStrLike]
    [signature: (key)]
    [concrete_types: (String) -> Option<String>]
    {
        std::env::var(key.as_str_like()).ok()
    }
}

python_function! {
    /// os.setenv - set environment variable
    pub fn setenv<K, V>(key: K, value: V) -> ()
    where [K: AsStrLike, V: AsStrLike]
    [signature: (key, value)]
    [concrete_types: (String, String) -> ()]
    {
        unsafe {
            std::env::set_var(key.as_str_like(), value.as_str_like());
        }
    }
}

python_function! {
    /// os.getcwd - get current working directory
    pub fn getcwd() -> Result<String, PyException>
    [signature: ()]
    [concrete_types: () -> Result<String, crate::PyException>]
    {
        std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .map_err(|e| crate::runtime_error(&format!("Failed to get current directory: {}", e)))
    }
}

python_function! {
    /// os.chdir - change current working directory
    pub fn chdir<P>(path: P) -> Result<(), PyException>
    where [P: AsPathLike]
    [signature: (path)]
    [concrete_types: (String) -> Result<(), crate::PyException>]
    {
        std::env::set_current_dir(path.as_path_like())
            .map_err(|e| crate::runtime_error(&format!("Failed to change directory to {}: {}", path.as_path_like(), e)))
    }
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
    use crate::{PyException, AsPathLike, python_function};
    
    /// os.path.sep - path separator for the current platform
    pub static sep: &str = if cfg!(target_os = "windows") { "\\" } else { "/" };

    python_function! {
        /// os.path.dirname - return directory name of pathname
        pub fn dirname<P>(path: P) -> String
        where [P: AsPathLike]
        [signature: (path)]
        [concrete_types: (String) -> String]
        {
            Path::new(path.as_path_like())
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| ".".to_string())
        }
    }
    
    python_function! {
        /// os.path.basename - return base name of pathname
        pub fn basename<P>(path: P) -> String
        where [P: AsPathLike]
        [signature: (path)]
        [concrete_types: (String) -> String]
        {
            Path::new(path.as_path_like())
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
                .unwrap_or_else(|| String::new())
        }
    }
    
    python_function! {
        /// os.path.join - join path components
        pub fn join<P1, P2>(path1: P1, path2: P2) -> String
        where [P1: AsPathLike, P2: AsPathLike]
        [signature: (path1, path2)]
        [concrete_types: (String, String) -> String]
        {
            let mut path = PathBuf::from(path1.as_path_like());
            path.push(path2.as_path_like());
            path.to_string_lossy().to_string()
        }
    }
    
    python_function! {
        /// os.path.join - join path components (3 arguments version)
        pub fn join3<P1, P2, P3>(path1: P1, path2: P2, path3: P3) -> String
        where [P1: AsPathLike, P2: AsPathLike, P3: AsPathLike]
        [signature: (path1, path2, path3)]
        [concrete_types: (String, String, String) -> String]
        {
            let mut path = PathBuf::from(path1.as_path_like());
            path.push(path2.as_path_like());
            path.push(path3.as_path_like());
            path.to_string_lossy().to_string()
        }
    }
    
    python_function! {
        /// os.path.join - join path components (variable arguments version)
        pub fn join_many<I, P>(components: I) -> String
        where [I: IntoIterator<Item = P>, P: AsPathLike]
        [signature: (components)]
        [concrete_types: (Vec<String>) -> String]
        {
            let mut path = PathBuf::new();
            for component in components {
                path.push(component.as_path_like());
            }
            path.to_string_lossy().to_string()
        }
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
    
    python_function! {
        /// os.path.exists - check if path exists
        pub fn exists<P>(path: P) -> bool
        where [P: AsPathLike]
        [signature: (path)]
        [concrete_types: (String) -> bool]
        {
            Path::new(path.as_path_like()).exists()
        }
    }
    
    python_function! {
        /// os.path.isfile - check if path is a regular file
        pub fn isfile<P>(path: P) -> bool
        where [P: AsRef<str>]
        [signature: (path)]
        [concrete_types: (String) -> bool]
        {
            Path::new(path.as_ref()).is_file()
        }
    }
    
    python_function! {
        /// os.path.isdir - check if path is a directory
        pub fn isdir<P>(path: P) -> bool
        where [P: AsRef<str>]
        [signature: (path)]
        [concrete_types: (String) -> bool]
        {
            Path::new(path.as_ref()).is_dir()
        }
    }
    
    python_function! {
        /// os.path.abspath - return absolute path
        pub fn abspath<P>(path: P) -> Result<String, PyException>
        where [P: AsRef<str>]
        [signature: (path)]
        [concrete_types: (String) -> Result<String, crate::PyException>]
        {
            std::fs::canonicalize(path.as_ref())
                .map(|p| p.to_string_lossy().to_string())
                .map_err(|e| crate::runtime_error(format!("Failed to get absolute path for {}: {}", path.as_ref(), e)))
        }
    }
    
    python_function! {
        /// os.path.relpath - return relative path
        pub fn relpath<P>(path: P, start: Option<String>) -> Result<String, PyException>
        where [P: AsRef<str>]
        [signature: (path, start=None)]
        [concrete_types: (String, Option<String>) -> Result<String, crate::PyException>]
        {
            let path_buf = Path::new(path.as_ref());
            let start_string = start.unwrap_or_else(|| ".".to_string());
            let start_path = Path::new(&start_string);
            
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
}