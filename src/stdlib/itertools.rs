//! Python itertools module implementation
//! 
//! This module provides functions creating iterators for efficient looping.
//! Implementation matches Python's itertools module API.

use std::collections::VecDeque;

/// count - infinite iterator starting from start
#[derive(Debug, Clone)]
pub struct Count<T> {
    current: T,
    step: T,
}

impl<T> Count<T> 
where
    T: Clone + std::ops::Add<Output = T>,
{
    /// Create new count iterator
    pub fn new(start: T, step: T) -> Self {
        Self {
            current: start,
            step,
        }
    }
    
    /// Get next value
    pub fn next(&mut self) -> T {
        let result = self.current.clone();
        self.current = self.current.clone() + self.step.clone();
        result
    }
    
    /// Take n values from count
    pub fn take(&mut self, n: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            result.push(self.next());
        }
        result
    }
}

/// cycle - infinite iterator cycling through iterable
#[derive(Debug, Clone)]
pub struct Cycle<T> {
    items: Vec<T>,
    index: usize,
}

impl<T> Cycle<T> 
where
    T: Clone,
{
    /// Create new cycle iterator
    pub fn new<I>(iterable: I) -> Self 
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            items: iterable.into_iter().collect(),
            index: 0,
        }
    }
    
    /// Get next value (cycles through items)
    pub fn next(&mut self) -> Option<T> {
        if self.items.is_empty() {
            return None;
        }
        
        let result = self.items[self.index].clone();
        self.index = (self.index + 1) % self.items.len();
        Some(result)
    }
    
    /// Take n values from cycle
    pub fn take(&mut self, n: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            if let Some(item) = self.next() {
                result.push(item);
            }
        }
        result
    }
}

/// repeat - infinite iterator returning same value
#[derive(Debug, Clone)]
pub struct Repeat<T> {
    item: T,
    times: Option<usize>,
    count: usize,
}

impl<T> Repeat<T> 
where
    T: Clone,
{
    /// Create infinite repeat iterator
    pub fn infinite(item: T) -> Self {
        Self {
            item,
            times: None,
            count: 0,
        }
    }
    
    /// Create repeat iterator with limit
    pub fn times(item: T, times: usize) -> Self {
        Self {
            item,
            times: Some(times),
            count: 0,
        }
    }
    
    /// Get next value
    pub fn next(&mut self) -> Option<T> {
        if let Some(limit) = self.times {
            if self.count >= limit {
                return None;
            }
        }
        
        self.count += 1;
        Some(self.item.clone())
    }
    
    /// Take n values from repeat
    pub fn take(&mut self, n: usize) -> Vec<T> {
        let mut result = Vec::new();
        for _ in 0..n {
            if let Some(item) = self.next() {
                result.push(item);
            } else {
                break;
            }
        }
        result
    }
}

/// chain - iterator chaining multiple iterables
#[derive(Debug)]
pub struct Chain<T> {
    iterables: VecDeque<Vec<T>>,
    current: usize,
}

impl<T> Chain<T> 
where
    T: Clone,
{
    /// Create new chain iterator
    pub fn new() -> Self {
        Self {
            iterables: VecDeque::new(),
            current: 0,
        }
    }
    
    /// Add iterable to chain
    pub fn add<I>(&mut self, iterable: I) 
    where
        I: IntoIterator<Item = T>,
    {
        self.iterables.push_back(iterable.into_iter().collect());
    }
    
    /// Get next value from chain
    pub fn next(&mut self) -> Option<T> {
        while !self.iterables.is_empty() {
            if let Some(current_vec) = self.iterables.front_mut() {
                if self.current < current_vec.len() {
                    let result = current_vec.get(self.current).cloned();
                    self.current += 1;
                    return result;
                } else {
                    self.iterables.pop_front();
                    self.current = 0;
                }
            } else {
                break;
            }
        }
        None
    }
    
    /// Collect all remaining items
    pub fn collect(mut self) -> Vec<T> {
        let mut result = Vec::new();
        while let Some(item) = self.next() {
            result.push(item);
        }
        result
    }
}

impl<T> Default for Chain<T> 
where
    T: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

/// islice - iterator slice with start, stop, step
#[derive(Debug)]
pub struct ISlice<T> {
    items: Vec<T>,
    stop: Option<usize>,
    step: usize,
    current: usize,
}

impl<T> ISlice<T> 
where
    T: Clone,
{
    /// Create islice iterator
    pub fn new<I>(iterable: I, start: usize, stop: Option<usize>, step: usize) -> Self 
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            items: iterable.into_iter().collect(),
            stop,
            step: step.max(1), // Ensure step is at least 1
            current: start,
        }
    }
    
    /// Get next value from slice
    pub fn next(&mut self) -> Option<T> {
        if self.current >= self.items.len() {
            return None;
        }
        
        if let Some(stop) = self.stop {
            if self.current >= stop {
                return None;
            }
        }
        
        let result = self.items.get(self.current).cloned();
        self.current += self.step;
        result
    }
    
    /// Collect all remaining items
    pub fn collect(mut self) -> Vec<T> {
        let mut result = Vec::new();
        while let Some(item) = self.next() {
            result.push(item);
        }
        result
    }
}

/// takewhile - iterator yielding items while predicate is true
#[derive(Debug)]
pub struct TakeWhile<T, F> {
    items: Vec<T>,
    predicate: F,
    index: usize,
    stopped: bool,
}

impl<T, F> TakeWhile<T, F> 
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    /// Create takewhile iterator
    pub fn new<I>(iterable: I, predicate: F) -> Self 
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            items: iterable.into_iter().collect(),
            predicate,
            index: 0,
            stopped: false,
        }
    }
    
    /// Get next value while predicate is true
    pub fn next(&mut self) -> Option<T> {
        if self.stopped || self.index >= self.items.len() {
            return None;
        }
        
        if let Some(item) = self.items.get(self.index) {
            if (self.predicate)(item) {
                self.index += 1;
                Some(item.clone())
            } else {
                self.stopped = true;
                None
            }
        } else {
            None
        }
    }
    
    /// Collect all remaining items
    pub fn collect(mut self) -> Vec<T> {
        let mut result = Vec::new();
        while let Some(item) = self.next() {
            result.push(item);
        }
        result
    }
}

/// dropwhile - iterator dropping items while predicate is true
#[derive(Debug)]
pub struct DropWhile<T, F> {
    items: Vec<T>,
    predicate: F,
    index: usize,
    started: bool,
}

impl<T, F> DropWhile<T, F> 
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    /// Create dropwhile iterator
    pub fn new<I>(iterable: I, predicate: F) -> Self 
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            items: iterable.into_iter().collect(),
            predicate,
            index: 0,
            started: false,
        }
    }
    
    /// Get next value after dropping initial items
    pub fn next(&mut self) -> Option<T> {
        if !self.started {
            // Skip items while predicate is true
            while self.index < self.items.len() {
                if let Some(item) = self.items.get(self.index) {
                    if !(self.predicate)(item) {
                        self.started = true;
                        break;
                    }
                    self.index += 1;
                } else {
                    break;
                }
            }
        }
        
        if self.index < self.items.len() {
            let result = self.items.get(self.index).cloned();
            self.index += 1;
            result
        } else {
            None
        }
    }
    
    /// Collect all remaining items
    pub fn collect(mut self) -> Vec<T> {
        let mut result = Vec::new();
        while let Some(item) = self.next() {
            result.push(item);
        }
        result
    }
}

/// filterfalse - iterator filtering items where predicate is false
#[derive(Debug)]
pub struct FilterFalse<T, F> {
    items: Vec<T>,
    predicate: F,
    index: usize,
}

impl<T, F> FilterFalse<T, F> 
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    /// Create filterfalse iterator
    pub fn new<I>(iterable: I, predicate: F) -> Self 
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            items: iterable.into_iter().collect(),
            predicate,
            index: 0,
        }
    }
    
    /// Get next value where predicate is false
    pub fn next(&mut self) -> Option<T> {
        while self.index < self.items.len() {
            if let Some(item) = self.items.get(self.index) {
                self.index += 1;
                if !(self.predicate)(item) {
                    return Some(item.clone());
                }
            } else {
                break;
            }
        }
        None
    }
    
    /// Collect all remaining items
    pub fn collect(mut self) -> Vec<T> {
        let mut result = Vec::new();
        while let Some(item) = self.next() {
            result.push(item);
        }
        result
    }
}

/// compress - iterator selecting items based on selectors
#[derive(Debug)]
pub struct Compress<T> {
    data: Vec<T>,
    selectors: Vec<bool>,
    index: usize,
}

impl<T> Compress<T> 
where
    T: Clone,
{
    /// Create compress iterator
    pub fn new<I, S>(data: I, selectors: S) -> Self 
    where
        I: IntoIterator<Item = T>,
        S: IntoIterator<Item = bool>,
    {
        Self {
            data: data.into_iter().collect(),
            selectors: selectors.into_iter().collect(),
            index: 0,
        }
    }
    
    /// Get next selected value
    pub fn next(&mut self) -> Option<T> {
        while self.index < self.data.len().min(self.selectors.len()) {
            let current_index = self.index;
            self.index += 1;
            
            if self.selectors.get(current_index).copied().unwrap_or(false) {
                return self.data.get(current_index).cloned();
            }
        }
        None
    }
    
    /// Collect all remaining items
    pub fn collect(mut self) -> Vec<T> {
        let mut result = Vec::new();
        while let Some(item) = self.next() {
            result.push(item);
        }
        result
    }
}

// Module-level convenience functions

/// count - create count iterator
pub fn count<T>(start: T, step: T) -> Count<T> 
where
    T: Clone + std::ops::Add<Output = T>,
{
    Count::new(start, step)
}

/// cycle - create cycle iterator
pub fn cycle<T, I>(iterable: I) -> Cycle<T> 
where
    T: Clone,
    I: IntoIterator<Item = T>,
{
    Cycle::new(iterable)
}

/// repeat - create repeat iterator
pub fn repeat<T>(item: T) -> Repeat<T> 
where
    T: Clone,
{
    Repeat::infinite(item)
}

/// repeat_times - create limited repeat iterator
pub fn repeat_times<T>(item: T, times: usize) -> Repeat<T> 
where
    T: Clone,
{
    Repeat::times(item, times)
}

/// chain - chain multiple iterables
pub fn chain_from_iterable<T, I>(iterables: I) -> Vec<T> 
where
    I: IntoIterator<Item = Vec<T>>,
{
    let mut result = Vec::new();
    for iterable in iterables {
        result.extend(iterable);
    }
    result
}

/// islice - slice iterator
pub fn islice<T, I>(iterable: I, start: usize, stop: Option<usize>, step: usize) -> Vec<T> 
where
    T: Clone,
    I: IntoIterator<Item = T>,
{
    ISlice::new(iterable, start, stop, step).collect()
}

/// takewhile - take while predicate is true
pub fn takewhile<T, I, F>(iterable: I, predicate: F) -> Vec<T> 
where
    T: Clone,
    I: IntoIterator<Item = T>,
    F: Fn(&T) -> bool,
{
    TakeWhile::new(iterable, predicate).collect()
}

/// dropwhile - drop while predicate is true
pub fn dropwhile<T, I, F>(iterable: I, predicate: F) -> Vec<T> 
where
    T: Clone,
    I: IntoIterator<Item = T>,
    F: Fn(&T) -> bool,
{
    DropWhile::new(iterable, predicate).collect()
}

/// filterfalse - filter where predicate is false
pub fn filterfalse<T, I, F>(iterable: I, predicate: F) -> Vec<T> 
where
    T: Clone,
    I: IntoIterator<Item = T>,
    F: Fn(&T) -> bool,
{
    FilterFalse::new(iterable, predicate).collect()
}

/// compress - select items based on selectors
pub fn compress<T, I, S>(data: I, selectors: S) -> Vec<T> 
where
    T: Clone,
    I: IntoIterator<Item = T>,
    S: IntoIterator<Item = bool>,
{
    Compress::new(data, selectors).collect()
}

/// combinations - generate combinations of length r
pub fn combinations<T>(iterable: &[T], r: usize) -> Vec<Vec<T>> 
where
    T: Clone,
{
    if r == 0 {
        return vec![vec![]];
    }
    
    if r > iterable.len() {
        return vec![];
    }
    
    let mut result = Vec::new();
    combinations_helper(iterable, r, 0, &mut vec![], &mut result);
    result
}

fn combinations_helper<T>(
    iterable: &[T], 
    r: usize, 
    start: usize, 
    current: &mut Vec<T>, 
    result: &mut Vec<Vec<T>>
) where
    T: Clone,
{
    if current.len() == r {
        result.push(current.clone());
        return;
    }
    
    for i in start..iterable.len() {
        current.push(iterable[i].clone());
        combinations_helper(iterable, r, i + 1, current, result);
        current.pop();
    }
}

/// permutations - generate permutations of length r
pub fn permutations<T>(iterable: &[T], r: Option<usize>) -> Vec<Vec<T>> 
where
    T: Clone,
{
    let r = r.unwrap_or(iterable.len());
    
    if r == 0 {
        return vec![vec![]];
    }
    
    if r > iterable.len() {
        return vec![];
    }
    
    let mut result = Vec::new();
    let mut used = vec![false; iterable.len()];
    permutations_helper(iterable, r, &mut vec![], &mut used, &mut result);
    result
}

fn permutations_helper<T>(
    iterable: &[T], 
    r: usize, 
    current: &mut Vec<T>, 
    used: &mut [bool], 
    result: &mut Vec<Vec<T>>
) where
    T: Clone,
{
    if current.len() == r {
        result.push(current.clone());
        return;
    }
    
    for i in 0..iterable.len() {
        if !used[i] {
            current.push(iterable[i].clone());
            used[i] = true;
            permutations_helper(iterable, r, current, used, result);
            current.pop();
            used[i] = false;
        }
    }
}

/// product - cartesian product of iterables
pub fn product<T>(iterables: &[Vec<T>]) -> Vec<Vec<T>> 
where
    T: Clone,
{
    if iterables.is_empty() {
        return vec![vec![]];
    }
    
    let mut result = vec![vec![]];
    
    for iterable in iterables {
        let mut new_result = Vec::new();
        for existing in &result {
            for item in iterable {
                let mut new_combo = existing.clone();
                new_combo.push(item.clone());
                new_result.push(new_combo);
            }
        }
        result = new_result;
    }
    
    result
}

/// accumulate - running totals
pub fn accumulate<T, F>(iterable: &[T], func: Option<F>) -> Vec<T> 
where
    T: Clone + std::ops::Add<Output = T>,
    F: Fn(&T, &T) -> T,
{
    if iterable.is_empty() {
        return vec![];
    }
    
    let mut result = vec![iterable[0].clone()];
    
    for i in 1..iterable.len() {
        let next_val = if let Some(ref f) = func {
            f(&result[i - 1], &iterable[i])
        } else {
            result[i - 1].clone() + iterable[i].clone()
        };
        result.push(next_val);
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_count() {
        let mut counter = count(0, 2);
        assert_eq!(counter.take(5), vec![0, 2, 4, 6, 8]);
    }
    
    #[test]
    fn test_cycle() {
        let mut cycler = cycle(vec![1, 2, 3]);
        assert_eq!(cycler.take(7), vec![1, 2, 3, 1, 2, 3, 1]);
    }
    
    #[test]
    fn test_repeat() {
        let mut repeater = repeat_times(5, 3);
        assert_eq!(repeater.take(5), vec![5, 5, 5]); // Only 3 items available
    }
    
    #[test]
    fn test_combinations() {
        let result = combinations(&[1, 2, 3, 4], 2);
        assert_eq!(result, vec![
            vec![1, 2], vec![1, 3], vec![1, 4],
            vec![2, 3], vec![2, 4], vec![3, 4]
        ]);
    }
    
    #[test]
    fn test_permutations() {
        let result = permutations(&[1, 2], None);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&vec![1, 2]));
        assert!(result.contains(&vec![2, 1]));
    }
    
    #[test]
    fn test_compress() {
        let result = compress(
            vec!['A', 'B', 'C', 'D'],
            vec![true, false, true, false]
        );
        assert_eq!(result, vec!['A', 'C']);
    }
}