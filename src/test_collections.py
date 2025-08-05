# Test Python collections operations via compiled code

def test_list_operations():
    # Test list creation and methods
    my_list = [1, 2, 3]
    my_list.append(4)
    my_list.extend([5, 6])
    my_list.insert(0, 0)
    
    length = len(my_list)
    first_item = my_list[0] if len(my_list) > 0 else None
    
    # Test list methods
    my_list.reverse()
    popped = my_list.pop()
    
    return length, first_item, popped, my_list

def test_dict_operations():
    # Test dictionary creation and methods
    my_dict = {"a": 1, "b": 2}
    my_dict["c"] = 3
    
    keys = list(my_dict.keys())
    values = list(my_dict.values())
    length = len(my_dict)
    
    # Test dict methods
    value_a = my_dict.get("a", 0)
    value_missing = my_dict.get("missing", -1)
    
    return length, keys, values, value_a, value_missing

def test_string_operations():
    # Test string methods
    text = "Hello World"
    upper_text = text.upper()
    lower_text = text.lower()
    words = text.split(" ")
    joined = "-".join(words)
    
    # Test string properties
    length = len(text)
    starts_with = text.startswith("Hello")
    ends_with = text.endswith("World")
    
    return upper_text, lower_text, words, joined, length, starts_with, ends_with

def test_set_operations():
    # Test set operations
    set1 = {1, 2, 3}
    set2 = {3, 4, 5}
    
    set1.add(4)
    length = len(set1)
    
    # Test set operations
    union_result = set1.union(set2)
    intersection_result = set1.intersection(set2)
    
    return length, union_result, intersection_result

def test_tuple_operations():
    # Test tuple operations
    my_tuple = (1, 2, 3, 4)
    length = len(my_tuple)
    first = my_tuple[0] if len(my_tuple) > 0 else None
    
    return length, first, my_tuple