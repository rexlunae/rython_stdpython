use stdpython::*;

fn main() {
    // Demonstrate generic abs function
    println!("abs(-5i64) = {}", abs(-5i64));
    println!("abs(-3.14f64) = {}", abs(-3.14f64));
    
    // Demonstrate generic sum function
    let nums = vec![1, 2, 3, 4, 5];
    println!("sum([1,2,3,4,5]) = {}", sum(&nums[..]));
    
    let pylist = PyList::from_vec(vec![10, 20, 30]);
    println!("sum(PyList([10,20,30])) = {}", sum(&pylist));
    
    // Demonstrate generic type conversions
    println!("str(123) = '{}'", str(123i64));
    println!("str(true) = '{}'", str(true));
    println!("bool(42) = {}", bool(42i64));
    println!("bool('') = {}", bool(""));
    
    // Show that single function names match Python exactly
    println!("\n=== Python-style API ===");
    let my_string = PyStr::new("Hello World");
    println!("len('Hello World') = {}", len(&my_string));
    println!("bool('Hello World') = {}", bool(&my_string));
    
    let empty_list: PyList<i32> = PyList::new();
    println!("bool([]) = {}", bool(&empty_list));
    
    let numbers = vec![5, 1, 9, 3];
    println!("min([5,1,9,3]) = {:?}", min(&numbers));
    println!("max([5,1,9,3]) = {:?}", max(&numbers));
}