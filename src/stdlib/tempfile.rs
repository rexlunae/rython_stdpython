//! Python tempfile module implementation
//! 
//! This module provides facilities for creating temporary files and directories.
//! Implementation matches Python's tempfile module API.

use crate::PyException;
use std::path::PathBuf;

/// Get temporary directory
#[cfg(feature = "std")]
pub fn gettempdir() -> PathBuf {
    std::env::temp_dir()
}

/// Get temporary directory as string
#[cfg(feature = "std")]
pub fn gettempdir_str() -> String {
    gettempdir().to_string_lossy().to_string()
}

/// Generate temporary filename
#[cfg(feature = "std")]
pub fn mktemp(suffix: Option<&str>, prefix: Option<&str>, dir: Option<&str>) -> String {
    let mut path = if let Some(dir) = dir {
        PathBuf::from(dir)
    } else {
        gettempdir()
    };
    
    let prefix = prefix.unwrap_or("tmp");
    let suffix = suffix.unwrap_or("");
    
    // Generate random component
    let random_part = generate_random_string(8);
    let filename = format!("{}{}{}", prefix, random_part, suffix);
    
    path.push(filename);
    path.to_string_lossy().to_string()
}

/// Create and open temporary file
#[cfg(feature = "std")]
pub fn mkstemp(suffix: Option<&str>, prefix: Option<&str>, dir: Option<&str>, _text: bool) -> Result<(i32, String), PyException> {
    use std::fs::OpenOptions;
    #[cfg(unix)]
    use std::os::unix::io::AsRawFd;
    #[cfg(unix)]
    use std::os::unix::fs::OpenOptionsExt;
    
    let mut attempts = 0;
    let max_attempts = 1000;
    
    while attempts < max_attempts {
        let filename = mktemp(suffix, prefix, dir);
        
        let mut open_options = OpenOptions::new();
        open_options.read(true).write(true).create_new(true);
        
        #[cfg(unix)]
        open_options.mode(0o600);
        
        match open_options.open(&filename) {
            Ok(file) => {
                #[cfg(unix)]
                let fd = file.as_raw_fd();
                #[cfg(windows)]
                let fd = 0; // Placeholder for Windows - would need proper implementation
                #[cfg(not(any(unix, windows)))]
                let fd = -1; // Fallback for other platforms
                
                std::mem::forget(file); // Don't close the file
                return Ok((fd, filename));
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                attempts += 1;
                continue;
            }
            Err(e) => {
                return Err(crate::runtime_error(format!("Failed to create temporary file: {}", e)));
            }
        }
    }
    
    Err(crate::runtime_error("Failed to create temporary file after maximum attempts"))
}

/// Create temporary directory
#[cfg(feature = "std")]
pub fn mkdtemp(suffix: Option<&str>, prefix: Option<&str>, dir: Option<&str>) -> Result<String, PyException> {
    let mut attempts = 0;
    let max_attempts = 1000;
    
    while attempts < max_attempts {
        let mut path = if let Some(dir) = dir {
            PathBuf::from(dir)
        } else {
            gettempdir()
        };
        
        let prefix = prefix.unwrap_or("tmp");
        let suffix = suffix.unwrap_or("");
        let random_part = generate_random_string(8);
        let dirname = format!("{}{}{}", prefix, random_part, suffix);
        
        path.push(dirname);
        
        match std::fs::create_dir(&path) {
            Ok(_) => return Ok(path.to_string_lossy().to_string()),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                attempts += 1;
                continue;
            }
            Err(e) => {
                return Err(crate::runtime_error(format!("Failed to create temporary directory: {}", e)));
            }
        }
    }
    
    Err(crate::runtime_error("Failed to create temporary directory after maximum attempts"))
}

/// Temporary file context manager
#[cfg(feature = "std")]
#[derive(Debug)]
pub struct NamedTemporaryFile {
    file: Option<std::fs::File>,
    path: PathBuf,
    delete_on_drop: bool,
}

#[cfg(feature = "std")]
impl NamedTemporaryFile {
    /// Create new temporary file
    pub fn new(
        _mode: Option<&str>,
        _buffering: Option<i32>,
        encoding: Option<&str>,
        _newline: Option<&str>,
        suffix: Option<&str>,
        prefix: Option<&str>,
        dir: Option<&str>,
        delete: bool,
    ) -> Result<Self, PyException> {
        let (_, path_str) = mkstemp(suffix, prefix, dir, encoding.is_some())?;
        let path = PathBuf::from(&path_str);
        
        let file = std::fs::File::options()
            .read(true)
            .write(true)
            .open(&path)
            .map_err(|e| crate::runtime_error(format!("Failed to open temporary file: {}", e)))?;
            
        Ok(Self {
            file: Some(file),
            path,
            delete_on_drop: delete,
        })
    }
    
    /// Get file name
    pub fn name(&self) -> String {
        self.path.to_string_lossy().to_string()
    }
    
    /// Read from file
    pub fn read(&mut self, size: Option<usize>) -> Result<Vec<u8>, PyException> {
        use std::io::Read;
        
        if let Some(ref mut file) = self.file {
            let mut buffer = Vec::new();
            if let Some(size) = size {
                buffer.resize(size, 0);
                let bytes_read = file.read(&mut buffer)
                    .map_err(|e| crate::runtime_error(format!("Failed to read from temporary file: {}", e)))?;
                buffer.truncate(bytes_read);
            } else {
                file.read_to_end(&mut buffer)
                    .map_err(|e| crate::runtime_error(format!("Failed to read from temporary file: {}", e)))?;
            }
            Ok(buffer)
        } else {
            Err(crate::value_error("File is closed"))
        }
    }
    
    /// Write to file
    pub fn write(&mut self, data: &[u8]) -> Result<usize, PyException> {
        use std::io::Write;
        
        if let Some(ref mut file) = self.file {
            file.write(data)
                .map_err(|e| crate::runtime_error(format!("Failed to write to temporary file: {}", e)))
        } else {
            Err(crate::value_error("File is closed"))
        }
    }
    
    /// Flush file
    pub fn flush(&mut self) -> Result<(), PyException> {
        use std::io::Write;
        
        if let Some(ref mut file) = self.file {
            file.flush()
                .map_err(|e| crate::runtime_error(format!("Failed to flush temporary file: {}", e)))
        } else {
            Err(crate::value_error("File is closed"))
        }
    }
    
    /// Close file
    pub fn close(&mut self) -> Result<(), PyException> {
        if self.file.is_some() {
            self.file = None;
            Ok(())
        } else {
            Err(crate::value_error("File already closed"))
        }
    }
    
    /// Seek in file
    pub fn seek(&mut self, offset: i64, whence: i32) -> Result<u64, PyException> {
        use std::io::{Seek, SeekFrom};
        
        if let Some(ref mut file) = self.file {
            let seek_from = match whence {
                0 => SeekFrom::Start(offset as u64),
                1 => SeekFrom::Current(offset),
                2 => SeekFrom::End(offset),
                _ => return Err(crate::value_error("Invalid whence value")),
            };
            
            file.seek(seek_from)
                .map_err(|e| crate::runtime_error(format!("Failed to seek in temporary file: {}", e)))
        } else {
            Err(crate::value_error("File is closed"))
        }
    }
}

#[cfg(feature = "std")]
impl Drop for NamedTemporaryFile {
    fn drop(&mut self) {
        if self.delete_on_drop {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

/// Temporary directory context manager
#[cfg(feature = "std")]
#[derive(Debug)]
pub struct TemporaryDirectory {
    path: Option<PathBuf>,
}

#[cfg(feature = "std")]
impl TemporaryDirectory {
    /// Create new temporary directory
    pub fn new(
        suffix: Option<&str>,
        prefix: Option<&str>,
        dir: Option<&str>,
    ) -> Result<Self, PyException> {
        let path_str = mkdtemp(suffix, prefix, dir)?;
        Ok(Self {
            path: Some(PathBuf::from(path_str)),
        })
    }
    
    /// Get directory name
    pub fn name(&self) -> Option<String> {
        self.path.as_ref().map(|p| p.to_string_lossy().to_string())
    }
    
    /// Cleanup directory
    pub fn cleanup(&mut self) -> Result<(), PyException> {
        if let Some(path) = self.path.take() {
            std::fs::remove_dir_all(&path)
                .map_err(|e| crate::runtime_error(format!("Failed to remove temporary directory: {}", e)))?;
        }
        Ok(())
    }
}

#[cfg(feature = "std")]
impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

/// SpooledTemporaryFile - temporary file that starts in memory
#[cfg(feature = "std")]
#[derive(Debug)]
pub struct SpooledTemporaryFile {
    data: Vec<u8>,
    position: usize,
    max_size: usize,
    file: Option<NamedTemporaryFile>,
}

#[cfg(feature = "std")]
impl SpooledTemporaryFile {
    /// Create new spooled temporary file
    pub fn new(
        max_size: Option<usize>,
        _mode: Option<&str>,
        _buffering: Option<i32>,
        _encoding: Option<&str>,
        _newline: Option<&str>,
        _suffix: Option<&str>,
        _prefix: Option<&str>,
        _dir: Option<&str>,
    ) -> Self {
        Self {
            data: Vec::new(),
            position: 0,
            max_size: max_size.unwrap_or(5000),
            file: None,
        }
    }
    
    /// Write data
    pub fn write(&mut self, data: &[u8]) -> Result<usize, PyException> {
        if self.file.is_some() {
            return self.file.as_mut().unwrap().write(data);
        }
        
        // Check if we need to roll over to file
        if self.data.len() + data.len() > self.max_size {
            self.rollover()?;
            return self.file.as_mut().unwrap().write(data);
        }
        
        self.data.extend_from_slice(data);
        Ok(data.len())
    }
    
    /// Read data
    pub fn read(&mut self, size: Option<usize>) -> Result<Vec<u8>, PyException> {
        if let Some(ref mut file) = self.file {
            return file.read(size);
        }
        
        let available = self.data.len().saturating_sub(self.position);
        let to_read = size.map(|s| s.min(available)).unwrap_or(available);
        
        if to_read == 0 {
            return Ok(Vec::new());
        }
        
        let result = self.data[self.position..self.position + to_read].to_vec();
        self.position += to_read;
        Ok(result)
    }
    
    /// Seek in file
    pub fn seek(&mut self, offset: i64, whence: i32) -> Result<u64, PyException> {
        if let Some(ref mut file) = self.file {
            return file.seek(offset, whence);
        }
        
        let new_position = match whence {
            0 => offset as usize,
            1 => (self.position as i64 + offset) as usize,
            2 => (self.data.len() as i64 + offset) as usize,
            _ => return Err(crate::value_error("Invalid whence value")),
        };
        
        self.position = new_position.min(self.data.len());
        Ok(self.position as u64)
    }
    
    /// Roll over to file
    fn rollover(&mut self) -> Result<(), PyException> {
        if self.file.is_some() {
            return Ok(());
        }
        
        let mut temp_file = NamedTemporaryFile::new(
            None, None, None, None, None, None, None, true
        )?;
        
        temp_file.write(&self.data)?;
        temp_file.seek(self.position as i64, 0)?;
        
        self.file = Some(temp_file);
        self.data.clear();
        self.position = 0;
        
        Ok(())
    }
}

// Helper functions

fn generate_random_string(length: usize) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    // Simple pseudo-random string generator
    let mut hasher = DefaultHasher::new();
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
    std::process::id().hash(&mut hasher);
    
    let seed = hasher.finish();
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let char_bytes = chars.as_bytes();
    
    (0..length)
        .map(|i| {
            let idx = ((seed >> (i % 8)) as usize) % char_bytes.len();
            char_bytes[idx] as char
        })
        .collect()
}

// Module constants
pub const TMP_MAX: usize = 10000;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[cfg(feature = "std")]
    #[test]
    fn test_gettempdir() {
        let temp_dir = gettempdir();
        assert!(temp_dir.exists());
    }
    
    #[cfg(feature = "std")]
    #[test]
    fn test_mktemp() {
        let temp_file = mktemp(Some(".txt"), Some("test_"), None);
        assert!(temp_file.contains("test_"));
        assert!(temp_file.ends_with(".txt"));
    }
    
    #[cfg(feature = "std")]
    #[test]
    fn test_mkdtemp() {
        let temp_dir = mkdtemp(None, Some("test_"), None).unwrap();
        assert!(std::path::Path::new(&temp_dir).exists());
        std::fs::remove_dir(&temp_dir).unwrap();
    }
    
    #[test]
    fn test_generate_random_string() {
        let s1 = generate_random_string(10);
        let s2 = generate_random_string(10);
        assert_eq!(s1.len(), 10);
        assert_eq!(s2.len(), 10);
        // They should be different (extremely unlikely to be same)
        assert_ne!(s1, s2);
    }
}