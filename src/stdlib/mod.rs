//! Python Standard Library Implementation
//! 
//! This module contains implementations of Python's standard library modules
//! that are commonly used in Python programs. Each submodule provides
//! functionality equivalent to the corresponding Python module.

/// Python sys module - system-specific parameters and functions
#[cfg(feature = "std")]
pub mod sys;

/// Python os module - operating system interface
#[cfg(feature = "std")]
pub mod os;

/// Python subprocess module - subprocess management
#[cfg(feature = "std")]
pub mod subprocess;