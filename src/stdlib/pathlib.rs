//! Python pathlib module implementation
//! 
//! This module provides object-oriented filesystem paths.
//! Implementation matches Python's pathlib module API.

use crate::PyException;
use std::path::{Path as StdPath, PathBuf as StdPathBuf};

/// Pure path - platform-independent path operations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PurePath {
    path: StdPathBuf,
}

impl PurePath {
    /// Create new PurePath
    pub fn new<P: AsRef<StdPath>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
    
    /// Get path parts
    pub fn parts(&self) -> Vec<String> {
        self.path.components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .collect()
    }
    
    /// Get drive (Windows only)
    pub fn drive(&self) -> String {
        #[cfg(windows)]
        {
            if let Some(prefix) = self.path.components().next() {
                if let std::path::Component::Prefix(prefix_component) = prefix {
                    return prefix_component.as_os_str().to_string_lossy().to_string();
                }
            }
        }
        String::new()
    }
    
    /// Get root
    pub fn root(&self) -> String {
        if self.path.is_absolute() {
            std::path::MAIN_SEPARATOR.to_string()
        } else {
            String::new()
        }
    }
    
    /// Get anchor (drive + root)
    pub fn anchor(&self) -> String {
        format!("{}{}", self.drive(), self.root())
    }
    
    /// Get parent directory
    pub fn parent(&self) -> PurePath {
        if let Some(parent) = self.path.parent() {
            PurePath::new(parent)
        } else {
            PurePath::new("")
        }
    }
    
    /// Get all parents
    pub fn parents(&self) -> Vec<PurePath> {
        let mut parents = Vec::new();
        let mut current = self.path.as_path();
        
        while let Some(parent) = current.parent() {
            parents.push(PurePath::new(parent));
            current = parent;
        }
        
        parents
    }
    
    /// Get file name
    pub fn name(&self) -> String {
        self.path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default()
    }
    
    /// Get file suffix
    pub fn suffix(&self) -> String {
        self.path.extension()
            .map(|ext| format!(".{}", ext.to_string_lossy()))
            .unwrap_or_default()
    }
    
    /// Get all suffixes
    pub fn suffixes(&self) -> Vec<String> {
        let name = self.name();
        if name.is_empty() {
            return Vec::new();
        }
        
        let mut suffixes = Vec::new();
        let parts: Vec<&str> = name.split('.').collect();
        
        if parts.len() > 1 {
            for i in 1..parts.len() {
                suffixes.push(format!(".{}", parts[i]));
            }
        }
        
        suffixes
    }
    
    /// Get stem (filename without final suffix)
    pub fn stem(&self) -> String {
        self.path.file_stem()
            .map(|stem| stem.to_string_lossy().to_string())
            .unwrap_or_default()
    }
    
    /// Join with other path
    pub fn joinpath<P: AsRef<StdPath>>(&self, other: P) -> PurePath {
        PurePath::new(self.path.join(other))
    }
    
    /// Check if path matches pattern
    pub fn match_pattern(&self, pattern: &str) -> bool {
        // Simple glob-like matching
        let name = self.name();
        match_glob(&name, pattern)
    }
    
    /// Get relative path to other
    pub fn relative_to(&self, other: &PurePath) -> Result<PurePath, PyException> {
        self.path.strip_prefix(&other.path)
            .map(|p| PurePath::new(p))
            .map_err(|_| crate::value_error("Path is not relative to the given path"))
    }
    
    /// Check if path is absolute
    pub fn is_absolute(&self) -> bool {
        self.path.is_absolute()
    }
    
    /// Check if path is relative
    pub fn is_relative(&self) -> bool {
        self.path.is_relative()
    }
    
    /// Convert to string
    pub fn as_posix(&self) -> String {
        self.path.to_string_lossy().replace('\\', "/")
    }
    
    /// Replace path components
    pub fn with_name<S: AsRef<str>>(&self, name: S) -> PurePath {
        PurePath::new(self.path.with_file_name(name.as_ref()))
    }
    
    /// Replace suffix
    pub fn with_suffix<S: AsRef<str>>(&self, suffix: S) -> PurePath {
        let suffix = suffix.as_ref();
        let stem = self.stem();
        if suffix.is_empty() {
            PurePath::new(self.path.with_file_name(stem))
        } else {
            let new_name = if suffix.starts_with('.') {
                format!("{}{}", stem, suffix)
            } else {
                format!("{}.{}", stem, suffix)
            };
            PurePath::new(self.path.with_file_name(new_name))
        }
    }
}

impl std::fmt::Display for PurePath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

impl From<&str> for PurePath {
    fn from(s: &str) -> Self {
        PurePath::new(s)
    }
}

impl From<String> for PurePath {
    fn from(s: String) -> Self {
        PurePath::new(s)
    }
}

/// Concrete path - includes filesystem operations
#[derive(Debug, Clone)]
pub struct Path {
    pure_path: PurePath,
}

impl Path {
    /// Create new Path
    pub fn new<P: AsRef<StdPath>>(path: P) -> Self {
        Self {
            pure_path: PurePath::new(path),
        }
    }
    
    /// Current working directory
    #[cfg(feature = "std")]
    pub fn cwd() -> Result<Path, PyException> {
        std::env::current_dir()
            .map(|p| Path::new(p))
            .map_err(|e| crate::runtime_error(format!("Failed to get current directory: {}", e)))
    }
    
    /// Home directory
    #[cfg(feature = "std")]
    pub fn home() -> Result<Path, PyException> {
        std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .map(|home_str| Path::new(home_str))
            .map_err(|_| crate::runtime_error("Failed to get home directory"))
    }
    
    /// Get absolute path
    #[cfg(feature = "std")]
    pub fn absolute(&self) -> Result<Path, PyException> {
        self.pure_path.path.canonicalize()
            .or_else(|_| {
                if self.pure_path.path.is_absolute() {
                    Ok(self.pure_path.path.clone())
                } else {
                    std::env::current_dir().map(|cwd| cwd.join(&self.pure_path.path))
                }
            })
            .map(|p| Path::new(p))
            .map_err(|e| crate::runtime_error(format!("Failed to get absolute path: {}", e)))
    }
    
    /// Resolve path (follow symlinks)
    #[cfg(feature = "std")]
    pub fn resolve(&self) -> Result<Path, PyException> {
        self.pure_path.path.canonicalize()
            .map(|p| Path::new(p))
            .map_err(|e| crate::runtime_error(format!("Failed to resolve path: {}", e)))
    }
    
    /// Check if path exists
    #[cfg(feature = "std")]
    pub fn exists(&self) -> bool {
        self.pure_path.path.exists()
    }
    
    /// Check if path is file
    #[cfg(feature = "std")]
    pub fn is_file(&self) -> bool {
        self.pure_path.path.is_file()
    }
    
    /// Check if path is directory
    #[cfg(feature = "std")]
    pub fn is_dir(&self) -> bool {
        self.pure_path.path.is_dir()
    }
    
    /// Check if path is symlink
    #[cfg(feature = "std")]
    pub fn is_symlink(&self) -> bool {
        self.pure_path.path.is_symlink()
    }
    
    /// Get file statistics
    #[cfg(feature = "std")]
    pub fn stat(&self) -> Result<FileStats, PyException> {
        self.pure_path.path.metadata()
            .map(|meta| FileStats::from_metadata(meta))
            .map_err(|e| crate::runtime_error(format!("Failed to get file stats: {}", e)))
    }
    
    /// Create directory
    #[cfg(feature = "std")]
    pub fn mkdir(&self, parents: bool, exist_ok: bool) -> Result<(), PyException> {
        if parents {
            std::fs::create_dir_all(&self.pure_path.path)
        } else {
            std::fs::create_dir(&self.pure_path.path)
        }.or_else(|e| {
            if exist_ok && e.kind() == std::io::ErrorKind::AlreadyExists && self.is_dir() {
                Ok(())
            } else {
                Err(e)
            }
        })
        .map_err(|e| crate::runtime_error(format!("Failed to create directory: {}", e)))
    }
    
    /// Remove directory
    #[cfg(feature = "std")]
    pub fn rmdir(&self) -> Result<(), PyException> {
        std::fs::remove_dir(&self.pure_path.path)
            .map_err(|e| crate::runtime_error(format!("Failed to remove directory: {}", e)))
    }
    
    /// Remove file
    #[cfg(feature = "std")]
    pub fn unlink(&self, missing_ok: bool) -> Result<(), PyException> {
        std::fs::remove_file(&self.pure_path.path)
            .or_else(|e| {
                if missing_ok && e.kind() == std::io::ErrorKind::NotFound {
                    Ok(())
                } else {
                    Err(e)
                }
            })
            .map_err(|e| crate::runtime_error(format!("Failed to remove file: {}", e)))
    }
    
    /// List directory contents
    #[cfg(feature = "std")]
    pub fn iterdir(&self) -> Result<Vec<Path>, PyException> {
        std::fs::read_dir(&self.pure_path.path)
            .map_err(|e| crate::runtime_error(format!("Failed to read directory: {}", e)))
            .map(|entries| {
                entries.filter_map(|entry| {
                    entry.ok().map(|e| Path::new(e.path()))
                }).collect()
            })
    }
    
    /// Glob pattern matching
    #[cfg(feature = "std")]
    pub fn glob(&self, pattern: &str) -> Result<Vec<Path>, PyException> {
        // Simple implementation - in a real implementation you'd want to use a proper glob library
        let entries = self.iterdir()?;
        let mut matches = Vec::new();
        
        for entry in entries {
            if entry.pure_path.match_pattern(pattern) {
                matches.push(entry);
            }
        }
        
        Ok(matches)
    }
    
    /// Recursive glob
    #[cfg(feature = "std")]
    pub fn rglob(&self, pattern: &str) -> Result<Vec<Path>, PyException> {
        let mut matches = Vec::new();
        self.rglob_recursive(pattern, &mut matches)?;
        Ok(matches)
    }
    
    #[cfg(feature = "std")]
    fn rglob_recursive(&self, pattern: &str, matches: &mut Vec<Path>) -> Result<(), PyException> {
        if let Ok(entries) = self.iterdir() {
            for entry in entries {
                if entry.pure_path.match_pattern(pattern) {
                    matches.push(entry.clone());
                }
                if entry.is_dir() {
                    entry.rglob_recursive(pattern, matches)?;
                }
            }
        }
        Ok(())
    }
    
    /// Read text file
    #[cfg(feature = "std")]
    pub fn read_text(&self, _encoding: Option<&str>) -> Result<String, PyException> {
        std::fs::read_to_string(&self.pure_path.path)
            .map_err(|e| crate::runtime_error(format!("Failed to read text file: {}", e)))
    }
    
    /// Write text file
    #[cfg(feature = "std")]
    pub fn write_text<S: AsRef<str>>(&self, data: S) -> Result<(), PyException> {
        std::fs::write(&self.pure_path.path, data.as_ref())
            .map_err(|e| crate::runtime_error(format!("Failed to write text file: {}", e)))
    }
    
    /// Read bytes
    #[cfg(feature = "std")]
    pub fn read_bytes(&self) -> Result<Vec<u8>, PyException> {
        std::fs::read(&self.pure_path.path)
            .map_err(|e| crate::runtime_error(format!("Failed to read bytes: {}", e)))
    }
    
    /// Write bytes
    #[cfg(feature = "std")]
    pub fn write_bytes(&self, data: &[u8]) -> Result<(), PyException> {
        std::fs::write(&self.pure_path.path, data)
            .map_err(|e| crate::runtime_error(format!("Failed to write bytes: {}", e)))
    }
}

// Delegate PurePath methods to Path
impl std::ops::Deref for Path {
    type Target = PurePath;
    
    fn deref(&self) -> &Self::Target {
        &self.pure_path
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.pure_path)
    }
}

impl From<&str> for Path {
    fn from(s: &str) -> Self {
        Path::new(s)
    }
}

impl From<String> for Path {
    fn from(s: String) -> Self {
        Path::new(s)
    }
}

/// File statistics
#[derive(Debug, Clone)]
pub struct FileStats {
    pub size: u64,
    pub is_dir: bool,
    pub is_file: bool,
    pub is_symlink: bool,
    #[cfg(feature = "std")]
    pub modified: Option<std::time::SystemTime>,
    #[cfg(feature = "std")]
    pub accessed: Option<std::time::SystemTime>,
    #[cfg(feature = "std")]
    pub created: Option<std::time::SystemTime>,
}

#[cfg(feature = "std")]
impl FileStats {
    fn from_metadata(metadata: std::fs::Metadata) -> Self {
        Self {
            size: metadata.len(),
            is_dir: metadata.is_dir(),
            is_file: metadata.is_file(),
            is_symlink: metadata.is_symlink(),
            modified: metadata.modified().ok(),
            accessed: metadata.accessed().ok(),
            created: metadata.created().ok(),
        }
    }
}

// Module-level functions

/// Create PurePath
pub fn pure_path<P: AsRef<StdPath>>(path: P) -> PurePath {
    PurePath::new(path)
}

/// Create Path
pub fn path<P: AsRef<StdPath>>(p: P) -> Path {
    Path::new(p)
}

// Helper functions

fn match_glob(text: &str, pattern: &str) -> bool {
    // Simple glob matching - supports * and ?
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let text_chars: Vec<char> = text.chars().collect();
    
    match_glob_recursive(&text_chars, &pattern_chars, 0, 0)
}

fn match_glob_recursive(text: &[char], pattern: &[char], text_idx: usize, pattern_idx: usize) -> bool {
    // Base cases
    if pattern_idx >= pattern.len() {
        return text_idx >= text.len();
    }
    
    if text_idx >= text.len() {
        // Check if remaining pattern is only '*'
        return pattern[pattern_idx..].iter().all(|&c| c == '*');
    }
    
    match pattern[pattern_idx] {
        '*' => {
            // Try matching zero or more characters
            match_glob_recursive(text, pattern, text_idx, pattern_idx + 1) ||
            match_glob_recursive(text, pattern, text_idx + 1, pattern_idx)
        }
        '?' => {
            // Match exactly one character
            match_glob_recursive(text, pattern, text_idx + 1, pattern_idx + 1)
        }
        c => {
            // Match exact character
            if text[text_idx] == c {
                match_glob_recursive(text, pattern, text_idx + 1, pattern_idx + 1)
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_purepath_parts() {
        let p = PurePath::new("/home/user/file.txt");
        let parts = p.parts();
        assert!(parts.contains(&"home".to_string()));
        assert!(parts.contains(&"user".to_string()));
        assert!(parts.contains(&"file.txt".to_string()));
    }
    
    #[test]
    fn test_purepath_suffix() {
        let p = PurePath::new("file.tar.gz");
        assert_eq!(p.suffix(), ".gz");
        assert_eq!(p.suffixes(), vec![".tar", ".gz"]);
        assert_eq!(p.stem(), "file.tar");
    }
    
    #[test]
    fn test_purepath_joinpath() {
        let p1 = PurePath::new("/home/user");
        let p2 = p1.joinpath("documents");
        assert_eq!(p2.name(), "documents");
    }
    
    #[test]
    fn test_glob_matching() {
        assert!(match_glob("file.txt", "*.txt"));
        assert!(match_glob("test.py", "test.*"));
        assert!(match_glob("hello", "h?llo"));
        assert!(!match_glob("file.py", "*.txt"));
    }
}