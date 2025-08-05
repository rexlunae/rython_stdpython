# Test Python iteration functions via compiled code

def test_enumerate_function():
    # Test enumerate()
    items = ["a", "b", "c"]
    enumerated = list(enumerate(items))
    return enumerated

def test_zip_function():
    # Test zip()
    list1 = [1, 2, 3]
    list2 = ["a", "b", "c"]
    zipped = list(zip(list1, list2))
    return zipped

def test_range_function():
    # Test range() in different forms
    range1 = list(range(5))
    range2 = list(range(2, 8))
    range3 = list(range(0, 10, 2))
    
    return range1, range2, range3

def test_list_comprehension():
    # Test list comprehension with stdpython functions
    numbers = [1, 2, 3, 4, 5]
    
    # Use abs, sum, len in comprehension
    abs_values = [abs(x) for x in [-1, -2, 3, -4, 5]]
    doubled = [x * 2 for x in numbers]
    sum_result = sum(doubled)
    
    return abs_values, doubled, sum_result

def test_filtering():
    # Test filtering with bool conversions
    mixed = [0, 1, "", "hello", [], [1, 2], {}, {"a": 1}]
    
    # Filter truthy values
    truthy = [x for x in mixed if bool(x)]
    
    return truthy