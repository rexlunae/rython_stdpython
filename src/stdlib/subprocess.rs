//! Python subprocess module implementation
//! 
//! This module provides Python's subprocess module functionality for
//! spawning new processes and connecting to their input/output/error pipes.
//!
//! Note: This module is only available with the `std` feature enabled,
//! as it requires process spawning functionality.

use std::process::Command;
use std::collections::HashMap;
use crate::{PyException, AsStrLike, AsPathLike, AsEnvLike};

/// Result of a subprocess run
/// 
/// This struct represents the result of running a subprocess,
/// similar to Python's subprocess.CompletedProcess.
#[derive(Debug, Clone)]
pub struct CompletedProcess {
    /// Exit status of the process
    pub returncode: i32,
    /// Standard output of the process (if captured)
    pub stdout: Option<String>,
    /// Standard error of the process (if captured)
    pub stderr: Option<String>,
    /// Command and arguments that were executed
    pub args: Vec<String>,
}

impl CompletedProcess {
    /// Create a new CompletedProcess with the given return code
    pub fn new(args: Vec<String>, returncode: i32) -> Self {
        Self { 
            returncode,
            stdout: None,
            stderr: None, 
            args,
        }
    }
    
    /// Create a new CompletedProcess with output
    pub fn with_output(args: Vec<String>, returncode: i32, stdout: String, stderr: String) -> Self {
        Self {
            returncode,
            stdout: Some(stdout),
            stderr: Some(stderr),
            args,
        }
    }
}

/// subprocess.run - run a command (generic version using traits)
/// 
/// # Arguments
/// * `args` - Command and arguments to run (any collection of string-like types)
/// * `cwd` - Working directory for the command (optional path-like type)
/// 
/// # Returns
/// A CompletedProcess instance with the result
pub fn run_mixed_args<A, S, C>(args: A, cwd: Option<C>) -> Result<CompletedProcess, PyException> 
where
    A: IntoIterator<Item = S>,
    S: AsStrLike,
    C: AsPathLike,
{
    // Convert to owned strings first to avoid lifetime issues
    let owned_args: Vec<String> = args.into_iter().map(|s| s.as_str_like().to_string()).collect();
    let str_args: Vec<&str> = owned_args.iter().map(|s| s.as_str()).collect();
    let str_cwd = cwd.as_ref().map(|c| c.as_path_like());
    run_with_env_str(str_args, str_cwd, None)
}

/// subprocess.run - run a command
/// 
/// # Arguments
/// * `args` - Command and arguments to run
/// * `cwd` - Working directory for the command (optional)
/// 
/// # Returns
/// A CompletedProcess instance with the result
/// 
/// # Example
/// ```rust
/// use stdpython::subprocess;
/// let result = subprocess::run(vec!["echo", "hello"], None::<&str>);
/// // result.returncode should be 0 for success
/// ```
pub fn run<A: AsRef<str>, C: AsRef<str>>(args: Vec<A>, cwd: Option<C>) -> Result<CompletedProcess, PyException> {
    let str_args: Vec<&str> = args.iter().map(|a| a.as_ref()).collect();
    let str_cwd = cwd.as_ref().map(|c| c.as_ref());
    run_with_env_str(str_args, str_cwd, None)
}

/// subprocess.run with environment variables - run a command with custom environment (generic)
/// 
/// # Arguments
/// * `args` - Command and arguments to run (any collection of string-like types)
/// * `cwd` - Working directory for the command (optional path-like type)
/// * `env` - Environment variables for the command (optional env-like type)
/// 
/// # Returns
/// A CompletedProcess instance with the result
pub fn run_with_env_generic<A, S, C, E, K, V>(
    args: A, 
    cwd: Option<C>, 
    env: Option<E>
) -> Result<CompletedProcess, PyException> 
where
    A: IntoIterator<Item = S>,
    S: AsStrLike,
    C: AsPathLike,
    E: AsEnvLike<K, V>,
    K: AsRef<str>,
    V: AsRef<str>,
{
    // Convert to owned strings first to avoid lifetime issues
    let owned_args: Vec<String> = args.into_iter().map(|s| s.as_str_like().to_string()).collect();
    let str_args: Vec<&str> = owned_args.iter().map(|s| s.as_str()).collect();
    let str_cwd = cwd.as_ref().map(|c| c.as_path_like());
    let env_map = env.as_ref().map(|e| {
        let env_like = e.as_env_like();
        let mut hash_map = HashMap::new();
        for (k, v) in env_like {
            hash_map.insert(k.to_string(), v.to_string());
        }
        hash_map
    });
    run_with_env_str(str_args, str_cwd, env_map.as_ref())
}

/// subprocess.run with environment variables - run a command with custom environment
/// 
/// # Arguments
/// * `args` - Command and arguments to run
/// * `cwd` - Working directory for the command (optional)
/// * `env` - Environment variables for the command (optional)
/// 
/// # Returns
/// A CompletedProcess instance with the result
pub fn run_with_env_str(args: Vec<&str>, cwd: Option<&str>, env: Option<&HashMap<String, String>>) -> Result<CompletedProcess, PyException> {
    if args.is_empty() {
        return Err(crate::value_error("Empty command"));
    }
    
    let mut command = Command::new(&args[0]);
    
    // Add arguments
    if args.len() > 1 {
        command.args(&args[1..]);
    }
    
    // Set working directory if provided
    if let Some(dir) = cwd {
        command.current_dir(dir);
    }
    
    // Set environment variables if provided
    if let Some(env_vars) = env {
        command.envs(env_vars);
    }
    
    // Execute the command
    match command.status() {
        Ok(status) => {
            let returncode = status.code().unwrap_or(if status.success() { 0 } else { 1 });
            let args_owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
            Ok(CompletedProcess::new(args_owned, returncode))
        }
        Err(e) => {
            Err(crate::runtime_error(&format!("Failed to execute command '{}': {}", args[0], e)))
        }
    }
}

/// subprocess.run with output capture
/// 
/// # Arguments
/// * `args` - Command and arguments to run
/// * `cwd` - Working directory for the command (optional)
/// * `capture_output` - Whether to capture stdout and stderr
/// 
/// # Returns
/// A CompletedProcess instance with the result including captured output
pub fn run_with_output<A: AsRef<str>, C: AsRef<str>>(args: Vec<A>, cwd: Option<C>, capture_output: bool) -> Result<CompletedProcess, PyException> {
    if args.is_empty() {
        return Err(crate::value_error("Empty command"));
    }
    
    let str_args: Vec<&str> = args.iter().map(|a| a.as_ref()).collect();
    let str_cwd = cwd.as_ref().map(|c| c.as_ref());
    
    let mut command = Command::new(&str_args[0]);
    
    // Add arguments
    if str_args.len() > 1 {
        command.args(&str_args[1..]);
    }
    
    // Set working directory if provided
    if let Some(dir) = str_cwd {
        command.current_dir(dir);
    }
    
    let args_owned: Vec<String> = str_args.iter().map(|s| s.to_string()).collect();
    
    if capture_output {
        // Capture output
        match command.output() {
            Ok(output) => {
                let returncode = output.status.code().unwrap_or(if output.status.success() { 0 } else { 1 });
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                Ok(CompletedProcess::with_output(args_owned, returncode, stdout, stderr))
            }
            Err(e) => {
                Err(crate::runtime_error(format!("Failed to execute command '{}': {}", str_args[0], e)))
            }
        }
    } else {
        // Just get the exit status
        match command.status() {
            Ok(status) => {
                let returncode = status.code().unwrap_or(if status.success() { 0 } else { 1 });
                Ok(CompletedProcess::new(args_owned, returncode))
            }
            Err(e) => {
                Err(crate::runtime_error(format!("Failed to execute command '{}': {}", str_args[0], e)))
            }
        }
    }
}

/// subprocess.call - run command and return exit code (generic version)
/// 
/// # Arguments
/// * `args` - Command and arguments to run (any collection of string-like types)
/// 
/// # Returns
/// The exit code of the process
pub fn call_generic<A, S>(args: A) -> Result<i32, PyException>
where
    A: IntoIterator<Item = S>,
    S: AsStrLike,
{
    let result = run_mixed_args(args, None::<&str>)?;
    Ok(result.returncode)
}

/// subprocess.call - run command and return exit code
/// 
/// # Arguments
/// * `args` - Command and arguments to run
/// 
/// # Returns
/// The exit code of the process
pub fn call<A: AsRef<str>>(args: Vec<A>) -> Result<i32, PyException> {
    let result = run(args, None::<&str>)?;
    Ok(result.returncode)
}

/// subprocess.check_call - run command and check that it succeeds
/// 
/// # Arguments
/// * `args` - Command and arguments to run
/// 
/// # Returns
/// Nothing if successful, raises exception if process fails
pub fn check_call<A: AsRef<str> + Clone>(args: Vec<A>) -> Result<(), PyException> {
    let result = run(args.clone(), None::<&str>)?;
    if result.returncode != 0 {
        let cmd = args.iter().map(|a| a.as_ref()).collect::<Vec<&str>>().join(" ");
        return Err(crate::runtime_error(format!("Command '{}' failed with exit code {}", cmd, result.returncode)));
    }
    Ok(())
}

/// subprocess.check_output - run command and return output
/// 
/// # Arguments
/// * `args` - Command and arguments to run
/// 
/// # Returns
/// The stdout of the process as a string
pub fn check_output<A: AsRef<str> + Clone>(args: Vec<A>) -> Result<String, PyException> {
    let result = run_with_output(args.clone(), None::<&str>, true)?;
    if result.returncode != 0 {
        let cmd = args.iter().map(|a| a.as_ref()).collect::<Vec<&str>>().join(" ");
        return Err(crate::runtime_error(format!("Command '{}' failed with exit code {}", cmd, result.returncode)));
    }
    Ok(result.stdout.unwrap_or_default())
}

// Compatibility aliases for generated code
// Note: Functions are already public in this module, no need to re-export
