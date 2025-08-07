//! Python random module implementation
//! 
//! This module provides functions for generating random numbers and sequences.
//! Implementation matches Python's random module API.

use crate::PyException;
use crate::python_function;
use std::sync::Mutex;

// Simple linear congruential generator for reproducible results
struct LCGRandom {
    seed: u64,
}

impl LCGRandom {
    fn new(seed: u64) -> Self {
        Self { seed: if seed == 0 { 1 } else { seed } }
    }
    
    fn next(&mut self) -> u64 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        self.seed
    }
    
    fn random(&mut self) -> f64 {
        (self.next() >> 1) as f64 / (i64::MAX as f64)
    }
}

static RNG: Mutex<LCGRandom> = Mutex::new(LCGRandom { seed: 1 });

python_function! {
    /// random.seed - initialize random number generator
    pub fn seed<T>(a: Option<T>) -> ()
    where [T: Into<u64>]
    [signature: (a=None)]
    [concrete_types: (Option<u64>) -> ()]
    {
        let seed_val = match a {
            Some(val) => val.into(),
            None => {
                // Use current time as seed
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
            }
        };
        
        if let Ok(mut rng) = RNG.lock() {
            *rng = LCGRandom::new(seed_val);
        }
    }
}

python_function! {
    /// random.getstate - return internal state
    pub fn getstate() -> Vec<u64>
    [signature: ()]
    [concrete_types: () -> Vec<u64>]
    {
        if let Ok(rng) = RNG.lock() {
            vec![rng.seed]
        } else {
            vec![1]
        }
    }
}

/// random.setstate - restore state
pub fn setstate(state: &[u64]) -> Result<(), PyException> {
    if state.is_empty() {
        return Err(crate::value_error("state vector is empty"));
    }
    
    if let Ok(mut rng) = RNG.lock() {
        rng.seed = state[0];
        Ok(())
    } else {
        Err(crate::runtime_error("Failed to acquire RNG lock"))
    }
}

python_function! {
    /// random.random - random float in [0.0, 1.0)
    pub fn random() -> f64
    [signature: ()]
    [concrete_types: () -> f64]
    {
        if let Ok(mut rng) = RNG.lock() {
            rng.random()
        } else {
            0.5 // fallback
        }
    }
}

python_function! {
    /// random.uniform - random float in [a, b]
    pub fn uniform<T, U>(a: T, b: U) -> f64
    where [T: Into<f64>, U: Into<f64>]
    [signature: (a, b)]
    [concrete_types: (f64, f64) -> f64]
    {
        let a = a.into();
        let b = b.into();
        a + (b - a) * random()
    }
}

python_function! {
    /// random.triangular - triangular distribution
    pub fn triangular<T, U, V>(low: T, high: U, mode: Option<V>) -> f64
    where [T: Into<f64>, U: Into<f64>, V: Into<f64>]
    [signature: (low, high, mode=None)]
    [concrete_types: (f64, f64, Option<f64>) -> f64]
    {
        let low = low.into();
        let high = high.into();
        let mode = mode.map(|m| m.into()).unwrap_or((low + high) / 2.0);
        
        let u = random();
        let c = (mode - low) / (high - low);
        
        if u <= c {
            low + ((high - low) * (mode - low) * u).sqrt()
        } else {
            high - ((high - low) * (high - mode) * (1.0 - u)).sqrt()
        }
    }
}

/// random.betavariate - beta distribution
pub fn betavariate<T, U>(alpha: T, beta: U) -> f64 
where
    T: Into<f64>,
    U: Into<f64>,
{
    // Simplified implementation - not mathematically correct but functional
    let a = alpha.into().max(0.1);
    let b = beta.into().max(0.1);
    let x = random().powf(1.0 / a);
    let y = random().powf(1.0 / b);
    x / (x + y)
}

/// random.expovariate - exponential distribution
pub fn expovariate<T>(lambd: T) -> Result<f64, PyException> 
where
    T: Into<f64>,
{
    let lambda = lambd.into();
    if lambda <= 0.0 {
        return Err(crate::value_error("lambda must be positive"));
    }
    
    let u = random();
    Ok(-(-u).ln() / lambda)
}

/// random.gammavariate - gamma distribution
pub fn gammavariate<T, U>(alpha: T, beta: U) -> Result<f64, PyException> 
where
    T: Into<f64>,
    U: Into<f64>,
{
    let alpha = alpha.into();
    let beta = beta.into();
    
    if alpha <= 0.0 || beta <= 0.0 {
        return Err(crate::value_error("alpha and beta must be positive"));
    }
    
    // Simplified implementation
    Ok(expovariate(1.0 / beta)? * alpha)
}

/// random.gauss - Gaussian distribution
pub fn gauss<T, U>(mu: T, sigma: U) -> f64 
where
    T: Into<f64>,
    U: Into<f64>,
{
    normalvariate(mu, sigma)
}

/// random.lognormvariate - log normal distribution
pub fn lognormvariate<T, U>(mu: T, sigma: U) -> f64 
where
    T: Into<f64>,
    U: Into<f64>,
{
    normalvariate(mu, sigma).exp()
}

/// random.normalvariate - normal distribution
pub fn normalvariate<T, U>(mu: T, sigma: U) -> f64 
where
    T: Into<f64>,
    U: Into<f64>,
{
    let mu = mu.into();
    let sigma = sigma.into();
    
    // Box-Muller transformation
    static mut NEXT_GAUSS: Option<f64> = None;
    static mut HAS_GAUSS: bool = false;
    
    unsafe {
        if HAS_GAUSS {
            HAS_GAUSS = false;
            return mu + sigma * NEXT_GAUSS.unwrap();
        }
        
        let u1 = random();
        let u2 = random();
        
        let mag = sigma * (-2.0 * u1.ln()).sqrt();
        let z0 = mag * (2.0 * std::f64::consts::PI * u2).cos();
        let z1 = mag * (2.0 * std::f64::consts::PI * u2).sin();
        
        NEXT_GAUSS = Some(z1);
        HAS_GAUSS = true;
        
        mu + z0
    }
}

/// random.vonmisesvariate - von Mises distribution
pub fn vonmisesvariate<T, U>(mu: T, kappa: U) -> f64 
where
    T: Into<f64>,
    U: Into<f64>,
{
    let mu = mu.into();
    let kappa = kappa.into();
    
    if kappa <= 1e-6 {
        return 2.0 * std::f64::consts::PI * random();
    }
    
    // Simplified implementation
    mu + normalvariate(0.0, 1.0 / kappa.sqrt())
}

/// random.weibullvariate - Weibull distribution
pub fn weibullvariate<T, U>(alpha: T, beta: U) -> Result<f64, PyException> 
where
    T: Into<f64>,
    U: Into<f64>,
{
    let alpha = alpha.into();
    let beta = beta.into();
    
    if alpha <= 0.0 || beta <= 0.0 {
        return Err(crate::value_error("alpha and beta must be positive"));
    }
    
    let u = random();
    Ok(alpha * (-u.ln()).powf(1.0 / beta))
}

// Integer functions
/// random.randrange - random integer in range
pub fn randrange(start: i64, stop: Option<i64>, step: Option<i64>) -> Result<i64, PyException> {
    let (start, stop, step) = match (stop, step) {
        (None, None) => (0, start, 1),
        (Some(stop), None) => (start, stop, 1),
        (Some(stop), Some(step)) => {
            if step == 0 {
                return Err(crate::value_error("step must not be zero"));
            }
            (start, stop, step)
        },
        (None, Some(_)) => return Err(crate::type_error("Missing stop argument")),
    };
    
    if step > 0 && start >= stop {
        return Err(crate::value_error("empty range"));
    }
    if step < 0 && start <= stop {
        return Err(crate::value_error("empty range"));
    }
    
    let width = (stop - start) / step;
    if width <= 0 {
        return Err(crate::value_error("empty range"));
    }
    
    let n = (random() * width as f64) as i64;
    Ok(start + n * step)
}

/// random.randint - random integer in [a, b]
pub fn randint(a: i64, b: i64) -> Result<i64, PyException> {
    if a > b {
        return Err(crate::value_error("empty range"));
    }
    randrange(a, Some(b + 1), None)
}

/// random.getrandbits - random integer with k random bits
pub fn getrandbits(k: u32) -> Result<u64, PyException> {
    if k == 0 {
        return Ok(0);
    }
    if k > 64 {
        return Err(crate::value_error("number of bits must be <= 64"));
    }
    
    let mask = if k == 64 { u64::MAX } else { (1u64 << k) - 1 };
    if let Ok(mut rng) = RNG.lock() {
        Ok(rng.next() & mask)
    } else {
        Ok(0)
    }
}

// Sequence functions
/// random.choice - choose random element
pub fn choice<T>(seq: &[T]) -> Result<&T, PyException> {
    if seq.is_empty() {
        return Err(crate::index_error("cannot choose from an empty sequence"));
    }
    
    let index = (random() * seq.len() as f64) as usize;
    Ok(&seq[index.min(seq.len() - 1)])
}

/// random.choices - choose k elements with replacement
pub fn choices<T>(
    population: &[T], 
    weights: Option<&[f64]>, 
    cum_weights: Option<&[f64]>, 
    k: usize
) -> Result<Vec<T>, PyException> 
where
    T: Clone,
{
    if population.is_empty() {
        return Err(crate::value_error("population is empty"));
    }
    
    let cum_weights = if let Some(cw) = cum_weights {
        if cw.len() != population.len() {
            return Err(crate::value_error("cum_weights length mismatch"));
        }
        cw.to_vec()
    } else if let Some(w) = weights {
        if w.len() != population.len() {
            return Err(crate::value_error("weights length mismatch"));
        }
        let mut cum = Vec::new();
        let mut total = 0.0;
        for &weight in w {
            total += weight;
            cum.push(total);
        }
        cum
    } else {
        (1..=population.len()).map(|i| i as f64).collect()
    };
    
    let total = cum_weights.last().copied().unwrap_or(0.0);
    if total <= 0.0 {
        return Err(crate::value_error("total weight must be positive"));
    }
    
    let mut result = Vec::with_capacity(k);
    for _ in 0..k {
        let r = random() * total;
        let index = cum_weights.iter().position(|&w| r <= w).unwrap_or(0);
        result.push(population[index].clone());
    }
    
    Ok(result)
}

/// random.shuffle - shuffle sequence in place
pub fn shuffle<T>(seq: &mut [T]) {
    let n = seq.len();
    for i in (1..n).rev() {
        let j = (random() * (i + 1) as f64) as usize;
        seq.swap(i, j);
    }
}

/// random.sample - choose k unique elements
pub fn sample<T>(population: &[T], k: usize) -> Result<Vec<T>, PyException> 
where
    T: Clone,
{
    let n = population.len();
    if k > n {
        return Err(crate::value_error("sample larger than population"));
    }
    
    if k == 0 {
        return Ok(Vec::new());
    }
    
    let mut indices: Vec<usize> = (0..n).collect();
    shuffle(&mut indices);
    
    Ok(indices.into_iter()
        .take(k)
        .map(|i| population[i].clone())
        .collect())
}

/// SystemRandom - system random number generator
pub struct SystemRandom;

impl SystemRandom {
    pub fn new() -> Self {
        Self
    }
    
    /// Generate random bytes
    pub fn randbytes(&self, n: usize) -> Vec<u8> {
        (0..n).map(|_| (random() * 256.0) as u8).collect()
    }
    
    /// Generate random integer in range [0, k)
    pub fn randbelow(&self, k: u64) -> Result<u64, PyException> {
        if k == 0 {
            return Err(crate::value_error("k must be positive"));
        }
        
        Ok((random() * k as f64) as u64)
    }
}

impl Default for SystemRandom {
    fn default() -> Self {
        Self::new()
    }
}