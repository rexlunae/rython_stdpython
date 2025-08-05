# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`rython_stdpython` is a comprehensive standard library crate for the Rython ecosystem, providing complete Python-compatible implementations of all built-in functions, types, and methods. This library serves as the runtime foundation that enables Python code compiled to Rust via `python-ast-rs` to access all Python built-ins without any imports.

## Architecture

This is a comprehensive Rust library that provides a complete Python runtime environment:

- **Complete Python Built-ins**: All functions available in Python without imports (print, len, range, enumerate, zip, min, max, sum, all, any, etc.)
- **Full Type System**: Python-compatible implementations of str, list, dict, tuple, set, int, float, bool with all their methods
- **Exception Handling**: Python exception types (ValueError, TypeError, IndexError, etc.)
- **Runtime Traits**: Truthy evaluation, length calculation, and other Python semantics
- **PyO3 Integration**: Seamless interoperability with PyO3 for advanced Python integration

### Key Components

- **`src/lib.rs`**: Complete Rust runtime library with:
  - All Python built-in functions (40+ functions implemented)
  - Python-style types with full method implementations (PyStr, PyList, PyDictionary, PyTuple, PySet)
  - Exception types and error handling
  - Runtime traits (Len, Truthy) for Python semantics
  - Comprehensive test suite (5 test modules covering all functionality)

- **`src/lib.py`**: Python-side interface definitions with:
  - Complete docstrings for all built-in functions
  - Proper function signatures matching Python's built-ins
  - Base Object class with Python-style methods
  - Full API compatibility documentation

- **Dependencies**: 
  - `pyo3` (v0.25) for Python-Rust integration
  - `python-mod` (local dependency) for Python module compilation

## Development Commands

### Building
```bash
# Standard library version (default)
cargo build          # Debug build with std
cargo build --release # Release build with std
cargo check          # Fast compilation check

# No-std version for embedded systems
cargo build --no-default-features --features nostd
cargo check --no-default-features --features nostd
```

### Testing
```bash
# Test standard library version
cargo test --features std    # Run comprehensive test suite (6 test modules)
cargo test -- --nocapture   # Run tests with output
cargo test test_pystr        # Run specific string tests
cargo test test_pylist       # Run specific list tests
cargo test test_pydict       # Run specific dictionary tests
cargo test test_pyset        # Run specific set tests
cargo test test_python_functions  # Run built-in function tests

# Test no-std version
cargo test --no-default-features --features nostd
cargo test --no-default-features --features nostd nostd_basic
```

## Feature Configuration

### Standard Library (default)
```toml
[dependencies]
stdpython = "1.0"  # Uses std by default
```

### No Standard Library (nostd)
```toml
[dependencies]
stdpython = { version = "1.0", default-features = false, features = ["nostd"] }
```

### Available Features
- **`std` (default)**: Full standard library support with I/O operations, file handling
- **`nostd`**: No-std compatible version using `hashbrown` and `heapless` for embedded systems

## Implemented Functionality

### Python Built-in Functions (40+ implemented)
- **Math/Logic**: `abs()`, `min()`, `max()`, `sum()`, `all()`, `any()`
  - Uses generic traits for type flexibility (PyAbs, PySum)
  - Single function names match Python exactly (no `abs_int`, `sum_float` variants)
- **Iteration**: `enumerate()`, `zip()`, `range()`, `len()`
- **Type Conversion**: `bool()`, `int()`, `float()`, `str()`, `list()`, `dict()`, `tuple()`, `set()`
  - Generic implementations using traits (PyBool, PyInt, PyFloat, PyToString)
  - Work with any type that implements the appropriate trait
- **Object Introspection**: `type()`, `isinstance()`, `hasattr()`, `getattr()`, `setattr()`, `delattr()`, `id()`, `hash()`
- **Character/Unicode**: `ord()`, `chr()`
- **I/O**: `print()` with full parameter support

### Generic Type System Design

The library uses Rust traits to provide generic, Python-like behavior:

#### Conversion Traits
- **`PyAbs`**: Generic absolute value (`abs(-5i64)`, `abs(-3.14f64)`)
- **`PyBool`**: Generic boolean conversion (`bool(42)`, `bool("")`)
- **`PyInt`**: Generic integer conversion (`int("123")`, `int(3.14)`)
- **`PyFloat`**: Generic float conversion (`float("3.14")`, `float(42)`)
- **`PyToString`**: Generic string conversion (`str(123)`, `str(true)`)
- **`PySum`**: Generic summation (`sum(&[1,2,3])`, `sum(&pylist)`)

#### Runtime Traits
- **`Len`**: Universal length calculation
- **`Truthy`**: Python-style truthiness evaluation

### Python Built-in Types with Complete Method Sets

#### PyStr (String Type)
- **Core Methods**: `split()`, `join()`, `strip()`, `lower()`, `upper()`, `replace()`
- **Search Methods**: `find()`, `count()`, `startswith()`, `endswith()`
- **Formatting**: `format()` (basic implementation)
- **Full Python semantics**: Length, truthiness, display formatting

#### PyList (List Type)  
- **Modification**: `append()`, `extend()`, `insert()`, `remove()`, `pop()`, `clear()`
- **Search/Sort**: `index()`, `count()`, `sort()`, `reverse()`
- **Utilities**: `copy()`, indexing with `get()`/`set()`
- **Full Python semantics**: Length, truthiness, iteration support

#### PyDictionary (Dictionary Type)
- **Access**: `get()`, `get_or_default()`, `contains_key()`
- **Modification**: `set()`, `pop()`, `clear()`, `update()`
- **Iteration**: `keys()`, `values()`, `items()`
- **Full Python semantics**: Length, truthiness

#### PyTuple (Tuple Type)
- **Immutable sequence**: Index access, slicing support
- **Full Python semantics**: Length, truthiness, proper display formatting

#### PySet (Set Type)
- **Modification**: `add()`, `remove()`, `discard()`, `clear()`
- **Set Operations**: `union()`, `intersection()`, `difference()`
- **Membership**: `contains()`
- **Full Python semantics**: Length, truthiness

### Exception System
Complete Python exception hierarchy:
- **PyException**: Base exception class with proper Display/Error traits
- **Specific Exceptions**: ValueError, TypeError, IndexError, KeyError, AttributeError, NameError, ZeroDivisionError, OverflowError, RuntimeError
- **Helper Functions**: Convenient constructors for each exception type

### Runtime Traits
- **Len**: Universal length calculation for all collection types
- **Truthy**: Python-style truthiness evaluation (empty = False, non-empty = True)
- **Implementations**: Full trait coverage for Rust built-ins (String, Vec, primitives)

## Integration with Rython Ecosystem

This crate provides the complete runtime foundation for:

- **rythonc**: Python-to-Rust compiler uses this for all built-in runtime support
- **python-mod-rs**: Embeds Python modules that depend on these built-ins
- **python-ast-rs**: Generates code that calls these runtime functions

## Usage in Compiled Python Code

When Python code is compiled to Rust, it generates calls to functions in this library. The generic trait system allows natural Python-like code:

```python
# Python code:
my_list = [1, 2, 3]
total = sum(my_list)
print(str(total))
print(bool(my_list))
```

```rust
// Generated Rust code (using generic functions):
let mut my_list = PyList::from_vec(vec![1, 2, 3]);
let total = sum(&my_list);  // Works with PyList<T> via PySum trait
print(str(total));          // Works with any T via PyToString trait  
print(bool(&my_list));      // Works with any T via PyBool trait
```

### Benefits of Generic Design

1. **Single Function Names**: `abs()` instead of `abs_int()`, `abs_float()`
2. **Type Flexibility**: Functions work with any type implementing the appropriate trait
3. **Python Compatibility**: Exact same function signatures as Python
4. **Performance**: Zero-cost abstractions - traits compile to direct calls
5. **Extensibility**: Easy to add support for new types by implementing traits

## No-Std Support

The library supports `no_std` environments for embedded systems and resource-constrained targets.

### What's Available in No-Std Mode

✅ **Core Python Functions**: `abs()`, `min()`, `max()`, `sum()`, `all()`, `any()`, `len()`
✅ **Type Conversions**: `bool()`, `int()`, `float()`, `str()` with error handling
✅ **Collections**: `PyList`, `PyDictionary`, `PyTuple`, `PySet`, `PyStr` with all methods
✅ **Generic Traits**: All trait-based functionality works identically
✅ **Exception System**: Complete exception hierarchy for error handling
✅ **Iteration**: `enumerate()`, `zip()`, `range()` functions

### What's Not Available in No-Std Mode

❌ **I/O Operations**: `print()`, `input()`, `open()`, file operations (require OS)
❌ **PyO3 Integration**: Python interop features (require std library)

### No-Std Alternatives

Instead of OS-dependent functions, no-std provides string-based alternatives:
- `print_to_string()` instead of `print()` - returns formatted output as String
- `print_args_to_string()` instead of `print_args()` - formats multiple arguments

### Usage Example (No-Std)

```rust
#![no_std]
extern crate alloc;
use alloc::vec;
use stdpython::*;

fn embedded_main() {
    // All core Python functions work
    let nums = vec![1, 2, 3, 4, 5];
    let total = sum(&nums[..]);  // Works with generic traits
    
    // Collections work identically
    let mut list = PyList::from_vec(nums);
    list.append(6);
    
    // String operations work
    let message = str(total);  // "15"
    let output = print_to_string(&message);  // Returns string instead of printing
    
    // All type conversions and logic work
    assert_eq!(bool(&list), true);
    assert_eq!(len(&list), 6);
}
```

## Development Notes

- **Complete Python Compatibility**: All built-ins behave exactly like their Python counterparts
- **Performance Optimized**: Native Rust implementations provide better performance than interpreted Python
- **Memory Safe**: All operations are memory-safe with Rust's ownership system
- **Well Tested**: Comprehensive test suite ensures reliability (5 test modules, 20+ test cases)
- **Extensible**: Easy to add new built-ins as Rython language support expands
- **Documentation**: All functions fully documented with Python-compatible docstrings

## Testing and Quality Assurance

The library includes extensive testing:
- **Unit Tests**: Every major function and method tested
- **Integration Tests**: Complex interactions between types tested  
- **Edge Cases**: Boundary conditions and error cases covered
- **Python Compatibility**: Behavior matches Python reference implementation

Run `cargo test` to execute the full test suite and verify all functionality works correctly.