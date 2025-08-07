//! Python collections module implementation
//! 
//! This module provides specialized container datatypes.
//! Implementation matches Python's collections module API.

use crate::{PyException, Len, Truthy, python_function};
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

/// Counter - dict subclass for counting hashable objects
#[derive(Debug, Clone)]
pub struct Counter<T> 
where 
    T: Hash + Eq + Clone + std::fmt::Debug,
{
    counts: HashMap<T, i64>,
}

impl<T> Counter<T> 
where 
    T: Hash + Eq + Clone + std::fmt::Debug,
{
    /// Create a new Counter
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }
    
    /// Create Counter from iterable
    pub fn from_iter<I>(iterable: I) -> Self 
    where 
        I: IntoIterator<Item = T>,
    {
        let mut counter = Self::new();
        for item in iterable {
            counter.update_one(&item, 1);
        }
        counter
    }
    
    /// Update counts with elements from iterable
    pub fn update<I>(&mut self, iterable: I) 
    where 
        I: IntoIterator<Item = T>,
    {
        for item in iterable {
            self.update_one(&item, 1);
        }
    }
    
    /// Update count for single element
    pub fn update_one(&mut self, element: &T, count: i64) {
        *self.counts.entry(element.clone()).or_insert(0) += count;
        if self.counts[element] <= 0 {
            self.counts.remove(element);
        }
    }
    
    /// Get count for element
    pub fn get(&self, element: &T) -> i64 {
        self.counts.get(element).copied().unwrap_or(0)
    }
    
    /// Get most common elements
    pub fn most_common(&self, n: Option<usize>) -> Vec<(T, i64)> {
        let mut items: Vec<(T, i64)> = self.counts.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        items.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| format!("{:?}", a.0).cmp(&format!("{:?}", b.0))));
        
        match n {
            Some(limit) => items.into_iter().take(limit).collect(),
            None => items,
        }
    }
    
    /// Get all elements (with repetitions)
    pub fn elements(&self) -> Vec<T> {
        let mut result = Vec::new();
        for (element, count) in &self.counts {
            for _ in 0..*count {
                result.push(element.clone());
            }
        }
        result
    }
    
    /// Subtract counts from another counter
    pub fn subtract(&mut self, other: &Counter<T>) {
        for (element, count) in &other.counts {
            self.update_one(element, -count);
        }
    }
    
    /// Get total count
    pub fn total(&self) -> i64 {
        self.counts.values().sum()
    }
    
    /// Clear all counts
    pub fn clear(&mut self) {
        self.counts.clear();
    }
    
    /// Get keys (elements)
    pub fn keys(&self) -> Vec<T> {
        self.counts.keys().cloned().collect()
    }
    
    /// Get values (counts)
    pub fn values(&self) -> Vec<i64> {
        self.counts.values().copied().collect()
    }
    
    /// Get items (element, count pairs)
    pub fn items(&self) -> Vec<(T, i64)> {
        self.counts.iter().map(|(k, v)| (k.clone(), *v)).collect()
    }
}

impl<T> Default for Counter<T> 
where 
    T: Hash + Eq + Clone + std::fmt::Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Len for Counter<T> 
where 
    T: Hash + Eq + Clone + std::fmt::Debug,
{
    fn len(&self) -> usize {
        self.counts.len()
    }
}

impl<T> Truthy for Counter<T> 
where 
    T: Hash + Eq + Clone + std::fmt::Debug,
{
    fn is_truthy(&self) -> bool {
        !self.counts.is_empty()
    }
}

/// deque - double-ended queue
#[derive(Debug, Clone)]
pub struct deque<T> {
    inner: VecDeque<T>,
    maxlen: Option<usize>,
}

impl<T> deque<T> {
    /// Create a new deque
    pub fn new() -> Self {
        Self {
            inner: VecDeque::new(),
            maxlen: None,
        }
    }
    
    /// Create deque from iterable
    pub fn from_iter<I>(iterable: I, maxlen: Option<usize>) -> Self 
    where 
        I: IntoIterator<Item = T>,
    {
        let mut deque = Self {
            inner: VecDeque::new(),
            maxlen,
        };
        for item in iterable {
            deque.append(item);
        }
        deque
    }
    
    /// Create deque with maximum length
    pub fn with_maxlen(maxlen: usize) -> Self {
        Self {
            inner: VecDeque::new(),
            maxlen: Some(maxlen),
        }
    }
    
    /// Add element to right end
    pub fn append(&mut self, item: T) {
        self.inner.push_back(item);
        self.check_maxlen();
    }
    
    /// Add element to left end
    pub fn appendleft(&mut self, item: T) {
        self.inner.push_front(item);
        self.check_maxlen();
    }
    
    /// Remove and return element from right end
    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop_back()
    }
    
    /// Remove and return element from left end
    pub fn popleft(&mut self) -> Option<T> {
        self.inner.pop_front()
    }
    
    /// Extend right side with iterable
    pub fn extend<I>(&mut self, iterable: I) 
    where 
        I: IntoIterator<Item = T>,
    {
        for item in iterable {
            self.append(item);
        }
    }
    
    /// Extend left side with iterable
    pub fn extendleft<I>(&mut self, iterable: I) 
    where 
        I: IntoIterator<Item = T>,
    {
        for item in iterable {
            self.appendleft(item);
        }
    }
    
    /// Remove first occurrence of value
    pub fn remove(&mut self, value: &T) -> Result<(), PyException> 
    where 
        T: PartialEq,
    {
        if let Some(pos) = self.inner.iter().position(|x| x == value) {
            self.inner.remove(pos);
            Ok(())
        } else {
            Err(crate::value_error("deque.remove(x): x not in deque"))
        }
    }
    
    /// Rotate deque n steps
    pub fn rotate(&mut self, n: i32) {
        if self.inner.is_empty() {
            return;
        }
        
        let len = self.inner.len() as i32;
        let steps = ((n % len) + len) % len;
        
        for _ in 0..steps {
            if let Some(item) = self.inner.pop_back() {
                self.inner.push_front(item);
            }
        }
    }
    
    /// Reverse the deque in place
    pub fn reverse(&mut self) {
        let items: Vec<T> = self.inner.drain(..).collect();
        for item in items.into_iter().rev() {
            self.inner.push_back(item);
        }
    }
    
    /// Count occurrences of value
    pub fn count(&self, value: &T) -> usize 
    where 
        T: PartialEq,
    {
        self.inner.iter().filter(|&x| x == value).count()
    }
    
    /// Find index of first occurrence
    pub fn index(&self, value: &T, start: Option<usize>, stop: Option<usize>) -> Result<usize, PyException> 
    where 
        T: PartialEq,
    {
        let start = start.unwrap_or(0);
        let stop = stop.unwrap_or(self.inner.len());
        
        for (i, item) in self.inner.iter().enumerate().skip(start).take(stop - start) {
            if item == value {
                return Ok(i);
            }
        }
        
        Err(crate::value_error("deque.index(x): x not in deque"))
    }
    
    /// Insert item at position
    pub fn insert(&mut self, index: usize, item: T) {
        if index >= self.inner.len() {
            self.inner.push_back(item);
        } else {
            self.inner.insert(index, item);
        }
        self.check_maxlen();
    }
    
    /// Clear the deque
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    
    /// Copy the deque
    pub fn copy(&self) -> Self 
    where 
        T: Clone,
    {
        Self {
            inner: self.inner.clone(),
            maxlen: self.maxlen,
        }
    }
    
    /// Get maximum length
    pub fn maxlen(&self) -> Option<usize> {
        self.maxlen
    }
    
    fn check_maxlen(&mut self) {
        if let Some(max_len) = self.maxlen {
            while self.inner.len() > max_len {
                self.inner.pop_front();
            }
        }
    }
    
    /// Get item by index
    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index)
    }
    
    /// Get item by index (mutable)
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.inner.get_mut(index)
    }
}

impl<T> Default for deque<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Len for deque<T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> Truthy for deque<T> {
    fn is_truthy(&self) -> bool {
        !self.inner.is_empty()
    }
}

/// defaultdict - dict subclass with default factory function
#[derive(Debug, Clone)]
pub struct defaultdict<K, V> 
where 
    K: Hash + Eq + Clone,
    V: Clone,
{
    inner: HashMap<K, V>,
    default_factory: Option<fn() -> V>,
}

impl<K, V> defaultdict<K, V> 
where 
    K: Hash + Eq + Clone,
    V: Clone,
{
    /// Create new defaultdict with factory function
    pub fn new(default_factory: fn() -> V) -> Self {
        Self {
            inner: HashMap::new(),
            default_factory: Some(default_factory),
        }
    }
    
    /// Create defaultdict without factory
    pub fn without_factory() -> Self {
        Self {
            inner: HashMap::new(),
            default_factory: None,
        }
    }
    
    /// Get value, creating with factory if missing
    pub fn get_or_default(&mut self, key: &K) -> Result<V, PyException> {
        if let Some(value) = self.inner.get(key) {
            Ok(value.clone())
        } else if let Some(factory) = self.default_factory {
            let default_value = factory();
            self.inner.insert(key.clone(), default_value.clone());
            Ok(default_value)
        } else {
            Err(crate::key_error(format!("Key not found and no default factory")))
        }
    }
    
    /// Get value without creating default
    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }
    
    /// Set value
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }
    
    /// Remove key
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.inner.remove(key)
    }
    
    /// Check if key exists
    pub fn contains_key(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }
    
    /// Get keys
    pub fn keys(&self) -> Vec<K> {
        self.inner.keys().cloned().collect()
    }
    
    /// Get values
    pub fn values(&self) -> Vec<V> {
        self.inner.values().cloned().collect()
    }
    
    /// Get items
    pub fn items(&self) -> Vec<(K, V)> {
        self.inner.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
    
    /// Clear all items
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    
    /// Get missing method (for compatibility)
    pub fn default_factory_fn(&self) -> Option<fn() -> V> {
        self.default_factory
    }
}

impl<K, V> Len for defaultdict<K, V> 
where 
    K: Hash + Eq + Clone,
    V: Clone,
{
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V> Truthy for defaultdict<K, V> 
where 
    K: Hash + Eq + Clone,
    V: Clone,
{
    fn is_truthy(&self) -> bool {
        !self.inner.is_empty()
    }
}

/// OrderedDict - dictionary that maintains insertion order
/// Note: In Rust, HashMap doesn't guarantee order, so we use a Vec for ordering
#[derive(Debug, Clone)]
pub struct OrderedDict<K, V> 
where 
    K: Hash + Eq + Clone,
    V: Clone,
{
    inner: HashMap<K, V>,
    order: Vec<K>,
}

impl<K, V> OrderedDict<K, V> 
where 
    K: Hash + Eq + Clone,
    V: Clone,
{
    /// Create new OrderedDict
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            order: Vec::new(),
        }
    }
    
    /// Insert key-value pair
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if !self.inner.contains_key(&key) {
            self.order.push(key.clone());
        }
        self.inner.insert(key, value)
    }
    
    /// Get value by key
    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }
    
    /// Remove key-value pair
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.inner.remove(key) {
            self.order.retain(|k| k != key);
            Some(value)
        } else {
            None
        }
    }
    
    /// Pop last item (LIFO order)
    pub fn popitem(&mut self, last: bool) -> Result<(K, V), PyException> {
        if self.order.is_empty() {
            return Err(crate::key_error("dictionary is empty"));
        }
        
        let key = if last {
            self.order.pop().unwrap()
        } else {
            self.order.remove(0)
        };
        
        let value = self.inner.remove(&key).unwrap();
        Ok((key, value))
    }
    
    /// Move key to end (or beginning)
    pub fn move_to_end(&mut self, key: &K, last: bool) -> Result<(), PyException> {
        if !self.inner.contains_key(key) {
            return Err(crate::key_error("Key not found"));
        }
        
        self.order.retain(|k| k != key);
        if last {
            self.order.push(key.clone());
        } else {
            self.order.insert(0, key.clone());
        }
        
        Ok(())
    }
    
    /// Get keys in order
    pub fn keys(&self) -> Vec<K> {
        self.order.clone()
    }
    
    /// Get values in order
    pub fn values(&self) -> Vec<V> {
        self.order.iter().filter_map(|k| self.inner.get(k).cloned()).collect()
    }
    
    /// Get items in order
    pub fn items(&self) -> Vec<(K, V)> {
        self.order.iter().filter_map(|k| self.inner.get(k).map(|v| (k.clone(), v.clone()))).collect()
    }
    
    /// Clear all items
    pub fn clear(&mut self) {
        self.inner.clear();
        self.order.clear();
    }
    
    /// Check if key exists
    pub fn contains_key(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }
}

impl<K, V> Default for OrderedDict<K, V> 
where 
    K: Hash + Eq + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Len for OrderedDict<K, V> 
where 
    K: Hash + Eq + Clone,
    V: Clone,
{
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V> Truthy for OrderedDict<K, V> 
where 
    K: Hash + Eq + Clone,
    V: Clone,
{
    fn is_truthy(&self) -> bool {
        !self.inner.is_empty()
    }
}

/// ChainMap - groups multiple mappings into single view
#[derive(Debug)]
pub struct ChainMap<K, V> 
where 
    K: Hash + Eq + Clone,
    V: Clone,
{
    maps: Vec<HashMap<K, V>>,
}

impl<K, V> ChainMap<K, V> 
where 
    K: Hash + Eq + Clone,
    V: Clone,
{
    /// Create new ChainMap
    pub fn new(maps: Vec<HashMap<K, V>>) -> Self {
        Self { maps }
    }
    
    /// Create empty ChainMap
    pub fn empty() -> Self {
        Self { maps: vec![HashMap::new()] }
    }
    
    /// Get value by key (searches all maps)
    pub fn get(&self, key: &K) -> Option<&V> {
        for map in &self.maps {
            if let Some(value) = map.get(key) {
                return Some(value);
            }
        }
        None
    }
    
    /// Set value (in first map)
    pub fn insert(&mut self, key: K, value: V) {
        if self.maps.is_empty() {
            self.maps.push(HashMap::new());
        }
        self.maps[0].insert(key, value);
    }
    
    /// Remove key from first map
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if !self.maps.is_empty() {
            self.maps[0].remove(key)
        } else {
            None
        }
    }
    
    /// Get all keys
    pub fn keys(&self) -> Vec<K> {
        let mut keys = std::collections::HashSet::new();
        for map in &self.maps {
            keys.extend(map.keys().cloned());
        }
        keys.into_iter().collect()
    }
    
    /// Get all values
    pub fn values(&self) -> Vec<V> {
        let mut seen_keys = std::collections::HashSet::new();
        let mut values = Vec::new();
        
        for map in &self.maps {
            for (key, value) in map {
                if !seen_keys.contains(key) {
                    seen_keys.insert(key.clone());
                    values.push(value.clone());
                }
            }
        }
        
        values
    }
    
    /// Check if key exists
    pub fn contains_key(&self, key: &K) -> bool {
        self.maps.iter().any(|map| map.contains_key(key))
    }
    
    /// Add new child map
    pub fn new_child(&mut self, map: HashMap<K, V>) -> &mut Self {
        self.maps.insert(0, map);
        self
    }
    
    /// Get number of maps
    pub fn num_maps(&self) -> usize {
        self.maps.len()
    }
}

impl<K, V> Len for ChainMap<K, V> 
where 
    K: Hash + Eq + Clone,
    V: Clone,
{
    fn len(&self) -> usize {
        self.keys().len()
    }
}

// Convenience functions
python_function! {
    /// Create counter from iterable
    pub fn counter<I>(iterable: I) -> Counter<String>
    where [I: IntoIterator<Item = String>]
    [signature: (iterable)]
    [concrete_types: (Vec<String>) -> Counter<String>]
    {
        Counter::from_iter(iterable)
    }
}

python_function! {
    /// Create deque from iterable
    pub fn create_deque<I>(iterable: I, maxlen: Option<usize>) -> deque<String>
    where [I: IntoIterator<Item = String>]
    [signature: (iterable, maxlen=None)]
    [concrete_types: (Vec<String>, Option<usize>) -> deque<String>]
    {
        deque::from_iter(iterable, maxlen)
    }
}

python_function! {
    /// Create defaultdict with int factory
    pub fn defaultdict_int() -> defaultdict<String, i64>
    [signature: ()]
    [concrete_types: () -> defaultdict<String, i64>]
    {
        defaultdict::new(|| 0i64)
    }
}

python_function! {
    /// Create defaultdict with list factory
    pub fn defaultdict_list() -> defaultdict<String, Vec<String>>
    [signature: ()]
    [concrete_types: () -> defaultdict<String, Vec<String>>]
    {
        defaultdict::new(|| Vec::new())
    }
}