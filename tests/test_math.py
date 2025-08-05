# Simple math operations using stdpython functions
def test_abs():
    return abs(-42)
    
def test_sum_list():
    numbers = [1, 2, 3, 4, 5]
    return sum(numbers)
    
def test_min_max():
    numbers = [5, 1, 9, 3, 7]
    return min(numbers), max(numbers)
    
def test_type_conversion():
    # Test int, float, str, bool conversions
    int_val = int("123")
    float_val = float("3.14")
    str_val = str(456)
    bool_val = bool(1)
    return int_val, float_val, str_val, bool_val