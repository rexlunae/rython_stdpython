//! Standard Python Runtime Library for Rython
//! 
//! This library provides all the built-in functions, types, and methods
//! that are available in Python without any imports. It serves as the
//! runtime foundation for Python code compiled to Rust using python-ast-rs.
//!
//! ## Features
//!
//! - `std` (default): Full standard library support with I/O operations
//! - `nostd`: No-std compatible version for embedded systems

#![cfg_attr(not(feature = "std"), no_std)]
// Allow non-conventional naming for Python API compatibility
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// Import conditional on std feature
#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};
#[cfg(feature = "std")]
use std::fmt::{Display, Debug};
#[cfg(feature = "std")]
use std::hash::Hash;

// Import conditional on nostd - using alternative crates
#[cfg(not(feature = "std"))]
use hashbrown::{HashMap, HashSet};
#[cfg(not(feature = "std"))]
use core::fmt::{Display, Debug};
#[cfg(not(feature = "std"))]
use core::hash::Hash;
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, string::String, format};
#[cfg(not(feature = "std"))]
use alloc::string::ToString;


// PyO3 only available with std
#[cfg(feature = "std")]
pub use pyo3::{PyAny, PyObject};

// ============================================================================
// GENERIC TRAITS FOR PYTHON OPERATIONS
// ============================================================================

/// Trait for types that can be used as string-like parameters
/// 
/// This allows functions to accept both &str and String seamlessly
pub trait AsStrLike {
    fn as_str_like(&self) -> &str;
}

impl AsStrLike for str {
    fn as_str_like(&self) -> &str {
        self
    }
}

impl AsStrLike for String {
    fn as_str_like(&self) -> &str {
        self.as_str()
    }
}

impl AsStrLike for &str {
    fn as_str_like(&self) -> &str {
        self
    }
}

impl AsStrLike for &String {
    fn as_str_like(&self) -> &str {
        self.as_str()
    }
}

/// Trait for types that can be converted to owned strings
/// 
/// This is useful for return values that need to be owned
pub trait IntoOwnedString {
    fn into_owned_string(self) -> String;
}

impl IntoOwnedString for &str {
    fn into_owned_string(self) -> String {
        self.to_string()
    }
}

impl IntoOwnedString for String {
    fn into_owned_string(self) -> String {
        self
    }
}

/// Trait for types that can be used as path-like parameters
/// 
/// This allows path functions to work with various string types
pub trait AsPathLike {
    fn as_path_like(&self) -> &str;
}

impl<T: AsStrLike> AsPathLike for T {
    fn as_path_like(&self) -> &str {
        self.as_str_like()
    }
}

/// Trait for collections that can be used as argument lists
/// 
/// This allows subprocess functions to accept various collection types
pub trait AsArgList<T> {
    fn as_arg_list(&self) -> Vec<&str>;
}

impl<T> AsArgList<T> for Vec<T> 
where
    T: AsRef<str>,
{
    fn as_arg_list(&self) -> Vec<&str> {
        self.iter().map(|s| s.as_ref()).collect()
    }
}

impl<T> AsArgList<T> for &[T] 
where
    T: AsRef<str>,
{
    fn as_arg_list(&self) -> Vec<&str> {
        self.iter().map(|s| s.as_ref()).collect()
    }
}

/// Trait for environment-like collections (key-value pairs)
pub trait AsEnvLike<K, V> {
    fn as_env_like(&self) -> HashMap<&str, &str>;
}

impl<K, V> AsEnvLike<K, V> for HashMap<K, V>
where
    K: AsRef<str>,
    V: AsRef<str>,
{
    fn as_env_like(&self) -> HashMap<&str, &str> {
        self.iter()
            .map(|(k, v)| (k.as_ref(), v.as_ref()))
            .collect()
    }
}

// ============================================================================
// PYTHON BUILT-IN FUNCTIONS
// ============================================================================

/// Python print() function - outputs objects to stdout with optional separator and ending
/// 
/// # Arguments
/// * `objects` - Values to print (anything implementing Display)
/// * `sep` - String inserted between values (default: " ")
/// * `end` - String appended after the last value (default: "\n")
/// * `flush` - Whether to forcibly flush the stream (default: false)
/// 
/// Note: Only available with `std` feature - requires OS I/O capabilities
#[cfg(feature = "std")]
pub fn print<T: Display>(object: T) {
    println!("{}", object);
}

/// Python print() function with multiple arguments
/// Note: Only available with `std` feature - requires OS I/O capabilities
#[cfg(feature = "std")]
pub fn print_args<T: Display, S: AsRef<str>, E: AsRef<str>>(objects: &[T], sep: S, end: E) {
    let output = objects.iter()
        .map(|obj| format!("{}", obj))
        .collect::<Vec<_>>()
        .join(sep.as_ref());
    print!("{}{}", output, end.as_ref());
}

/// No-std version of print - stores output in a string instead of printing
/// 
/// This version is available in nostd environments but doesn't perform actual I/O
#[cfg(not(feature = "std"))]
pub fn print_to_string<T: Display>(object: T) -> String {
    format!("{}", object)
}

/// No-std version of print with multiple arguments
#[cfg(not(feature = "std"))]
pub fn print_args_to_string<T: Display, S: AsRef<str>, E: AsRef<str>>(objects: &[T], sep: S, end: E) -> String {
    let output = objects.iter()
        .map(|obj| format!("{}", obj))
        .collect::<Vec<_>>()
        .join(sep.as_ref());
    format!("{}{}", output, end.as_ref())
}

/// Python len() function - returns the length of an object
pub fn len<T>(obj: &T) -> usize 
where 
    T: Len,
{
    obj.len()
}

/// Python dict() function - creates a new dictionary (generic version)
/// 
/// # Arguments
/// * `pairs` - Key-value pairs to initialize the dictionary with
/// 
/// # Returns
/// A new HashMap containing the provided key-value pairs
pub fn dict<K, V>(pairs: HashMap<K, V>) -> HashMap<K, V> 
where
    K: Hash + Eq,
{
    pairs
}

/// Python dict() function with environment merging (generic version)
/// 
/// This merges environment-like collections with additional key-value pairs
pub fn dict_with_env<E, K, V>(env: E, additional: HashMap<K, V>) -> HashMap<K, V>
where
    E: AsEnvLike<K, V>,
    K: Hash + Eq + for<'a> From<&'a str>,
    V: for<'a> From<&'a str>,
{
    let env_map = env.as_env_like();
    let mut result: HashMap<K, V> = env_map.into_iter()
        .map(|(k, v)| (K::from(k), V::from(v)))
        .collect();
    result.extend(additional);
    result
}

/// Simplified dict creation from key-value pairs
pub fn dict_from_pairs<K, V, I>(pairs: I) -> HashMap<K, V>
where
    K: Hash + Eq,
    I: IntoIterator<Item = (K, V)>,
{
    pairs.into_iter().collect()
}

// ============================================================================
// PYTHON NUMERIC OPERATIONS
// ============================================================================

/// Trait for types that support absolute value
pub trait PyAbs {
    type Output;
    fn py_abs(self) -> Self::Output;
}

/// Trait for types that can be summed
pub trait PySum<T> {
    fn py_sum(self) -> T;
}

/// Python abs() function - returns absolute value
pub fn abs<T: PyAbs>(x: T) -> T::Output {
    x.py_abs()
}

/// Python min() function
pub fn min<T: Ord + Clone>(iterable: &[T]) -> Option<T> {
    iterable.iter().min().cloned()
}

/// Python max() function
pub fn max<T: Ord + Clone>(iterable: &[T]) -> Option<T> {
    iterable.iter().max().cloned()
}

/// Python sum() function
pub fn sum<I, T>(iterable: I) -> T
where
    I: PySum<T>,
{
    iterable.py_sum()
}

// Implementations for PyAbs trait
impl PyAbs for i64 {
    type Output = i64;
    fn py_abs(self) -> Self::Output {
        self.abs()
    }
}

impl PyAbs for i32 {
    type Output = i32;
    fn py_abs(self) -> Self::Output {
        self.abs()
    }
}

impl PyAbs for f64 {
    type Output = f64;
    fn py_abs(self) -> Self::Output {
        self.abs()
    }
}

impl PyAbs for f32 {
    type Output = f32;
    fn py_abs(self) -> Self::Output {
        self.abs()
    }
}

// Implementations for PySum trait
impl PySum<i64> for &[i64] {
    fn py_sum(self) -> i64 {
        self.iter().sum()
    }
}

impl PySum<i32> for &[i32] {
    fn py_sum(self) -> i32 {
        self.iter().sum()
    }
}

impl PySum<f64> for &[f64] {
    fn py_sum(self) -> f64 {
        self.iter().sum()
    }
}

impl PySum<f32> for &[f32] {
    fn py_sum(self) -> f32 {
        self.iter().sum()
    }
}

impl<T> PySum<T> for &PyList<T>
where
    T: core::iter::Sum<T> + Clone,
{
    fn py_sum(self) -> T {
        self.inner.iter().cloned().sum()
    }
}

/// Python all() function - returns True if all elements are truthy
pub fn all<T: Truthy>(iterable: &[T]) -> bool {
    iterable.iter().all(|x| x.is_truthy())
}

/// Python any() function - returns True if any element is truthy
pub fn any<T: Truthy>(iterable: &[T]) -> bool {
    iterable.iter().any(|x| x.is_truthy())
}

/// Python enumerate() function - returns iterator with index and value pairs
pub fn enumerate<T>(iterable: Vec<T>) -> Vec<(usize, T)> {
    iterable.into_iter().enumerate().collect()
}

/// Python zip() function - combines multiple iterables
pub fn zip<T, U>(iter1: Vec<T>, iter2: Vec<U>) -> Vec<(T, U)> {
    iter1.into_iter().zip(iter2.into_iter()).collect()
}

/// Python range() function - generates sequence of numbers
pub fn range(stop: i64) -> Vec<i64> {
    (0..stop).collect()
}

pub fn range_start_stop(start: i64, stop: i64) -> Vec<i64> {
    (start..stop).collect()
}

pub fn range_start_stop_step(start: i64, stop: i64, step: i64) -> Vec<i64> {
    if step == 0 {
        panic!("range() step argument must not be zero");
    }
    let mut result = Vec::new();
    let mut current = start;
    
    if step > 0 {
        while current < stop {
            result.push(current);
            current += step;
        }
    } else {
        while current > stop {
            result.push(current);
            current += step;
        }
    }
    result
}

// ============================================================================
// PYTHON TYPE CONVERSION TRAITS
// ============================================================================

/// Trait for Python-style boolean conversion
pub trait PyBool {
    fn py_bool(self) -> bool;
}

/// Trait for Python-style integer conversion
pub trait PyInt {
    fn py_int(self) -> Result<i64, PyException>;
}

/// Trait for Python-style float conversion
pub trait PyFloat {
    fn py_float(self) -> Result<f64, PyException>;
}

/// Trait for Python-style string conversion
pub trait PyToString {
    fn py_str(self) -> String;
}

/// Python bool() function - converts to boolean
pub fn bool<T: PyBool>(x: T) -> bool {
    x.py_bool()
}

/// Python int() function - converts to integer
pub fn int<T: PyInt>(x: T) -> Result<i64, PyException> {
    x.py_int()
}

/// Python float() function - converts to float  
pub fn float<T: PyFloat>(x: T) -> Result<f64, PyException> {
    x.py_float()
}

/// Python str() function - converts to string
pub fn str<T: PyToString>(x: T) -> String {
    x.py_str()
}

// PyBool implementations
impl PyBool for i64 {
    fn py_bool(self) -> bool {
        self != 0
    }
}

impl PyBool for f64 {
    fn py_bool(self) -> bool {
        self != 0.0
    }
}

impl PyBool for &str {
    fn py_bool(self) -> bool {
        !self.is_empty()
    }
}

impl PyBool for String {
    fn py_bool(self) -> bool {
        !self.is_empty()
    }
}

impl PyBool for bool {
    fn py_bool(self) -> bool {
        self
    }
}

impl<T> PyBool for &PyList<T> {
    fn py_bool(self) -> bool {
        !self.inner.is_empty()
    }
}

impl<K, V> PyBool for &PyDictionary<K, V> 
where
    K: Eq + Hash,
{
    fn py_bool(self) -> bool {
        !self.inner.is_empty()
    }
}

impl PyBool for &PyStr {
    fn py_bool(self) -> bool {
        !self.inner.is_empty()
    }
}

// PyInt implementations
impl PyInt for &str {
    fn py_int(self) -> Result<i64, PyException> {
        self.parse().map_err(|_| value_error(&format!("invalid literal for int(): '{}'", self)))
    }
}

impl PyInt for String {
    fn py_int(self) -> Result<i64, PyException> {
        self.as_str().py_int()
    }
}

impl PyInt for f64 {
    fn py_int(self) -> Result<i64, PyException> {
        Ok(self as i64)
    }
}

impl PyInt for bool {
    fn py_int(self) -> Result<i64, PyException> {
        Ok(if self { 1 } else { 0 })
    }
}

impl PyInt for i64 {
    fn py_int(self) -> Result<i64, PyException> {
        Ok(self)
    }
}

// PyFloat implementations
impl PyFloat for &str {
    fn py_float(self) -> Result<f64, PyException> {
        self.parse().map_err(|_| value_error(&format!("could not convert string to float: '{}'", self)))
    }
}

impl PyFloat for String {
    fn py_float(self) -> Result<f64, PyException> {
        self.as_str().py_float()
    }
}

impl PyFloat for i64 {
    fn py_float(self) -> Result<f64, PyException> {
        Ok(self as f64)
    }
}

impl PyFloat for f64 {
    fn py_float(self) -> Result<f64, PyException> {
        Ok(self)
    }
}

// PyToString implementations
impl PyToString for i64 {
    fn py_str(self) -> String {
        self.to_string()
    }
}

impl PyToString for f64 {
    fn py_str(self) -> String {
        self.to_string()
    }
}

impl PyToString for bool {
    fn py_str(self) -> String {
        if self { "True".to_string() } else { "False".to_string() }
    }
}

impl PyToString for &str {
    fn py_str(self) -> String {
        self.to_string()
    }
}

impl PyToString for String {
    fn py_str(self) -> String {
        self
    }
}

// ============================================================================
// PYTHON BUILT-IN TYPES AND TRAITS
// ============================================================================

/// Trait for objects that have a length
pub trait Len {
    fn len(&self) -> usize;
}

/// Trait for objects that can be evaluated for truthiness
pub trait Truthy {
    fn is_truthy(&self) -> bool;
}

/// Python-style string type with all string methods
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyStr {
    inner: String,
}

impl PyStr {
    pub fn new(s: impl Into<String>) -> Self {
        Self { inner: s.into() }
    }
    
    /// Python str.split() method
    pub fn split(&self, sep: Option<&str>) -> Vec<PyStr> {
        match sep {
            Some(separator) => self.inner.split(separator)
                .map(|s| PyStr::new(s))
                .collect(),
            None => self.inner.split_whitespace()
                .map(|s| PyStr::new(s))
                .collect(),
        }
    }
    
    /// Python str.join() method
    pub fn join(&self, iterable: &[PyStr]) -> PyStr {
        let strings: Vec<&str> = iterable.iter().map(|s| s.inner.as_str()).collect();
        PyStr::new(strings.join(&self.inner))
    }
    
    /// Python str.strip() method
    pub fn strip(&self) -> PyStr {
        PyStr::new(self.inner.trim())
    }
    
    /// Python str.lower() method
    pub fn lower(&self) -> PyStr {
        PyStr::new(self.inner.to_lowercase())
    }
    
    /// Python str.upper() method
    pub fn upper(&self) -> PyStr {
        PyStr::new(self.inner.to_uppercase())
    }
    
    /// Python str.replace() method
    pub fn replace<O: AsRef<str>, N: AsRef<str>>(&self, old: O, new: N) -> PyStr {
        PyStr::new(self.inner.replace(old.as_ref(), new.as_ref()))
    }
    
    /// Python str.startswith() method
    pub fn startswith<P: AsRef<str>>(&self, prefix: P) -> bool {
        self.inner.starts_with(prefix.as_ref())
    }
    
    /// Python str.endswith() method
    pub fn endswith<S: AsRef<str>>(&self, suffix: S) -> bool {
        self.inner.ends_with(suffix.as_ref())
    }
    
    /// Python str.find() method
    pub fn find<S: AsRef<str>>(&self, sub: S) -> i64 {
        match self.inner.find(sub.as_ref()) {
            Some(pos) => pos as i64,
            None => -1,
        }
    }
    
    /// Python str.count() method
    pub fn count<S: AsRef<str>>(&self, sub: S) -> usize {
        self.inner.matches(sub.as_ref()).count()
    }
    
    /// Python str.format() method (basic implementation)
    pub fn format(&self, args: &[&str]) -> PyStr {
        let mut result = self.inner.clone();
        for (i, arg) in args.iter().enumerate() {
            result = result.replace(&format!("{{{}}}", i), arg);
        }
        PyStr::new(result)
    }
    
    /// Access inner string
    pub fn as_str(&self) -> &str {
        &self.inner
    }
}

impl Display for PyStr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Len for PyStr {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl Truthy for PyStr {
    fn is_truthy(&self) -> bool {
        !self.inner.is_empty()
    }
}

/// Python-style list type with all list methods
#[derive(Debug, Clone, PartialEq)]
pub struct PyList<T> {
    inner: Vec<T>,
}

impl<T> PyList<T> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }
    
    pub fn from_vec(vec: Vec<T>) -> Self {
        Self { inner: vec }
    }
    
    /// Python list.append() method
    pub fn append(&mut self, item: T) {
        self.inner.push(item);
    }
    
    /// Python list.extend() method
    pub fn extend(&mut self, items: Vec<T>) {
        self.inner.extend(items);
    }
    
    /// Python list.insert() method
    pub fn insert(&mut self, index: usize, item: T) {
        if index <= self.inner.len() {
            self.inner.insert(index, item);
        }
    }
    
    /// Python list.remove() method
    pub fn remove(&mut self, item: &T) -> bool 
    where 
        T: PartialEq,
    {
        if let Some(pos) = self.inner.iter().position(|x| x == item) {
            self.inner.remove(pos);
            true
        } else {
            false
        }
    }
    
    /// Python list.pop() method
    pub fn pop(&mut self, index: Option<usize>) -> Option<T> {
        match index {
            Some(i) if i < self.inner.len() => Some(self.inner.remove(i)),
            None if !self.inner.is_empty() => self.inner.pop(),
            _ => None,
        }
    }
    
    /// Python list.index() method
    pub fn index(&self, item: &T) -> Option<usize>
    where
        T: PartialEq,
    {
        self.inner.iter().position(|x| x == item)
    }
    
    /// Python list.count() method
    pub fn count(&self, item: &T) -> usize
    where
        T: PartialEq,
    {
        self.inner.iter().filter(|&x| x == item).count()
    }
    
    /// Python list.sort() method
    pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.inner.sort();
    }
    
    /// Python list.reverse() method
    pub fn reverse(&mut self) {
        self.inner.reverse();
    }
    
    /// Python list.clear() method
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    
    /// Python list.copy() method
    pub fn copy(&self) -> Self
    where
        T: Clone,
    {
        Self { inner: self.inner.clone() }
    }
    
    /// Get item by index
    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index)
    }
    
    /// Set item by index
    pub fn set(&mut self, index: usize, item: T) -> bool {
        if index < self.inner.len() {
            self.inner[index] = item;
            true
        } else {
            false
        }
    }
    
    /// Access inner vector
    pub fn as_vec(&self) -> &Vec<T> {
        &self.inner
    }
}

impl<T> Len for PyList<T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> Truthy for PyList<T> {
    fn is_truthy(&self) -> bool {
        !self.inner.is_empty()
    }
}

impl<T: Display> Display for PyList<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.inner.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

/// Python-style dictionary type with all dict methods
#[derive(Debug, Clone)]
pub struct PyDictionary<K, V>
where
    K: Eq + Hash,
{
    inner: HashMap<K, V>,
}

impl<K, V> PyDictionary<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self { inner: HashMap::new() }
    }
    
    /// Python dict.get() method
    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }
    
    /// Python dict.get() method with default
    pub fn get_or_default(&self, key: &K, default: V) -> V
    where
        V: Clone,
    {
        self.inner.get(key).cloned().unwrap_or(default)
    }
    
    /// Set key-value pair
    pub fn set(&mut self, key: K, value: V) {
        self.inner.insert(key, value);
    }
    
    /// Python dict.keys() method
    pub fn keys(&self) -> Vec<&K> {
        self.inner.keys().collect()
    }
    
    /// Python dict.values() method
    pub fn values(&self) -> Vec<&V> {
        self.inner.values().collect()
    }
    
    /// Python dict.items() method
    pub fn items(&self) -> Vec<(&K, &V)> {
        self.inner.iter().collect()
    }
    
    /// Python dict.update() method
    pub fn update(&mut self, other: PyDictionary<K, V>) {
        self.inner.extend(other.inner);
    }
    
    /// Python dict.pop() method
    pub fn pop(&mut self, key: &K) -> Option<V> {
        self.inner.remove(key)
    }
    
    /// Python dict.clear() method
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    
    /// Check if key exists
    pub fn contains_key(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }
}

impl<K, V> Len for PyDictionary<K, V> 
where 
    K: Eq + Hash,
{
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V> Truthy for PyDictionary<K, V> 
where 
    K: Eq + Hash,
{
    fn is_truthy(&self) -> bool {
        !self.inner.is_empty()
    }
}

/// Python-style tuple type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyTuple<T> {
    inner: Vec<T>,
}

impl<T> PyTuple<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { inner: items }
    }
    
    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index)
    }
    
    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }
}

impl<T> Len for PyTuple<T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> Truthy for PyTuple<T> {
    fn is_truthy(&self) -> bool {
        !self.inner.is_empty()
    }
}

impl<T: Display> Display for PyTuple<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(")?;
        for (i, item) in self.inner.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        if self.inner.len() == 1 {
            write!(f, ",")?;
        }
        write!(f, ")")
    }
}

/// Python-style set type with all set methods
#[derive(Debug, Clone)]
pub struct PySet<T>
where
    T: Eq + Hash,
{
    inner: HashSet<T>,
}

impl<T> PySet<T>
where
    T: Eq + Hash,
{
    pub fn new() -> Self {
        Self { inner: HashSet::new() }
    }
    
    /// Python set.add() method
    pub fn add(&mut self, item: T) {
        self.inner.insert(item);
    }
    
    /// Python set.remove() method
    pub fn remove(&mut self, item: &T) -> bool {
        self.inner.remove(item)
    }
    
    /// Python set.discard() method
    pub fn discard(&mut self, item: &T) {
        self.inner.remove(item);
    }
    
    /// Python set.union() method
    pub fn union(&self, other: &PySet<T>) -> PySet<T>
    where
        T: Clone,
    {
        let mut result = self.clone();
        result.inner.extend(other.inner.iter().cloned());
        result
    }
    
    /// Python set.intersection() method
    pub fn intersection(&self, other: &PySet<T>) -> PySet<T>
    where
        T: Clone,
    {
        PySet {
            inner: self.inner.intersection(&other.inner).cloned().collect(),
        }
    }
    
    /// Python set.difference() method
    pub fn difference(&self, other: &PySet<T>) -> PySet<T>
    where
        T: Clone,
    {
        PySet {
            inner: self.inner.difference(&other.inner).cloned().collect(),
        }
    }
    
    /// Check if item is in set
    pub fn contains(&self, item: &T) -> bool {
        self.inner.contains(item)
    }
    
    /// Python set.clear() method
    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl<T> Len for PySet<T> 
where 
    T: Eq + Hash,
{
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> Truthy for PySet<T> 
where 
    T: Eq + Hash,
{
    fn is_truthy(&self) -> bool {
        !self.inner.is_empty()
    }
}

// ============================================================================
// TRAIT IMPLEMENTATIONS FOR BUILT-IN TYPES
// ============================================================================

impl Truthy for bool {
    fn is_truthy(&self) -> bool {
        *self
    }
}

impl Truthy for i64 {
    fn is_truthy(&self) -> bool {
        *self != 0
    }
}

impl Truthy for f64 {
    fn is_truthy(&self) -> bool {
        *self != 0.0
    }
}

impl Len for String {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> Len for Vec<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

// ============================================================================
// PYTHON EXCEPTIONS
// ============================================================================

/// Base class for all Python exceptions
#[derive(Debug, Clone)]
pub struct PyException {
    pub message: String,
    pub exception_type: String,
}

impl PyException {
    pub fn new<T: AsRef<str>, M: AsRef<str>>(exception_type: T, message: M) -> Self {
        Self {
            message: message.as_ref().to_string(),
            exception_type: exception_type.as_ref().to_string(),
        }
    }
}

impl Display for PyException {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}: {}", self.exception_type, self.message)
    }
}

// Error trait only available with std
#[cfg(feature = "std")]
impl std::error::Error for PyException {}

/// Python ValueError
pub fn value_error<M: AsRef<str>>(message: M) -> PyException {
    PyException::new("ValueError", message.as_ref())
}

/// Python TypeError  
pub fn type_error<M: AsRef<str>>(message: M) -> PyException {
    PyException::new("TypeError", message.as_ref())
}

/// Python IndexError
pub fn index_error<M: AsRef<str>>(message: M) -> PyException {
    PyException::new("IndexError", message.as_ref())
}

/// Python KeyError
pub fn key_error<M: AsRef<str>>(message: M) -> PyException {
    PyException::new("KeyError", message.as_ref())
}

/// Python AttributeError
pub fn attribute_error<M: AsRef<str>>(message: M) -> PyException {
    PyException::new("AttributeError", message.as_ref())
}

/// Python NameError
pub fn name_error<M: AsRef<str>>(message: M) -> PyException {
    PyException::new("NameError", message.as_ref())
}

/// Python ZeroDivisionError
pub fn zero_division_error<M: AsRef<str>>(message: M) -> PyException {
    PyException::new("ZeroDivisionError", message.as_ref())
}

/// Python OverflowError
pub fn overflow_error<M: AsRef<str>>(message: M) -> PyException {
    PyException::new("OverflowError", message.as_ref())
}

/// Python RuntimeError
pub fn runtime_error<M: AsRef<str>>(message: M) -> PyException {
    PyException::new("RuntimeError", message.as_ref())
}

// ============================================================================
// PYTHON STANDARD LIBRARY MODULES
// ============================================================================

/// Python Standard Library modules
pub mod stdlib;

// Re-export stdlib modules at the top level for convenience
#[cfg(feature = "std")]
pub use stdlib::sys;
#[cfg(feature = "std")]
pub use stdlib::os; 
#[cfg(feature = "std")]
pub use stdlib::subprocess;
#[cfg(feature = "std")]
pub use stdlib::sysconfig;
#[cfg(feature = "std")]
pub use stdlib::venv;
pub use stdlib::math;
#[cfg(feature = "std")]
pub use stdlib::random;
#[cfg(feature = "std")]
pub use stdlib::datetime;
pub use stdlib::string;
pub use stdlib::json;
pub use stdlib::collections;
pub use stdlib::itertools;
#[cfg(feature = "std")]
pub use stdlib::pathlib;
#[cfg(feature = "std")]
pub use stdlib::tempfile;
#[cfg(feature = "std")]
pub use stdlib::glob;

/// Placeholder for ensure_venv_ready function (from pyperformance or similar)
/// This is not a standard Python built-in, so we provide a stub that returns dummy values
pub fn ensure_venv_ready<K: AsRef<str>>(kind: K) -> (String, String) {
    // In a real implementation, this would set up a virtual environment
    // For now, return placeholder values
    (format!("/tmp/venv_{}", kind.as_ref()), "python".to_string())
}

/// Python special variables
pub const __file__: &str = "script.py";
pub const __name__: &str = "__main__";

// ============================================================================
// OS-DEPENDENT FUNCTIONS (std feature only)
// ============================================================================

/// Python input() function - reads input from user
/// 
/// Note: Only available with `std` feature - requires OS I/O capabilities
#[cfg(feature = "std")]
pub fn input<P: AsRef<str>>(prompt: Option<P>) -> Result<String, PyException> {
    use std::io::{self, Write};
    
    if let Some(p) = prompt {
        print!("{}", p.as_ref());
        io::stdout().flush().map_err(|e| runtime_error(&format!("I/O error: {}", e)))?;
    }
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .map_err(|e| runtime_error(&format!("I/O error: {}", e)))?;
    
    // Remove trailing newline
    if input.ends_with('\n') {
        input.pop();
        if input.ends_with('\r') {
            input.pop();
        }
    }
    
    Ok(input)
}

/// Python open() function - opens a file
/// 
/// Note: Only available with `std` feature - requires OS I/O capabilities
#[cfg(feature = "std")]
pub fn open<F: AsRef<str>, M: AsRef<str>>(filename: F, mode: Option<M>) -> Result<PyFile, PyException> {
    use std::fs::{File, OpenOptions};
    use std::io::{BufReader, BufWriter};
    
    let mode = mode.as_ref().map(|m| m.as_ref()).unwrap_or("r");
    
    let file = match mode {
        "r" => {
            let f = File::open(filename.as_ref())
                .map_err(|e| runtime_error(format!("Could not open file '{}': {}", filename.as_ref(), e)))?;
            PyFile::new_read(BufReader::new(f))
        },
        "w" => {
            let f = File::create(filename.as_ref())
                .map_err(|e| runtime_error(format!("Could not create file '{}': {}", filename.as_ref(), e)))?;
            PyFile::new_write(BufWriter::new(f))
        },
        "a" => {
            let f = OpenOptions::new()
                .create(true)
                .append(true)
                .open(filename.as_ref())
                .map_err(|e| runtime_error(format!("Could not open file '{}' for append: {}", filename.as_ref(), e)))?;
            PyFile::new_write(BufWriter::new(f))
        },
        _ => return Err(value_error(&format!("Invalid file mode: '{}'", mode))),
    };
    
    Ok(file)
}

/// Python file object
/// 
/// Note: Only available with `std` feature - requires OS I/O capabilities
#[cfg(feature = "std")]
pub struct PyFile {
    reader: Option<std::io::BufReader<std::fs::File>>,
    writer: Option<std::io::BufWriter<std::fs::File>>,
}

#[cfg(feature = "std")]
impl PyFile {
    fn new_read(reader: std::io::BufReader<std::fs::File>) -> Self {
        Self {
            reader: Some(reader),
            writer: None,
        }
    }
    
    fn new_write(writer: std::io::BufWriter<std::fs::File>) -> Self {
        Self {
            reader: None,
            writer: Some(writer),
        }
    }
    
    /// Python file.read() method
    pub fn read(&mut self) -> Result<String, PyException> {
        use std::io::Read;
        
        if let Some(reader) = &mut self.reader {
            let mut contents = String::new();
            reader.read_to_string(&mut contents)
                .map_err(|e| runtime_error(&format!("Read error: {}", e)))?;
            Ok(contents)
        } else {
            Err(runtime_error("File not opened for reading"))
        }
    }
    
    /// Python file.readline() method
    pub fn readline(&mut self) -> Result<String, PyException> {
        use std::io::BufRead;
        
        if let Some(reader) = &mut self.reader {
            let mut line = String::new();
            reader.read_line(&mut line)
                .map_err(|e| runtime_error(&format!("Read error: {}", e)))?;
            Ok(line)
        } else {
            Err(runtime_error("File not opened for reading"))
        }
    }
    
    /// Python file.readlines() method
    pub fn readlines(&mut self) -> Result<Vec<String>, PyException> {
        use std::io::BufRead;
        
        if let Some(reader) = &mut self.reader {
            let lines: Result<Vec<_>, _> = reader.lines().collect();
            lines.map_err(|e| runtime_error(&format!("Read error: {}", e)))
        } else {
            Err(runtime_error("File not opened for reading"))
        }
    }
    
    /// Python file.write() method
    pub fn write<D: AsRef<str>>(&mut self, data: D) -> Result<usize, PyException> {
        use std::io::Write;
        
        if let Some(writer) = &mut self.writer {
            writer.write(data.as_ref().as_bytes())
                .map_err(|e| runtime_error(&format!("Write error: {}", e)))
        } else {
            Err(runtime_error("File not opened for writing"))
        }
    }
    
    /// Python file.writelines() method
    pub fn writelines<S: AsRef<str>>(&mut self, lines: &[S]) -> Result<(), PyException> {
        for line in lines {
            self.write(line)?;
        }
        Ok(())
    }
    
    /// Python file.close() method
    pub fn close(&mut self) -> Result<(), PyException> {
        use std::io::Write;
        
        if let Some(mut writer) = self.writer.take() {
            writer.flush()
                .map_err(|e| runtime_error(&format!("Flush error: {}", e)))?;
        }
        
        self.reader = None;
        Ok(())
    }
}

// ============================================================================
// COMPILER INTEGRATION HELPERS
// ============================================================================

/// Helper function for list creation from Rust vectors (common in compiled code)
pub fn py_list<T>(items: Vec<T>) -> PyList<T> {
    PyList::from_vec(items)
}

/// Helper function for dictionary creation (common in compiled code)
pub fn py_dict<K, V>() -> PyDictionary<K, V> 
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    PyDictionary::new()
}

/// Helper function for set creation (common in compiled code)
pub fn py_set<T>() -> PySet<T> 
where
    T: Clone + Eq + Hash,
{
    PySet::new()
}

/// Helper function for tuple creation (common in compiled code)
pub fn py_tuple<T>(items: Vec<T>) -> PyTuple<T> {
    PyTuple::new(items)
}

/// Helper for string formatting (common in f-strings compilation)
pub fn format_string<T: AsRef<str>>(template: T, args: &[&dyn Display]) -> String {
    let mut result = template.as_ref().to_string();
    for (i, arg) in args.iter().enumerate() {
        let placeholder = format!("{{{}}}", i);
        result = result.replace(&placeholder, &format!("{}", arg));
    }
    result
}

/// Helper for range() function with optional parameters - more flexible than the basic range
pub fn range_flexible(start: i64, stop: Option<i64>, step: Option<i64>) -> Vec<i64> {
    let (actual_start, actual_stop, actual_step) = match (stop, step) {
        (None, None) => (0, start, 1),
        (Some(stop), None) => (start, stop, 1),
        (Some(stop), Some(step)) => (start, stop, step),
        (None, Some(_)) => panic!("range() missing required argument 'stop'"),
    };
    
    if actual_step == 0 {
        panic!("range() step argument must not be zero");
    }
    
    let mut result = Vec::new();
    let mut current = actual_start;
    
    if actual_step > 0 {
        while current < actual_stop {
            result.push(current);
            current += actual_step;
        }
    } else {
        while current > actual_stop {
            result.push(current);
            current += actual_step;
        }
    }
    
    result
}

/// Helper for enumerate() function with slice input - returns pairs of (index, reference)
pub fn enumerate_slice<T>(iterable: &[T]) -> Vec<(usize, &T)> {
    iterable.iter().enumerate().collect()
}

/// Helper for zip() function with slice inputs - combines two iterables with lifetimes
pub fn zip_slices<'a, T, U>(iterable1: &'a [T], iterable2: &'a [U]) -> Vec<(&'a T, &'a U)> {
    iterable1.iter().zip(iterable2.iter()).collect()
}

/// Helper for Python-style slicing
pub fn slice<T>(items: &[T], start: Option<i64>, stop: Option<i64>, step: Option<i64>) -> Vec<T> 
where
    T: Clone,
{
    let len = items.len() as i64;
    let step = step.unwrap_or(1);
    
    if step == 0 {
        panic!("slice step cannot be zero");
    }
    
    let (start, stop) = match (start, stop) {
        (None, None) => (0, len),
        (Some(s), None) => (s.max(0), len),
        (None, Some(e)) => (0, e.min(len)),
        (Some(s), Some(e)) => (s.max(0), e.min(len)),
    };
    
    let mut result = Vec::new();
    let mut current = start;
    
    if step > 0 {
        while current < stop && current < len {
            if current >= 0 {
                result.push(items[current as usize].clone());
            }
            current += step;
        }
    } else {
        while current > stop && current >= 0 {
            if current < len {
                result.push(items[current as usize].clone());
            }
            current += step;
        }
    }
    
    result
}

/// Helper for Python-style string multiplication
pub fn multiply_string<S: AsRef<str>>(s: S, count: i64) -> String {
    if count <= 0 {
        String::new()
    } else {
        s.as_ref().repeat(count as usize)
    }
}

/// Helper for Python-style list multiplication
pub fn multiply_list<T>(items: &[T], count: i64) -> Vec<T> 
where
    T: Clone,
{
    if count <= 0 {
        Vec::new()
    } else {
        items.iter().cycle().take(items.len() * count as usize).cloned().collect()
    }
}

/// Helper for in/not in operations on strings
pub fn string_contains<H: AsRef<str>, N: AsRef<str>>(haystack: H, needle: N) -> bool {
    haystack.as_ref().contains(needle.as_ref())
}

/// Helper for in/not in operations on lists
pub fn list_contains<T>(items: &[T], item: &T) -> bool 
where
    T: PartialEq,
{
    items.contains(item)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[cfg(not(feature = "std"))]
    use alloc::vec;

    #[test]
    fn test_python_functions() {
        // Test generic abs function
        assert_eq!(abs(-5i64), 5);
        assert_eq!(abs(-3.14f64), 3.14);
        assert_eq!(abs(-42i32), 42);
        assert_eq!(abs(-2.5f32), 2.5);
        
        // Test generic sum function  
        let nums_i64 = vec![1i64, 2, 3, 4, 5];
        assert_eq!(sum(&nums_i64[..]), 15);
        
        let nums_f64 = vec![1.5f64, 2.5, 3.0];
        assert_eq!(sum(&nums_f64[..]), 7.0);
        
        let nums_i32 = vec![10i32, 20, 30];
        assert_eq!(sum(&nums_i32[..]), 60);
        
        // Test with PyList
        let pylist = PyList::from_vec(vec![1i64, 2, 3]);
        assert_eq!(sum(&pylist), 6);
        
        // Test min/max
        assert_eq!(min(&nums_i64), Some(1));
        assert_eq!(max(&nums_i64), Some(5));
        
        // Test all/any
        let bools = vec![true, true, false];
        assert_eq!(any(&bools), true);
        assert_eq!(all(&bools), false);
    }
    
    #[test]
    fn test_generic_type_conversions() {
        // Test generic bool conversion
        assert_eq!(bool(42i64), true);
        assert_eq!(bool(0i64), false);
        assert_eq!(bool(3.14f64), true);
        assert_eq!(bool(0.0f64), false);
        assert_eq!(bool("hello"), true);
        assert_eq!(bool(""), false);
        
        // Test generic int conversion
        assert_eq!(int("123").unwrap(), 123);
        assert_eq!(int(45.7f64).unwrap(), 45);
        assert_eq!(int(true).unwrap(), 1);
        assert_eq!(int(false).unwrap(), 0);
        
        // Test generic float conversion
        assert_eq!(float("3.14").unwrap(), 3.14);
        assert_eq!(float(42i64).unwrap(), 42.0);
        
        // Test generic str conversion
        assert_eq!(str(123i64), "123");
        assert_eq!(str(3.14f64), "3.14");
        assert_eq!(str(true), "True");
        assert_eq!(str(false), "False");
        assert_eq!(str("hello"), "hello");
    }
    
    #[test]
    fn test_pystr() {
        let s = PyStr::new("hello world");
        assert_eq!(s.len(), 11);
        assert_eq!(s.upper().as_str(), "HELLO WORLD");
        assert_eq!(s.split(Some(" ")).len(), 2);
        assert_eq!(s.find("world"), 6);
        assert_eq!(s.count("l"), 3);
    }
    
    #[test]
    fn test_pylist() {
        let mut list = PyList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        
        assert_eq!(list.len(), 3);
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.pop(None), Some(3));
        assert_eq!(list.len(), 2);
    }
    
    #[test]
    fn test_pydict() {
        let mut dict = PyDictionary::new();
        dict.set("key1".to_string(), 42);
        dict.set("key2".to_string(), 100);
        
        assert_eq!(dict.len(), 2);
        assert_eq!(dict.get(&"key1".to_string()), Some(&42));
        assert_eq!(dict.keys().len(), 2);
    }
    
    #[test]
    fn test_pyset() {
        let mut set = PySet::new();
        set.add(1);
        set.add(2);
        set.add(1); // duplicate
        
        assert_eq!(set.len(), 2);
        assert!(set.contains(&1));
        assert!(!set.contains(&3));
    }
    
    #[test]
    fn test_compiler_helpers() {
        // Test range function
        assert_eq!(range_flexible(3, None, None), vec![0, 1, 2]);
        assert_eq!(range_flexible(1, Some(4), None), vec![1, 2, 3]);
        assert_eq!(range_flexible(0, Some(10), Some(2)), vec![0, 2, 4, 6, 8]);
        
        // Test enumerate
        let items = vec!["a", "b", "c"];
        let enumerated = enumerate_slice(&items);
        assert_eq!(enumerated, vec![(0, &"a"), (1, &"b"), (2, &"c")]);
        
        // Test zip
        let nums = vec![1, 2, 3];
        let chars = vec!['a', 'b', 'c'];
        let zipped = zip_slices(&nums, &chars);
        assert_eq!(zipped, vec![(&1, &'a'), (&2, &'b'), (&3, &'c')]);
        
        // Test string multiplication
        assert_eq!(multiply_string("abc", 3), "abcabcabc");
        assert_eq!(multiply_string("x", 0), "");
        
        // Test list multiplication
        let list = vec![1, 2];
        assert_eq!(multiply_list(&list, 3), vec![1, 2, 1, 2, 1, 2]);
        
        // Test contains operations
        assert!(string_contains("hello world", "world"));
        assert!(!string_contains("hello", "xyz"));
        
        let list = vec![1, 2, 3, 4, 5];
        assert!(list_contains(&list, &3));
        assert!(!list_contains(&list, &10));
        
        // Test slicing
        let items = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(slice(&items, Some(1), Some(4), None), vec![1, 2, 3]);
        assert_eq!(slice(&items, None, Some(3), None), vec![0, 1, 2]);
        assert_eq!(slice(&items, Some(0), None, Some(2)), vec![0, 2, 4]);
    }
    
    #[test]
    fn test_helper_constructors() {
        // Test py_list
        let list = py_list(vec![1, 2, 3]);
        assert_eq!(list.len(), 3);
        
        // Test py_dict
        let mut dict: PyDictionary<String, i32> = py_dict();
        dict.set("key".to_string(), 42);
        assert_eq!(dict.len(), 1);
        
        // Test py_set
        let mut set: PySet<i32> = py_set();
        set.add(1);
        set.add(2);
        assert_eq!(set.len(), 2);
        
        // Test py_tuple
        let tuple = py_tuple(vec![1, 2, 3]);
        assert_eq!(tuple.len(), 3);
    }
}
