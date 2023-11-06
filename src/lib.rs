//#![feature(c_variadic)]
//use std::collections::HashMap;
use std::fmt::Display;

pub use pyo3::{PyAny, types::PyDict, PyObject};
pub use python_mod::python_module_nostd;

python_module_nostd!{lib
    use pyo3::{
        PyAny, PyObject,
    };
}

pub use lib::*;

/// Python-equivalent print() function.
pub fn print<S: Display>(s: S) {
    println!("{}", s);
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
    }
}
