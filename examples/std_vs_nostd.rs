// Example showing the differences between std and nostd modes

#[cfg(feature = "std")]
fn main() {
    use stdpython::*;
    
    println!("=== STD MODE EXAMPLE ===");
    
    // Standard I/O operations available
    print("Hello from std mode!");
    
    // File operations available
    match open("example.txt", Some("w")) {
        Ok(mut file) => {
            let _ = file.write("Hello, file system!");
            let _ = file.close();
            println!("File operations work in std mode");
        },
        Err(e) => println!("File error: {}", e),
    }
    
    // All core functions work the same
    let nums = vec![1, 2, 3, 4, 5];
    println!("sum([1,2,3,4,5]) = {}", sum(&nums[..]));
    println!("abs(-42) = {}", abs(-42i64));
    println!("str(true) = '{}'", str(true));
    
    // Collections work
    let mut list = PyList::from_vec(vec![10, 20, 30]);
    list.append(40);
    println!("PyList after append: len = {}", len(&list));
    
    println!("=== PyO3 and python-mod integration available ===");
}

#[cfg(not(feature = "std"))]
fn main() {
    extern crate alloc;
    use alloc::vec;
    use stdpython::*;
    
    println!("=== NO-STD MODE EXAMPLE ===");
    
    // No direct printing, but string generation works
    let output = print_to_string("Hello from nostd mode!");
    println!("Generated output: {}", output);
    
    // File operations not available in nostd mode
    // This would cause a compile error:
    // let file = open("test.txt", Some("r"));  // ERROR: not available
    
    // All core functions work identically
    let nums = vec![1, 2, 3, 4, 5];
    let total = sum(&nums[..]);
    println!("sum([1,2,3,4,5]) = {}", total);
    println!("abs(-42) = {}", abs(-42i64));
    println!("str(true) = '{}'", str(true));
    
    // Collections work identically
    let mut list = PyList::from_vec(vec![10, 20, 30]);
    list.append(40);
    println!("PyList after append: len = {}", len(&list));
    
    // Generic traits work the same
    assert_eq!(bool(&list), true);
    assert_eq!(bool(0i64), false);
    
    println!("=== Core Python runtime works without std! ===");
}