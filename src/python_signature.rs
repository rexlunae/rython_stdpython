/// Custom Python signature system that allows generic parameters
/// 
/// This module provides macros and functionality to create Python-compatible
/// function signatures while preserving Rust's generic type system.
/// 
/// Unlike PyO3's #[pyo3(signature = (...))] which doesn't allow generics,
/// our system generates concrete wrapper functions that handle Python
/// calling conventions while calling the original generic functions.

/// Macro to add Python signature support to generic functions
/// 
/// This generates:
/// 1. The original generic function (unchanged)
/// 2. A concrete `function_name_py` wrapper that handles Python calling conventions
/// 3. A simple `function_name_wrapper` for generated Rust code
#[macro_export]
macro_rules! python_signature {
    // Single parameter, no defaults
    ($vis:vis fn $name:ident<$($generic:ident $(: $bound:path)?),*>($param:ident: $param_type:ty) -> $ret:ty $body:block) => {
        // Original generic function (unchanged)
        $vis fn $name<$($generic $(: $bound)?),*>($param: $param_type) -> $ret $body
        
        // Generate Python-compatible wrapper with concrete types
        paste::paste! {
            /// Python-compatible wrapper function
            #[cfg(feature = "std")]
            pub fn [<$name _py>]($param: f64) -> f64 {
                $name($param)
            }
            
            /// Simple wrapper for generated Rust code
            pub fn [<$name _wrapper>]($param: f64) -> f64 {
                $name($param)
            }
        }
    };
    
    // Two parameters, no defaults
    ($vis:vis fn $name:ident<$($generic:ident $(: $bound:path)?),*>($param1:ident: $param1_type:ty, $param2:ident: $param2_type:ty) -> $ret:ty $body:block) => {
        // Original generic function (unchanged)
        $vis fn $name<$($generic $(: $bound)?),*>($param1: $param1_type, $param2: $param2_type) -> $ret $body
        
        // Generate Python-compatible wrapper with concrete types
        paste::paste! {
            /// Python-compatible wrapper function
            #[cfg(feature = "std")]
            pub fn [<$name _py>]($param1: f64, $param2: f64) -> f64 {
                $name($param1, $param2)
            }
            
            /// Simple wrapper for generated Rust code
            pub fn [<$name _wrapper>]($param1: f64, $param2: f64) -> f64 {
                $name($param1, $param2)
            }
        }
    };
    
    // Single parameter with optional second parameter
    ($vis:vis fn $name:ident<$($generic:ident $(: $bound:path)?),*>($param1:ident: $param1_type:ty, $param2:ident: Option<$param2_type:ty>) -> $ret:ty $body:block) => {
        // Original generic function (unchanged)
        $vis fn $name<$($generic $(: $bound)?),*>($param1: $param1_type, $param2: Option<$param2_type>) -> $ret $body
        
        // Generate Python-compatible wrapper with concrete types
        paste::paste! {
            /// Python-compatible wrapper function
            #[cfg(feature = "std")]
            pub fn [<$name _py>]($param1: f64, $param2: Option<f64>) -> f64 {
                $name($param1, $param2)
            }
            
            /// Simple wrapper for generated Rust code
            pub fn [<$name _wrapper>]($param1: f64, $param2: Option<f64>) -> f64 {
                $name($param1, $param2)
            }
        }
    };
}

/// More advanced macro that can handle complex signatures with defaults
/// 
/// This macro generates wrapper functions that solve the parameter inference problem
/// by creating concrete-typed versions of generic functions.
#[macro_export]
macro_rules! python_function {
    // With where clause - use brackets to avoid parsing ambiguity
    (
        $(#[$attr:meta])*
        $vis:vis fn $name:ident<$($generic:ident $(: $bound:path)?),*>($($param:ident: $param_type:ty),*) -> $ret:ty
        where [$($where_clause:tt)+]
        [signature: ($($sig_param:ident $(=$default:expr)?),*)]
        [concrete_types: ($($concrete_param:ty),*) -> $concrete_ret:ty]
        $body:block
    ) => {
        // Original generic function with all attributes preserved
        $(#[$attr])*
        $vis fn $name<$($generic $(: $bound)?),*>($($param: $param_type),*) -> $ret
        where $($where_clause)+
        $body
        
        paste::paste! {
            /// Python-compatible wrapper function (concrete types)
            #[cfg(feature = "std")]
            pub fn [<$name _py>]($($param: $concrete_param),*) -> $concrete_ret {
                // Call the original generic function
                $name($($param),*)
            }
            
            /// Simple wrapper for generated Rust code
            pub fn [<$name _wrapper>]($($param: $concrete_param),*) -> $concrete_ret {
                $name($($param),*)
            }
        }
    };
    
    // Without where clause - use brackets to avoid parsing ambiguity
    (
        $(#[$attr:meta])*
        $vis:vis fn $name:ident<$($generic:ident $(: $bound:path)?),*>($($param:ident: $param_type:ty),*) -> $ret:ty
        [signature: ($($sig_param:ident $(=$default:expr)?),*)]
        [concrete_types: ($($concrete_param:ty),*) -> $concrete_ret:ty]
        $body:block
    ) => {
        // Original generic function with all attributes preserved
        $(#[$attr])*
        $vis fn $name<$($generic $(: $bound)?),*>($($param: $param_type),*) -> $ret
        $body
        
        paste::paste! {
            /// Python-compatible wrapper function (concrete types)
            #[cfg(feature = "std")]
            pub fn [<$name _py>]($($param: $concrete_param),*) -> $concrete_ret {
                // Call the original generic function
                $name($($param),*)
            }
            
            /// Simple wrapper for generated Rust code
            pub fn [<$name _wrapper>]($($param: $concrete_param),*) -> $concrete_ret {
                $name($($param),*)
            }
        }
    };
    
    // For non-generic functions
    (
        $(#[$attr:meta])*
        $vis:vis fn $name:ident($($param:ident: $param_type:ty),*) -> $ret:ty
        [signature: ($($sig_param:ident $(=$default:expr)?),*)]
        [concrete_types: ($($concrete_param:ty),*) -> $concrete_ret:ty]
        $body:block
    ) => {
        // Original function with all attributes preserved
        $(#[$attr])*
        $vis fn $name($($param: $param_type),*) -> $ret
        $body
        
        paste::paste! {
            /// Python-compatible wrapper function (already concrete types)
            #[cfg(feature = "std")]
            pub fn [<$name _py>]($($param: $concrete_param),*) -> $concrete_ret {
                // Call the original function
                $name($($param),*)
            }
            
            /// Simple wrapper for generated Rust code
            pub fn [<$name _wrapper>]($($param: $concrete_param),*) -> $concrete_ret {
                $name($($param),*)
            }
        }
    };
}

/// Trait to register Python-compatible functions for a module
pub trait PythonModule {
    /// Register all Python-compatible functions in this module
    fn register_python_functions() -> Vec<&'static str>;
    
    /// Get function by name (for dynamic dispatch)
    fn get_function(name: &str) -> Option<fn()>;
}

/// Macro to implement PythonModule for stdlib modules
/// 
/// This generates registry functions for modules containing Python-compatible functions.
#[macro_export]
macro_rules! impl_python_module {
    ($module:ident, [$($func:ident),* $(,)?]) => {
        paste::paste! {
            impl $crate::python_signature::PythonModule for super::$module {
                fn register_python_functions() -> Vec<&'static str> {
                    vec![$(stringify!([<$func _py>])),*]
                }
                
                fn get_function(name: &str) -> Option<fn()> {
                    match name {
                        $(stringify!([<$func _py>]) => Some(|| {}),)* // Placeholder
                        _ => None,
                    }
                }
            }
        }
    };
}

/// Generate a registry of all Python functions across all modules
pub struct PythonFunctionRegistry {
    functions: std::collections::HashMap<String, String>,
}

impl PythonFunctionRegistry {
    pub fn new() -> Self {
        Self {
            functions: std::collections::HashMap::new(),
        }
    }
    
    /// Register a function with its module and signature
    pub fn register(&mut self, module: &str, function: &str, signature: &str) {
        let full_name = format!("{}.{}", module, function);
        self.functions.insert(full_name, signature.to_string());
    }
    
    /// Get all registered functions
    pub fn get_all_functions(&self) -> &std::collections::HashMap<String, String> {
        &self.functions
    }
    
    /// Check if a function is registered
    pub fn has_function(&self, module: &str, function: &str) -> bool {
        let full_name = format!("{}.{}", module, function);
        self.functions.contains_key(&full_name)
    }
}

use std::sync::Mutex;

/// Global registry instance
static PYTHON_FUNCTION_REGISTRY: std::sync::LazyLock<Mutex<PythonFunctionRegistry>> = 
    std::sync::LazyLock::new(|| Mutex::new(PythonFunctionRegistry::new()));

/// Get the global Python function registry
pub fn get_registry() -> std::sync::MutexGuard<'static, PythonFunctionRegistry> {
    PYTHON_FUNCTION_REGISTRY.lock().unwrap()
}

/// Convenience macro to register a module's Python functions
#[macro_export]
macro_rules! register_python_module {
    ($module:expr, [$(($func:expr, $sig:expr)),* $(,)?]) => {{
        let mut registry = $crate::python_signature::get_registry();
        $(
            registry.register($module, $func, $sig);
        )*
    }};
}