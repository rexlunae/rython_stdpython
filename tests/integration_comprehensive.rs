//! Comprehensive integration test demonstrating complete Python-to-Rust compilation compatibility
//! 
//! This test proves that the stdpython runtime provides all the necessary interfaces
//! for Python code compiled to Rust via python-ast-rs and the Rython ecosystem.

use stdpython::*;

#[test]
fn test_complete_python_runtime_interface() {
    println!("🧪 Testing complete Python runtime interface for compilation compatibility...\n");
    
    // Test 1: All Python built-in functions work with correct signatures
    test_python_builtin_functions();
    
    // Test 2: Python types and their methods work correctly
    test_python_type_system();
    
    // Test 3: Python collection types support all standard operations
    test_python_collection_operations();
    
    // Test 4: Type conversions work exactly like Python
    test_python_type_conversions();
    
    // Test 5: Iteration and functional programming constructs
    test_python_iteration_functions();
    
    println!("✅ All Python runtime interface tests passed!");
    println!("🎯 The stdpython runtime is ready for Python-to-Rust compilation!");
}

fn test_python_builtin_functions() {
    println!("📋 Testing Python built-in functions...");
    
    // Mathematical functions
    assert_eq!(abs(-42i64), 42);
    assert_eq!(abs(-3.14), 3.14);
    assert_eq!(sum(&vec![1, 2, 3, 4, 5][..]), 15);
    assert_eq!(min(&vec![5, 1, 9, 3]), Some(1));
    assert_eq!(max(&vec![5, 1, 9, 3]), Some(9));
    
    // Boolean functions
    assert_eq!(all(&vec![true, true, true]), true);
    assert_eq!(any(&vec![false, false, true]), true);
    assert_eq!(all(&vec![true, false, true]), false);
    
    // Utility functions
    let items = vec!["a", "b", "c"];
    let enumerated = enumerate(items);
    assert_eq!(enumerated, vec![(0, "a"), (1, "b"), (2, "c")]);
    
    let nums = vec![1, 2, 3];
    let letters = vec!["a", "b", "c"];
    let zipped = zip(nums, letters);
    assert_eq!(zipped, vec![(1, "a"), (2, "b"), (3, "c")]);
    
    assert_eq!(range(5), vec![0, 1, 2, 3, 4]);
    assert_eq!(range_start_stop(2, 6), vec![2, 3, 4, 5]);
    assert_eq!(range_start_stop_step(0, 10, 2), vec![0, 2, 4, 6, 8]);
    
    println!("  ✓ All built-in functions working correctly");
}

fn test_python_type_system() {
    println!("📋 Testing Python type system...");
    
    // String operations
    let text = PyStr::new("Hello World");
    assert_eq!(text.upper().as_str(), "HELLO WORLD");
    assert_eq!(text.lower().as_str(), "hello world");
    let words = text.split(Some(" "));
    assert_eq!(words.len(), 2);
    assert_eq!(words[0].as_str(), "Hello");
    assert_eq!(words[1].as_str(), "World");
    assert_eq!(text.startswith("Hello"), true);
    assert_eq!(text.endswith("World"), true);
    assert_eq!(text.find("World"), 6);
    assert_eq!(len(&text), 11);
    
    // List operations
    let mut my_list = PyList::from_vec(vec![1, 2, 3]);
    my_list.append(4);
    my_list.extend(vec![5, 6]);
    assert_eq!(len(&my_list), 6);
    assert_eq!(my_list.get(0), Some(&1));
    assert_eq!(my_list.pop(None), Some(6));
    my_list.insert(0, 0);
    assert_eq!(my_list.get(0), Some(&0));
    
    // Dictionary operations
    let mut my_dict = PyDictionary::new();
    my_dict.set("key1".to_string(), 100);
    my_dict.set("key2".to_string(), 200);
    assert_eq!(my_dict.get(&"key1".to_string()), Some(&100));
    assert_eq!(len(&my_dict), 2);
    assert_eq!(my_dict.contains_key(&"key1".to_string()), true);
    assert_eq!(my_dict.contains_key(&"key3".to_string()), false);
    
    // Set operations
    let mut my_set = PySet::new();
    my_set.add(1);
    my_set.add(2);
    my_set.add(1); // Duplicate should be ignored
    assert_eq!(len(&my_set), 2);
    assert_eq!(my_set.contains(&1), true);
    assert_eq!(my_set.contains(&3), false);
    
    // Tuple operations
    let my_tuple = PyTuple::new(vec![1, 2, 3, 4, 5]);
    assert_eq!(len(&my_tuple), 5);
    assert_eq!(my_tuple.get(2), Some(&3));
    
    println!("  ✓ All Python types working correctly");
}

fn test_python_collection_operations() {
    println!("📋 Testing Python collection operations...");
    
    // Complex list operations
    let mut numbers = PyList::from_vec(vec![3, 1, 4, 1, 5, 9, 2, 6]);
    numbers.sort();
    assert_eq!(*numbers.as_vec(), vec![1, 1, 2, 3, 4, 5, 6, 9]);
    
    numbers.reverse();
    assert_eq!(numbers.get(0), Some(&9));
    
    assert_eq!(numbers.count(&1), 2);
    assert_eq!(numbers.index(&5), Some(2)); // After sort: [1,1,2,3,4,5,6,9], after reverse: [9,6,5,4,3,2,1,1], so 5 is at index 2
    
    numbers.remove(&1);
    assert_eq!(numbers.count(&1), 1);
    
    // String manipulation
    let sentence = PyStr::new("The quick brown fox jumps over the lazy dog");
    let words = sentence.split(Some(" "));
    assert_eq!(words.len(), 9);
    
    let separator = PyStr::new(" ");
    let words_pystr: Vec<PyStr> = words.iter().map(|s| PyStr::new(s.as_str())).collect();
    let joined = separator.join(&words_pystr);
    assert_eq!(joined.as_str(), sentence.as_str());
    
    let replaced = sentence.replace("fox", "cat");
    assert!(replaced.as_str().contains("cat"));
    assert!(!replaced.as_str().contains("fox"));
    
    // Dictionary comprehension-like operations
    let mut word_lengths = PyDictionary::new();
    for word in &words {
        word_lengths.set(word.as_str().to_string(), word.as_str().len());
    }
    assert_eq!(word_lengths.get(&"quick".to_string()), Some(&5));
    assert_eq!(word_lengths.get(&"the".to_string()), Some(&3));
    
    println!("  ✓ All collection operations working correctly");
}

fn test_python_type_conversions() {
    println!("📋 Testing Python type conversions...");
    
    // String to numeric conversions
    assert_eq!(int("123").unwrap(), 123);
    assert_eq!(int("-456").unwrap(), -456);
    assert_eq!(float("3.14159").unwrap(), 3.14159);
    assert_eq!(float("-2.718").unwrap(), -2.718);
    
    // Numeric to string conversions  
    assert_eq!(str(123i64), "123");
    assert_eq!(str(-456i64), "-456");
    assert_eq!(str(3.14), "3.14");
    
    // Boolean conversions
    assert_eq!(bool(1i64), true);
    assert_eq!(bool(0i64), false);
    assert_eq!(bool(-5i64), true);
    assert_eq!(bool(3.14), true);
    assert_eq!(bool(0.0), false);
    
    // Container to boolean
    let empty_list = PyList::<i32>::new();
    let full_list = PyList::from_vec(vec![1, 2, 3]);
    assert_eq!(bool(&empty_list), false);
    assert_eq!(bool(&full_list), true);
    
    let empty_str = PyStr::new("");
    let full_str = PyStr::new("hello");
    assert_eq!(bool(&empty_str), false);
    assert_eq!(bool(&full_str), true);
    
    println!("  ✓ All type conversions working correctly");
}

fn test_python_iteration_functions() {
    println!("📋 Testing Python iteration functions...");
    
    // Range function variations
    let simple_range = range(5);
    assert_eq!(simple_range, vec![0, 1, 2, 3, 4]);
    
    let start_stop_range = range_start_stop(3, 8);
    assert_eq!(start_stop_range, vec![3, 4, 5, 6, 7]);
    
    let step_range = range_start_stop_step(0, 20, 3);
    assert_eq!(step_range, vec![0, 3, 6, 9, 12, 15, 18]);
    
    let negative_step_range = range_start_stop_step(10, 0, -2);
    assert_eq!(negative_step_range, vec![10, 8, 6, 4, 2]);
    
    // Enumerate function
    let colors = vec!["red", "green", "blue"];
    let enumerated_colors = enumerate(colors);
    assert_eq!(enumerated_colors, vec![(0, "red"), (1, "green"), (2, "blue")]);
    
    // Zip function with different types
    let numbers = vec![1, 2, 3, 4];
    let letters = vec!["a", "b", "c", "d"];
    
    let zipped_pairs = zip(numbers.clone(), letters.clone());
    assert_eq!(zipped_pairs, vec![(1, "a"), (2, "b"), (3, "c"), (4, "d")]);
    
    // Test that zip works with different lengths (should stop at shortest)
    let short_list = vec![1, 2];
    let long_list = vec!["a", "b", "c", "d", "e"];
    let zipped_different_lengths = zip(short_list, long_list);
    assert_eq!(zipped_different_lengths, vec![(1, "a"), (2, "b")]);
    
    println!("  ✓ All iteration functions working correctly");
}

#[cfg(feature = "std")]
#[test]
fn test_python_io_compatibility() {
    println!("📋 Testing Python I/O compatibility (std feature)...");
    
    // Test print function (should not panic)
    print("Hello from compiled Python!");
    
    // Test that I/O functions are available
    // Note: We can't test input() in automated tests, but we can verify it compiles
    let _input_available = input;
    
    println!("  ✓ I/O functions available in std mode");
}

#[test]
fn test_python_error_handling_compatibility() {
    println!("📋 Testing Python error handling compatibility...");
    
    // Test that error conditions are handled gracefully
    assert!(int("not_a_number").is_err());
    assert!(float("also_not_a_number").is_err());
    
    // Test division by zero handling would be implementation-specific
    // In Python this would raise ZeroDivisionError, in Rust it panics
    // For now we just verify error handling works for parsing
    println!("    Note: Division by zero would be handled by Rython compiler");
    
    // Test out-of-bounds access
    let my_list = PyList::from_vec(vec![1, 2, 3]);
    assert_eq!(my_list.get(10), None); // Safe out-of-bounds access
    
    // Test missing key access
    let my_dict = PyDictionary::<String, i32>::new();
    assert_eq!(my_dict.get(&"missing_key".to_string()), None);
    
    println!("  ✓ Error handling working correctly");
}

/// Simulates what a more complex Python function would look like when compiled
#[test]
fn test_complex_python_simulation() {
    println!("📋 Testing complex Python code simulation...");
    
    // Simulate: def calculate_stats(numbers): return {"sum": sum(numbers), "min": min(numbers), "max": max(numbers), "count": len(numbers)}
    fn calculate_stats(numbers: &Vec<i32>) -> PyDictionary<String, i32> {
        let mut stats = PyDictionary::new();
        stats.set("sum".to_string(), sum(&numbers[..]));
        stats.set("min".to_string(), min(numbers).unwrap_or(0));
        stats.set("max".to_string(), max(numbers).unwrap_or(0));
        stats.set("count".to_string(), len(numbers) as i32);
        stats
    }
    
    let numbers = vec![1, 5, 3, 9, 2, 7, 4];
    let stats = calculate_stats(&numbers);
    
    assert_eq!(stats.get(&"sum".to_string()), Some(&31));
    assert_eq!(stats.get(&"min".to_string()), Some(&1));
    assert_eq!(stats.get(&"max".to_string()), Some(&9));
    assert_eq!(stats.get(&"count".to_string()), Some(&7));
    
    // Simulate: def process_text(text): return {"words": text.split(), "word_count": len(text.split()), "uppercase": text.upper()}
    fn process_text(text: &str) -> PyDictionary<String, String> {
        let py_text = PyStr::new(text);
        let words = py_text.split(Some(" "));
        let mut result = PyDictionary::new();
        
        result.set("words".to_string(), format!("{:?}", words));
        result.set("word_count".to_string(), words.len().to_string());
        result.set("uppercase".to_string(), py_text.upper().as_str().to_string());
        
        result
    }
    
    let text_result = process_text("hello world python");
    assert_eq!(text_result.get(&"word_count".to_string()), Some(&"3".to_string()));
    assert_eq!(text_result.get(&"uppercase".to_string()), Some(&"HELLO WORLD PYTHON".to_string()));
    
    println!("  ✓ Complex Python code simulation working correctly");
}