# rython_stdpython

The default Python runtime library for the Rython compiler ecosystem. This crate provides a comprehensive, Python-compatible standard library implementation in Rust that serves as the runtime foundation for Python code compiled to Rust via the Rython toolchain.

## Overview

`rython_stdpython` is a complete Python runtime environment written in Rust that enables compiled Python code to access all Python built-ins, types, and standard operations without requiring any imports. It provides both `std` and `no_std` variants, making it suitable for everything from desktop applications to embedded systems.

## Key Features

- **Complete Python Built-ins**: 40+ built-in functions implemented (`print`, `len`, `range`, `enumerate`, `zip`, `min`, `max`, `sum`, `all`, `any`, etc.)
- **Full Type System**: Python-compatible implementations of `str`, `list`, `dict`, `tuple`, `set`, `int`, `float`, `bool` with all their methods
- **Generic Trait System**: Flexible, zero-cost abstractions that work with any type implementing the appropriate trait
- **Exception Handling**: Complete Python exception hierarchy (`ValueError`, `TypeError`, `IndexError`, etc.)
- **Both std and no_std**: Supports standard library environments and embedded/no_std targets
- **Memory Safe**: All operations maintain Rust's memory safety guarantees
- **Performance Optimized**: Native Rust implementations provide superior performance to interpreted Python

## Architecture

This library uses a generic trait-based design that mirrors Python's built-in behavior:

### Conversion Traits
- `PyAbs`: Generic absolute value (`abs(-5i64)`, `abs(-3.14f64)`)
- `PyBool`: Generic boolean conversion (`bool(42)`, `bool("")`)
- `PyInt`: Generic integer conversion (`int("123")`, `int(3.14)`)
- `PyFloat`: Generic float conversion (`float("3.14")`, `float(42)`)
- `PyToString`: Generic string conversion (`str(123)`, `str(true)`)
- `PySum`: Generic summation (`sum(&[1,2,3])`, `sum(&pylist)`)

### Runtime Traits
- `Len`: Universal length calculation
- `Truthy`: Python-style truthiness evaluation

## What's Implemented

### Python Built-in Functions (40+)
✅ **Math/Logic**: `abs()`, `min()`, `max()`, `sum()`, `all()`, `any()`  
✅ **Iteration**: `enumerate()`, `zip()`, `range()`, `len()`  
✅ **Type Conversion**: `bool()`, `int()`, `float()`, `str()`, `list()`, `dict()`, `tuple()`, `set()`  
✅ **Object Introspection**: `type()`, `isinstance()`, `hasattr()`, `getattr()`, `setattr()`, `delattr()`, `id()`, `hash()`  
✅ **Character/Unicode**: `ord()`, `chr()`  
✅ **I/O**: `print()` with full parameter support (std mode only)  

### Python Built-in Types with Complete Method Sets

#### PyStr (String Type)
✅ **Core Methods**: `split()`, `join()`, `strip()`, `lower()`, `upper()`, `replace()`  
✅ **Search Methods**: `find()`, `count()`, `startswith()`, `endswith()`  
✅ **Formatting**: `format()` (basic implementation)  

#### PyList (List Type)  
✅ **Modification**: `append()`, `extend()`, `insert()`, `remove()`, `pop()`, `clear()`  
✅ **Search/Sort**: `index()`, `count()`, `sort()`, `reverse()`  
✅ **Utilities**: `copy()`, indexing with `get()`/`set()`  

#### PyDictionary (Dictionary Type)
✅ **Access**: `get()`, `get_or_default()`, `contains_key()`  
✅ **Modification**: `set()`, `pop()`, `clear()`, `update()`  
✅ **Iteration**: `keys()`, `values()`, `items()`  

#### PyTuple (Tuple Type)
✅ **Immutable sequence**: Index access, slicing support  

#### PySet (Set Type)
✅ **Modification**: `add()`, `remove()`, `discard()`, `clear()`  
✅ **Set Operations**: `union()`, `intersection()`, `difference()`  
✅ **Membership**: `contains()`  

### Exception System
✅ **Complete Exception Hierarchy**: `PyException`, `ValueError`, `TypeError`, `IndexError`, `KeyError`, `AttributeError`, `NameError`, `ZeroDivisionError`, `OverflowError`, `RuntimeError`

## What's Not Implemented

❌ **Advanced Python Features**: Decorators, metaclasses, generators, async/await  
❌ **Complex Built-ins**: `exec()`, `eval()`, `compile()`, `globals()`, `locals()`  
❌ **File I/O** (no_std mode): File operations, directory handling  
❌ **Networking**: Socket operations, HTTP clients  
❌ **Threading**: Thread management, locks, synchronization  
❌ **Regular Expressions**: `re` module functionality  
❌ **Date/Time**: `datetime`, `time` module functionality  
❌ **OS Interface**: `os`, `sys` module functionality  
❌ **Import System**: Dynamic module loading  

## Usage

### Standard Library Mode (default)
```toml
[dependencies]
rython_stdpython = "1.0"
```

### No-Std Mode (embedded systems)
```toml
[dependencies]
rython_stdpython = { version = "1.0", default-features = false, features = ["nostd"] }
```

### Example Usage

```rust
use rython_stdpython::*;

fn main() {
    // Generic functions work with any compatible type
    let nums = vec![1, 2, 3, 4, 5];
    let total = sum(&nums[..]);  // Generic summation
    
    // Python-like type conversions
    let s = str(total);  // "15"
    let b = bool(&nums); // true (non-empty)
    
    // Full Python collections
    let mut list = PyList::from_vec(nums);
    list.append(6);
    
    // All Python built-ins available
    print(&format!("Total: {}", total));
    assert_eq!(len(&list), 6);
}
```

## Integration with Rython

This crate serves as the runtime foundation for the entire Rython ecosystem:

- **rythonc**: The Python-to-Rust compiler generates code that calls these runtime functions
- **python-ast-rs**: AST code generation targets these built-in implementations  
- **python-mod-rs**: Embedded Python modules depend on these built-ins

When Python code is compiled to Rust, it naturally maps to function calls in this library:

```python
# Python code
my_list = [1, 2, 3]
total = sum(my_list)
print(str(total))
```

```rust
// Generated Rust code
let mut my_list = PyList::from_vec(vec![1, 2, 3]);
let total = sum(&my_list);  // Uses PySum trait
print(str(total));          // Uses PyToString trait
```

## Building and Testing

```bash
# Standard library version
cargo build
cargo test

# No-std version  
cargo build --no-default-features --features nostd
cargo test --no-default-features --features nostd

# Run specific test modules
cargo test test_python_functions  # Built-in function tests
cargo test test_pystr             # String type tests
cargo test test_pylist            # List type tests
```

## License

This project is part of the Rython compiler ecosystem.
