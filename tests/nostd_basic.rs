//! Basic test for nostd functionality

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::vec;

#[cfg(not(feature = "std"))]
use alloc::string::ToString;

#[test]
fn test_nostd_basic_functions() {
    use stdpython::*;
    
    // Test basic math functions
    assert_eq!(abs(-5i64), 5);
    assert_eq!(abs(-3.14f64), 3.14);
    
    // Test generic sum
    let nums = vec![1i64, 2, 3, 4, 5];
    assert_eq!(sum(&nums[..]), 15);
    
    // Test min/max
    assert_eq!(min(&nums), Some(1));
    assert_eq!(max(&nums), Some(5));
    
    // Test type conversions
    assert_eq!(bool(42i64), true);
    assert_eq!(bool(0i64), false);
    assert_eq!(str(123i64), "123");
    assert_eq!(str(true), "True");
    assert_eq!(str(false), "False");
}

#[test] 
fn test_nostd_collections() {
    use stdpython::*;
    
    // Test PyList
    let mut list = PyList::new();
    list.append(1);
    list.append(2);
    list.append(3);
    
    assert_eq!(list.len(), 3);
    assert_eq!(bool(&list), true);
    
    // Test PyStr
    let s = PyStr::new("hello");
    assert_eq!(len(&s), 5);
    assert_eq!(bool(&s), true);
    
    let empty_str = PyStr::new("");
    assert_eq!(bool(&empty_str), false);
    
    // Test PyDictionary
    let mut dict = PyDictionary::new();
    dict.set("key".to_string(), 42);
    assert_eq!(dict.len(), 1);
    assert_eq!(bool(&dict), true);
}

#[cfg(not(feature = "std"))]
#[test]
fn test_nostd_print_alternative() {
    use stdpython::*;
    
    // In nostd, we have print_to_string instead of print
    let output = print_to_string("Hello, nostd world!");
    assert_eq!(output, "Hello, nostd world!");
    
    let objects = vec!["a", "b", "c"];
    let output = print_args_to_string(&objects, ", ", "\n");
    assert_eq!(output, "a, b, c\n");
}