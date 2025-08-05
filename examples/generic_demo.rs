use stdpython::*;

fn main() {
    println!("=== Generic rython_stdpython Demo ===");

    // Test generic sys functions
    let exe_string: String = sys::get_executable();
    let exe_str: &str = &exe_string;
    println!("Executable (String): {}", exe_string);
    println!("Executable (&str): {}", exe_str);

    // Test generic argv
    let argv_vec: Vec<String> = sys::get_argv();
    println!("Args (Vec<String>): {:?}", argv_vec);

    // Test generic os.path functions - accepting different string types
    let path1 = "/usr/bin/python";
    let path2 = String::from("/home/user/script.py");
    
    let dirname1: String = os::path::dirname(path1);
    let dirname2: String = os::path::dirname(&path2);
    println!("Dirname of '{}': {}", path1, dirname1);
    println!("Dirname of '{}': {}", path2, dirname2);

    // Test generic path operations with different collections
    let components1 = vec!["home", "user", "docs"];
    let components2 = vec![String::from("usr"), String::from("local"), String::from("bin")];
    
    let joined1: String = os::path::join(components1);
    let joined2: String = os::path::join(components2);
    println!("Joined path 1: {}", joined1);
    println!("Joined path 2: {}", joined2);

    // Test generic subprocess with different argument types
    let args1 = vec!["echo", "hello"];
    let args2 = vec![String::from("echo"), String::from("world")];
    
    match subprocess::run_mixed_args(args1, None::<&str>) {
        Ok(result) => println!("Command 1 exit code: {}", result.returncode),
        Err(e) => println!("Command 1 failed: {}", e),
    }
    
    match subprocess::run_mixed_args(args2, None::<&str>) {
        Ok(result) => println!("Command 2 exit code: {}", result.returncode),
        Err(e) => println!("Command 2 failed: {}", e),
    }

    println!("=== Demo Complete ===");
}