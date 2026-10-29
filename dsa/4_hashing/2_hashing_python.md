# Python Hash-Based Data Structures

## 1. Python dict

### What is dict?

A **dictionary (`dict`)** is Python's built-in hash-based data structure for storing **key-value pairs**.

The general form is:

```python
dictionary = {
    key: value
}
```

Example:

```python
student = {
    "name": "Raj",
    "age": 25,
    "marks": 90
}
```

Here:

```text
"name"  → "Raj"
"age"   → 25
"marks" → 90
```

The keys are used to locate their corresponding values.

Python dictionaries are implemented using a hash-table-based design and provide **average O(1)** lookup, insertion, and deletion.

### Important properties

- Keys must be **hashable**.
- Keys must be unique.
- Values can be duplicated.
- Values do not need to be hashable.

For example:

```python
data = {
    "a": 10,
    "b": 10
}
```

This is valid because values do not have to be unique.

But:

```python
data = {
    "a": 10,
    "a": 20
}
```

does not create two `"a"` entries. The later assignment replaces the previous value.


### Creating dictionaries

#### Using `{}`

```python
student = {}
```

This creates an empty dictionary.

#### Using key-value pairs

```python
student = {
    "name": "Raj",
    "age": 25
}
```

#### Using `dict()`

```python
student = dict(name="Raj", age=25)
```

#### From a list of pairs

```python
student = dict([
    ("name", "Raj"),
    ("age", 25)
])
```

Result:

```python
{
    "name": "Raj",
    "age": 25
}
```

### Accessing values

Values can be accessed using their keys.

```python
student = {
    "name": "Raj",
    "age": 25
}

print(student["name"])
```

Output:

```text
Raj
```

If the key does not exist:

```python
student["city"]
```

Python raises:

```text
KeyError
```

For safer access when a key may not exist, use `get()`.

```python
student.get("city")
```

This returns:

```python
None
```

unless a default value is provided.


### Inserting

A new key-value pair can be inserted using assignment.

```python
student = {
    "name": "Raj"
}

student["age"] = 25
```

Now:

```python
{
    "name": "Raj",
    "age": 25
}
```

If the key does not already exist, assignment inserts a new entry.


### Updating

If the key already exists, assignment updates its value.

```python
student = {
    "name": "Raj",
    "age": 25
}

student["age"] = 26
```

Result:

```python
{
    "name": "Raj",
    "age": 26
}
```

Multiple entries can be updated using `update()`:

```python
student.update({
    "age": 26,
    "city": "Kurnool"
})
```

`update()`:

- Changes existing keys.
- Adds new keys.


### Deleting

A dictionary entry can be deleted using `del`.

```python
student = {
    "name": "Raj",
    "age": 25
}

del student["age"]
```

Result:

```python
{
    "name": "Raj"
}
```

If the key does not exist, `del` raises `KeyError`.

You can also use `pop()` when you want to remove and retrieve a value.

```python
age = student.pop("age")
```

### Membership testing

The `in` operator checks whether a **key** exists in a dictionary.

```python
student = {
    "name": "Raj",
    "age": 25
}

"name" in student
```

Result:

```python
True
```

Similarly:

```python
"city" in student
```

returns:

```python
False
```

#### Important

For dictionaries:

```python
key in dictionary
```

checks **keys**, not values.

To check values:

```python
25 in student.values()
```


### Iterating over dictionaries

A dictionary can be traversed using a `for` loop.

By default, iteration is over keys:

```python
student = {
    "name": "Raj",
    "age": 25
}

for key in student:
    print(key)
```

Output:

```text
name
age
```

To access both keys and values:

```python
for key, value in student.items():
    print(key, value)
```

Output:

```text
name Raj
age 25
```

Python dictionaries preserve **insertion order** in modern Python versions.

### `keys()`

Returns a view containing the dictionary's keys.

```python
student.keys()
```

Example:

```python
for key in student.keys():
    print(key)
```


### `values()`

Returns a view containing the dictionary's values.

```python
student.values()
```

Example:

```python
for value in student.values():
    print(value)
```

### `items()`

Returns a view containing `(key, value)` pairs.

```python
student.items()
```

Example:

```python
for key, value in student.items():
    print(key, value)
```

### pop()

`pop()` removes a key-value pair and returns the value.

```python
student = {
    "name": "Raj",
    "age": 25
}

age = student.pop("age")
```

Now:

```text
age = 25
```

and the dictionary becomes:

```python
{
    "name": "Raj"
}
```

A default value can be supplied:

```python
student.pop("city", "Unknown")
```

If `"city"` does not exist, `"Unknown"` is returned instead of raising `KeyError`.


## 2. Python set

### What is set?

A **set** is Python's built-in collection for storing **unique, hashable elements**.

Example:

```python
numbers = {10, 20, 30}
```

A set is useful when the primary concerns are:

- Membership testing
- Uniqueness
- Set operations

Unlike a dictionary, a set stores elements rather than key-value pairs.

### Uniqueness

A set cannot contain duplicate elements.

```python
numbers = {10, 20, 10, 30, 20}
```

The resulting set contains only:

```python
{10, 20, 30}
```

The duplicate values are automatically eliminated.

This makes sets useful for duplicate removal.

```python
numbers = [10, 20, 10, 30, 20]

unique = set(numbers)
```

Result:

```python
{10, 20, 30}
```

### Creating sets

#### Using `{}`

```python
numbers = {10, 20, 30}
```

#### Using `set()`

```python
numbers = set()
```

This is how you create an empty set.

Important:

```python
{}
```

creates an **empty dictionary**, not an empty set.

#### From another iterable

```python
numbers = set([10, 20, 30, 20])
```

Result:

```python
{10, 20, 30}
```

### Adding

Use `add()` to insert one element.

```python
numbers = {10, 20}

numbers.add(30)
```

Result:

```python
{10, 20, 30}
```

Adding an element that already exists does not create a duplicate.

```python
numbers.add(20)
```

The set remains unchanged.

### Removing

#### `remove()`

```python
numbers.remove(20)
```

Removes `20`.

If the element does not exist, `remove()` raises:

```text
KeyError
```

#### `discard()`

```python
numbers.discard(20)
```

Removes `20` if present.

If it is absent, nothing happens.

#### Difference

```text
remove(x)
    → raises KeyError if x is absent

discard(x)
    → does not raise an error if x is absent
```


### Membership testing

The `in` operator checks whether an element exists in a set.

```python
numbers = {10, 20, 30}

20 in numbers
```

Result:

```python
True
```

And:

```python
50 in numbers
```

returns:

```python
False
```

Set membership is typically:

```text
O(1) average
```

which makes sets very useful for fast existence checks.


### Set operations

Python provides mathematical set operations for combining and comparing sets.

Suppose:

```python
A = {1, 2, 3, 4}
B = {3, 4, 5, 6}
```

#### Union

The **union** contains all elements that occur in either set.

Mathematically:

```text
A ∪ B
```

Python:

```python
A | B
```

or:

```python
A.union(B)
```

Result:

```python
{1, 2, 3, 4, 5, 6}
```

Duplicates are automatically removed.


#### Intersection

The **intersection** contains elements common to both sets.

Mathematically:

```text
A ∩ B
```

Python:

```python
A & B
```

or:

```python
A.intersection(B)
```

Result:

```python
{3, 4}
```

#### Difference

The **difference** contains elements present in the first set but not the second.

Mathematically:

```text
A − B
```

Python:

```python
A - B
```

or:

```python
A.difference(B)
```

Result:

```python
{1, 2}
```

Note that difference is **not symmetric**:

```python
B - A
```

produces:

```python
{5, 6}
```

#### Symmetric difference

The **symmetric difference** contains elements that belong to exactly one of the two sets.

In other words:

```text
Elements in A or B
but not in both
```

Mathematically:

```text
A △ B
```

Python:

```python
A ^ B
```

or:

```python
A.symmetric_difference(B)
```

Result:

```python
{1, 2, 5, 6}
```


## 3. Hashability

### What is a hashable object?

An object is **hashable** if it has a hash value that remains stable during its lifetime and can therefore be used as:

- A dictionary key
- A set element

Conceptually, a hashable object supports:

```python
hash(obj)
```

and has compatible equality behavior.

Examples of commonly hashable objects:

```python
int
float
str
bool
tuple        # if all its elements are hashable
frozenset
```

Example:

```python
hash("hello")
hash(100)
hash((1, 2, 3))
```

These can be used as dictionary keys or set elements.

### Hashable vs unhashable objects

Common hashable objects:

```text
int
float
str
bool
tuple of hashable elements
frozenset
```

Common unhashable objects:

```text
list
dict
set
```

Therefore:

```python
d = {
    "name": "Raj"
}
```

is valid.

But:

```python
d = {
    [1, 2]: "value"
}
```

raises:

```text
TypeError: unhashable type: 'list'
```

Similarly:

```python
s = {[1, 2, 3]}
```

is invalid because lists are unhashable.


### Immutable keys

Dictionary keys and set elements need stable hash values.

For this reason, immutable objects are commonly suitable as keys.

For example:

```python
d = {
    "name": "Raj",
    101: "Alice",
    (10, 20): "point"
}
```

Here:

- `"name"` is immutable.
- `101` is immutable.
- `(10, 20)` is immutable and contains hashable integers.

A mutable object could change after being placed into a hash table, potentially changing information relevant to its hash.

That would make reliable lookup impossible.


### Why lists cannot be dictionary keys

Lists are mutable.

For example:

```python
numbers = [1, 2, 3]
```

We can later change them:

```python
numbers.append(4)
```

If lists were allowed as dictionary keys, their contents could change after insertion.

That creates a fundamental problem: the object's hash-related identity could no longer be stable.

Therefore Python makes lists **unhashable**.

This is invalid:

```python
d = {
    [1, 2, 3]: "value"
}
```

and raises:

```text
TypeError: unhashable type: 'list'
```

### Why lists cannot be set elements

The same rule applies to sets.

Set elements must be hashable.

Therefore:

```python
s = {[1, 2, 3]}
```

is invalid.

A list can change:

```python
numbers = [1, 2, 3]
numbers.append(4)
```

so it cannot safely serve as a stable set element.

If you need an immutable sequence as a set element, use a tuple when all of its elements are hashable:

```python
s = {(1, 2, 3)}
```


### Tuples as dictionary keys

Tuples can be dictionary keys **if all elements inside the tuple are hashable**.

This works:

```python
locations = {
    (10, 20): "Point A",
    (30, 40): "Point B"
}
```

The tuple:

```python
(10, 20)
```

is hashable because both `10` and `20` are hashable.

But this does not work:

```python
key = ([1, 2], 3)

d = {
    key: "value"
}
```

because the tuple contains a list.

So:

```text
Tuple itself immutable
        +
All elements hashable
        ↓
Tuple is hashable
```

But:

```text
Tuple contains unhashable element
        ↓
Tuple is unhashable
```


### Nested hashability

Hashability can depend on the objects nested inside another object.

Consider:

```python
a = (1, 2, 3)
```

All elements are hashable:

```text
int → hashable
```

Therefore:

```python
hash(a)
```

works.

Now consider:

```python
b = (1, [2, 3])
```

The tuple itself is immutable, but it contains a list.

```text
list → unhashable
```

Therefore:

```python
hash(b)
```

raises a `TypeError`.

The important rule is:

> **An immutable container is not automatically hashable. Its contained elements must also be hashable.**

This is why a tuple can be either hashable or unhashable depending on its contents.

### Mutable objects and hashing

Hash-based structures rely on the hash-related identity of an object remaining stable while it is stored.

Consider a conceptual object whose hash depends on:

```text
name
age
```

If the object is inserted into a set and its `name` or `age` later changes, its hash could change.

The set may have stored it according to the old hash.

After mutation:

```text
Old state
   ↓
Old hash
   ↓
Stored at location A

Mutation
   ↓
New state
   ↓
New hash
   ↓
Lookup searches location B
```

The object can become effectively unreachable through normal hash-based lookup.

This is one reason mutable objects are generally unsuitable when their mutable state participates in hashing.


## 4. Custom Objects

Python allows user-defined classes to participate in hashing.

Two special methods are particularly important:

```python
__hash__()
__eq__()
```

They determine how objects behave in hash-based collections.


### `__hash__`

`__hash__` returns the integer hash value of an object.

Example:

```python
class Student:
    def __hash__(self):
        return hash(self.name)
```

Then:

```python
student = Student()
hash(student)
```

calls the object's `__hash__()` implementation.

The hash value is used by dictionaries and sets to determine where the object belongs.

#### Important

You normally should not implement `__hash__` independently of equality. It must be consistent with `__eq__`.


### `__eq__`

`__eq__` defines how two objects are compared for equality.

Example:

```python
class Student:
    def __init__(self, name):
        self.name = name

    def __eq__(self, other):
        return self.name == other.name
```

Now two `Student` objects with the same name can be considered equal.

```python
Student("Raj") == Student("Raj")
```

can return:

```python
True
```


### Relationship between equality and hashing

Hash-based collections use both:

```text
hash()
```

and:

```text
==
```

Conceptually, lookup works like:

```text
Key
 ↓
Calculate hash
 ↓
Find candidate location
 ↓
Compare candidate keys using equality
```

The hash narrows down where to look.

Equality determines whether the objects are actually considered equal.

Therefore, `__hash__` and `__eq__` must be designed consistently.


### Equal objects → same hash

Python requires the following rule:

> If two objects compare equal, they must have the same hash value.

Formally:

```text
a == b
    ⇒
hash(a) == hash(b)
```

For example:

```python
class Student:
    def __init__(self, student_id):
        self.student_id = student_id

    def __eq__(self, other):
        return self.student_id == other.student_id

    def __hash__(self):
        return hash(self.student_id)
```

Then:

```python
a = Student(101)
b = Student(101)
```

should satisfy:

```python
a == b
```

and:

```python
hash(a) == hash(b)
```

#### Important

The reverse is **not** required.

If:

```python
hash(a) == hash(b)
```

it does **not** mean:

```python
a == b
```

That is a hash collision.


### Hash collision ≠ equality

Two different objects can have the same hash value.

For example:

```text
hash(a) = 42
hash(b) = 42
```

but:

```text
a != b
```

This is a **hash collision**.

Hash-based collections handle this by using equality checks after identifying candidate entries.

Therefore:

```text
Same hash
    ≠
Equal objects
```

But:

```text
Equal objects
    ⇒
Same hash
```

This distinction is fundamental when implementing custom hashable objects.


### Hashing custom objects safely

When designing a custom hashable class, the safest approach is to base both equality and hashing on the **same immutable attributes**.

For example:

```python
class Student:
    def __init__(self, student_id):
        self.student_id = student_id

    def __eq__(self, other):
        if not isinstance(other, Student):
            return NotImplemented
        return self.student_id == other.student_id

    def __hash__(self):
        return hash(self.student_id)
```

Here both methods use:

```text
student_id
```

Therefore:

```text
Equality identity → student_id
Hash identity     → student_id
```

A good design should satisfy:

```text
a == b
    ↓
hash(a) == hash(b)
```

Also, the attributes that determine the hash should not change while the object is being used as a dictionary key or set element.


### Problems with mutating hash-relevant state

Suppose a class uses `student_id` for both equality and hashing:

```python
class Student:
    def __init__(self, student_id):
        self.student_id = student_id

    def __eq__(self, other):
        return self.student_id == other.student_id

    def __hash__(self):
        return hash(self.student_id)
```

Now:

```python
student = Student(101)

students = {student}
```

The set places `student` according to:

```text
hash(101)
```

If we later change:

```python
student.student_id = 202
```

the object's hash becomes based on:

```text
hash(202)
```

But the set still has the object stored according to its previous hash/location.

This can lead to unexpected behavior:

```python
student in students
```

may return:

```python
False
```

even though the same object is physically present in the set.

#### Rule

> **Do not mutate attributes that participate in `__eq__` or `__hash__` while an object is being used as a dictionary key or set element.**

A common safe design is to make hash-relevant state immutable.
