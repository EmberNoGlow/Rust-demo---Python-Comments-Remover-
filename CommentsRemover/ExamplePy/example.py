# String (text)
name = "Alice"
greeting = "Hello, " + name + "!"
print(greeting)

# Integer (whole number)
age = 30
print(f"Alice is {age} years old.") # f-strings are a modern way to format strings

# Float (decimal number)
height = 5.75
print(f"Her height is {height} feet.")

# Boolean (True or False)
is_student = True
print(f"Is Alice a student? {is_student}")

# List (ordered, mutable collection)
fruits = ["apple", "banana", "cherry"]
print(f"Her favorite fruits are: {fruits}")
print(f"The first fruit is: {fruits[0]}") # Indexing starts at 0

# Tuple (ordered, immutable collection)
coordinates = (10, 20)
print(f"Coordinates: {coordinates}")
# coordinates[0] = 15 # This would cause an error because tuples are immutable

# Dictionary (key-value pairs)
person = {
    "name": "Bob",
    "age": 25,
    "city": "New York"
}
print(f"Person's details: {person}")
print(f"Bob's city: {person['city']}")