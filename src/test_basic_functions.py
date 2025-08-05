# Test basic Python functions calling stdpython runtime

def test_abs_function():
    # Test abs() with different types
    result1 = abs(-5)
    result2 = abs(-3.14)
    return result1, result2

def test_math_functions():
    # Test min, max, sum
    numbers = [1, 2, 3, 4, 5]
    min_val = min(numbers)
    max_val = max(numbers)
    sum_val = sum(numbers)
    return min_val, max_val, sum_val

def test_bool_functions():
    # Test all, any, bool
    all_true = [True, True, True]
    mixed = [True, False, True]
    empty_list = []
    
    all_result = all(all_true)
    any_result = any(mixed)
    bool_result = bool(42)
    bool_empty = bool(0)
    
    return all_result, any_result, bool_result, bool_empty

def test_type_conversions():
    # Test int, float, str conversions
    str_to_int = int("123")
    float_to_int = int(45.7)
    bool_to_int = int(True)
    
    int_to_float = float(42)
    str_to_float = float("3.14")
    
    int_to_str = str(123)
    float_to_str = str(3.14)
    bool_to_str = str(True)
    
    return (str_to_int, float_to_int, bool_to_int, 
            int_to_float, str_to_float,
            int_to_str, float_to_str, bool_to_str)

def test_len_function():
    # Test len() with different types
    string_len = len("hello")
    list_len = len([1, 2, 3, 4])
    return string_len, list_len