# Collection operations using stdpython types
def test_list_ops():
    my_list = [1, 2, 3]
    my_list.append(4)
    return len(my_list), my_list
    
def test_string_ops():
    text = "Hello World"
    upper = text.upper()
    words = text.split(" ")
    return upper, words, len(text)
    
def test_enumerate_zip():
    items = ["a", "b", "c"]
    numbers = [1, 2, 3]
    
    enumerated = list(enumerate(items))
    zipped = list(zip(numbers, items))
    
    return enumerated, zipped