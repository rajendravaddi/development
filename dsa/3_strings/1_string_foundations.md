# String Foundations

Strings are one of the most important data types in DSA.

A string is essentially a **sequence of characters**. This means many techniques that we already learned with arrays can also be applied to strings:

- Traversal
- Indexing
- Two pointers
- Sliding window
- Searching
- Prefix/suffix processing

However, strings have some important properties that are different from arrays, especially **immutability**, character representation, and string construction.


## 1. Characters

A **character** is a single element of a string.

For example:

```python
s = "hello"
```

The string contains five characters:

```text
h  e  l  l  o
```

We can access each character using its index:

```python
s = "hello"

print(s[0])  # h
print(s[1])  # e
print(s[4])  # o
```

Like arrays, string indexing starts from `0`.

```text
String:    h   e   l   l   o
Index:     0   1   2   3   4
```

### A character is also a string in Python

Python does not have a separate `char` data type like some other languages.

For example:

```python
s = "hello"

c = s[0]

print(c)
print(type(c))
```

Output:

```text
h
<class 'str'>
```

So:

```python
s[0]
```

returns a string of length `1`, not a special `char` object.

```python
c = "h"

print(len(c))  # 1
```


### Characters as elements of a sequence

It is useful to think about a string as an array-like sequence:

```text
String:
"HELLO"

Equivalent conceptual representation:

['H', 'E', 'L', 'L', 'O']
```

This does **not** mean Python internally stores a string exactly as a Python list.

It means that from a DSA perspective, we can reason about it as a sequence of characters.

This is why many array techniques can be adapted to strings.

For example, a two-pointer technique:

```python
s = "hello"

left = 0
right = len(s) - 1

while left < right:
    print(s[left], s[right])

    left += 1
    right -= 1
```

Output:

```text
h o
e l
```

The important idea is:

> A string is an ordered sequence of characters.


### Character types

Characters can represent many things:

```text
'a'       lowercase letter
'Z'       uppercase letter
'7'       digit character
' '       space
'@'       symbol
'😀'      Unicode character
```

Notice the difference between:

```python
7
```

and:

```python
"7"
```

The first is an integer.

The second is a character/string containing a digit.

For example:

```python
print(type(7))
print(type("7"))
```

Output:

```text
<class 'int'>
<class 'str'>
```

This distinction becomes important in string problems.


### Character vs number

Suppose:

```python
s = "123"
```

Then:

```python
s[0]
```

is:

```text
"1"
```

not:

```text
1
```

If we need the numeric value:

```python
x = int(s[0])

print(x)
```

Output:

```text
1
```

So we should distinguish:

```text
'1'  → character representing digit 1
 1   → integer value 1
```

This distinction is frequently important in DSA problems involving strings.


## 2. ASCII / Unicode

Computers ultimately represent data using numbers.

Characters are therefore associated with numeric values.

Two important concepts are:

- ASCII
- Unicode


### ASCII

ASCII stands for:

**American Standard Code for Information Interchange**

ASCII assigns numeric values to common characters.

For example:

```text
Character    ASCII

'A'           65
'B'           66
'C'           67
...
'Z'           90

'a'           97
'b'           98
'c'           99
...
'z'          122

'0'           48
'1'           49
...
'9'           57
```

This creates useful relationships.

```text
'B' - 'A' = 1
'C' - 'A' = 2
```

#### `ord()`

`ord()` converts a single character into its numeric Unicode code point.

```python
print(ord('A'))  # 65
print(ord('a'))  # 97
```

This is useful when solving problems involving character positions.

For example, to convert lowercase English letters to indices:

```python
index = ord('c') - ord('a')

print(index)
```

Output:

```text
2
```

Therefore:

```text
'a' → 0
'b' → 1
'c' → 2
...
'z' → 25
```

This technique will become especially useful when we work with character frequency arrays.


#### `chr()`

The opposite operation is `chr()`.

It converts a numeric Unicode code point into a character.

```python
print(chr(65))
print(chr(66))
print(chr(97))
```

Output:

```text
A
B
a
```

So:

```text
ord('A') → 65
chr(65)  → 'A'
```

They are essentially inverse operations.


### Unicode

ASCII is limited.

It mainly represents English letters, digits, punctuation, and control characters.

Modern applications need to represent characters from many languages and writing systems.

For example:

```text
English:    A
Hindi:      अ
Telugu:     అ
Japanese:   あ
Arabic:     م
Emoji:      😀
```

Unicode provides a much larger character system.

Python strings support Unicode naturally.

```python
s = "Hello 😀"

print(s)
```

Output:

```text
Hello 😀
```

We can also use characters from different languages:

```python
s = "Hello नमस्ते"

print(s)
```

#### Unicode code points

Unicode assigns code points to characters.

For example:

```python
print(ord('A'))
print(ord('😀'))
```

The exact numeric value is the Unicode code point.

We can convert it back:

```python
c = '😀'

value = ord(c)

print(chr(value))
```

Output:

```text
😀
```


### Important DSA point

For most beginner and intermediate DSA problems, the input is often restricted to something like:

```text
'a' - 'z'
```

or:

```text
'A' - 'Z'
```

or:

```text
'a' - 'z' + 'A' - 'Z'
```

In such cases, ASCII/Unicode numeric values can be very useful for mapping characters to array indices.

For example:

```python
s = "hello"

freq = [0] * 26

for ch in s:
    index = ord(ch) - ord('a')
    freq[index] += 1
```

For:

```text
"hello"
```

the frequency array conceptually stores:

```text
a → 0
b → 0
c → 0
...
e → 1
h → 1
l → 2
o → 1
```

We will use this idea later for frequency-based problems.


## 3. Indexing

Indexing means accessing a character at a particular position.

Consider:

```python
s = "abcdef"
```

The indices are:

```text
String:    a   b   c   d   e   f
Index:     0   1   2   3   4   5
```

We can access characters using:

```python
print(s[0])  # a
print(s[2])  # c
print(s[5])  # f
```


### Negative indexing

Negative indices count from the end.

```text
String:     h    e    l    l    o
Positive:   0    1    2    3    4
Negative:  -5   -4   -3   -2   -1
```

Therefore:

```python
s[-1]  # last character
s[-2]  # second-last character
s[-3]  # third-last character
```

Example:

```python
s = "hello"

print(s[-1])  # o
print(s[-2])  # l
print(s[-3])  # l
```


### Indexing complexity

Accessing a character by index is generally:

```text
Time: O(1)
```

For example:

```python
s[100]
```

does not require traversing all previous characters.

This is similar to array indexing.


### Invalid index

If the index is outside the valid range:

```python
s = "hello"

print(s[10])
```

Python raises:

```text
IndexError
```

The valid positive range is:

```text
0 <= index < len(s)
```


## 4. Slicing

Slicing allows us to extract a portion of a string.

The basic syntax is:

```python
s[start:end]
```

The important rule is:

> `start` is included, `end` is excluded.

Consider:

```python
s = "abcdef"
```

Then:

```python
print(s[1:4])
```

Output:

```text
bcd
```

Because:

```text
index:     0   1   2   3   4   5
string:    a   b   c   d   e   f
                ↑       ↑
              start    end
```

Indices `1`, `2`, and `3` are included.

Index `4` is excluded.


### Why is the end exclusive?

This convention is extremely useful.

For example:

```python
s[0:3]
```

contains exactly:

```text
3 characters
```

because:

```text
3 - 0 = 3
```

Similarly:

```python
s[2:5]
```

contains:

```text
5 - 2 = 3
```

characters.

This makes substring lengths easy to reason about.


### Omitting start

```python
s[:4]
```

means:

```python
s[0:4]
```

Example:

```python
s = "abcdef"

print(s[:4])
```

Output:

```text
abcd
```


### Omitting end

```python
s[2:]
```

means:

```python
s[2:len(s)]
```

Example:

```python
s = "abcdef"

print(s[2:])
```

Output:

```text
cdef
```


### Copying the entire string

```python
s[:]
```

selects the entire string.

```python
s = "hello"

t = s[:]

print(t)
```

Output:

```text
hello
```


### Slicing with a step

The full slicing syntax is:

```python
s[start:end:step]
```

For example:

```python
s = "abcdef"

print(s[0:6:2])
```

Output:

```text
ace
```

We take:

```text
index 0 → a
index 2 → c
index 4 → e
```


### Reversing using slicing

A common Python technique is:

```python
s[::-1]
```

Example:

```python
s = "hello"

print(s[::-1])
```

Output:

```text
olleh
```

The step `-1` means move backwards.


### Slicing creates a new string

Slicing is important from a complexity perspective.

For example:

```python
t = s[2:7]
```

creates a new string containing the selected characters.

If the slice contains `k` characters, the operation generally requires:

```text
Time: O(k)
Space: O(k)
```

This matters when writing DSA solutions.

For example, repeatedly doing:

```python
s[i:j]
```

inside a loop can sometimes create unnecessary work.

So although slicing is convenient, we should always consider its complexity.


## 5. Immutability

One of the most important differences between Python strings and mutable arrays/lists is:

> **Strings are immutable.**

Immutable means:

> Once a string object is created, its individual characters cannot be changed.

Consider:

```python
s = "hello"
```

We cannot do:

```python
s[0] = "H"
```

This produces an error because strings do not support item assignment.

### What does immutable mean?

Suppose:

```python
s = "hello"
```

Conceptually:

```text
s
↓
"h e l l o"
```

We cannot modify the first character directly.

This is different from a list:

```python
arr = ['h', 'e', 'l', 'l', 'o']

arr[0] = 'H'

print(arr)
```

Output:

```text
['H', 'e', 'l', 'l', 'o']
```

Lists are mutable.

Strings are immutable.


### Creating a new string

Although we cannot modify a string directly, we can create another string.

For example:

```python
s = "hello"

s = "H" + s[1:]

print(s)
```

Output:

```text
Hello
```

We didn't modify the original string.

Instead, we created a new string:

```text
"H" + "ello"
```

which produces:

```text
"Hello"
```

and then assigned that new string to `s`.


### Repeated modification

Consider:

```python
s = ""

for ch in "hello":
    s += ch
```

This works, but repeatedly creating strings can be inefficient because each concatenation may create another string.

For small inputs this is usually fine.

For large-scale DSA problems, we need to think carefully about string construction.

This becomes especially important when an algorithm builds a result character by character.

### Why immutability matters in DSA

Suppose we want to change:

```text
"hello"
```

into:

```text
"Hello"
```

We cannot simply modify:

```python
s[0]
```

Instead, we need to construct another string.

This affects:

- String reversal
- String transformation
- String construction
- Repeated concatenation
- Algorithms that modify many characters

Therefore, whenever we work with strings, we should remember:

```text
String → immutable
List   → mutable
```

## 6. Traversal

Traversal means visiting the characters of a string one by one.

This is one of the most fundamental operations in string algorithms.

Consider:

```python
s = "hello"
```

We can traverse it using a `for` loop:

```python
for ch in s:
    print(ch)
```

Output:

```text
h
e
l
l
o
```

This is usually the cleanest way to traverse a string when we only need the characters.


### Traversal using indices

We can also traverse using indices:

```python
s = "hello"

for i in range(len(s)):
    print(s[i])
```

Output:

```text
h
e
l
l
o
```

Here we have access to both:

```text
i → index
s[i] → character
```

This is useful when the algorithm needs the position of a character.


### Character traversal vs index traversal

Use:

```python
for ch in s:
```

when you only need the character.

Use:

```python
for i in range(len(s)):
```

when you need the index.

For example, finding the first occurrence of `'e'`:

```python
s = "hello"

for i in range(len(s)):
    if s[i] == 'e':
        print(i)
        break
```

Output:

```text
1
```


### Traversal with two pointers

Because strings are indexable, we can use the two-pointer pattern.

Example:

```python
s = "abcdef"

left = 0
right = len(s) - 1

while left < right:
    print(s[left], s[right])

    left += 1
    right -= 1
```

Output:

```text
a f
b e
c d
```

This is the same two-pointer idea we already learned with arrays.

This is an important connection:

> Strings can often be treated as read-only arrays of characters for algorithmic purposes.


### Traversal complexity

If a string contains `n` characters and we visit every character once:

```text
Time: O(n)
```

If we only use a few variables:

```text
Auxiliary space: O(1)
```

For example:

```python
count = 0

for ch in s:
    if ch == 'a':
        count += 1
```

Time:

```text
O(n)
```

Extra space:

```text
O(1)
```


## 7. Comparison

Strings can be compared using comparison operators.

For example:

```python
a = "apple"
b = "banana"

print(a == b)
```

Output:

```text
False
```

We can use:

```python
==
!=
<
>
<=
>=
```


### Equality

`==` checks whether two strings contain the same sequence of characters.

```python
print("hello" == "hello")
```

Output:

```text
True
```

But:

```python
print("hello" == "Hello")
```

Output:

```text
False
```

String comparison is case-sensitive.


### Lexicographical comparison

Strings can also be ordered lexicographically.

This is similar to dictionary ordering.

For example:

```python
print("apple" < "banana")
```

Output:

```text
True
```

Because `'a'` comes before `'b'`.

Similarly:

```python
print("cat" > "apple")
```

Output:

```text
True
```

### How lexicographical comparison works

Consider:

```text
"apple"
"apply"
```

Compare from left to right:

```text
a = a
p = p
p = p
l = l
e < y
```

Therefore:

```python
"apple" < "apply"
```

is:

```text
True
```

The comparison stops at the first position where the characters differ.


### Different lengths

Consider:

```text
"app"
"apple"
```

The first three characters are equal:

```text
a = a
p = p
p = p
```

The shorter string comes first.

Therefore:

```python
print("app" < "apple")
```

Output:

```text
True
```


### Character ordering

Lexicographical comparison is based on character ordering.

For ASCII characters:

```text
'A' = 65
'B' = 66
...
'Z' = 90

'a' = 97
'b' = 98
...
'z' = 122
```

Therefore:

```python
print('A' < 'a')
```

Output:

```text
True
```

because:

```text
65 < 97
```

This is why uppercase and lowercase characters can behave differently during comparisons.


### Case-sensitive comparison

```python
a = "hello"
b = "Hello"

print(a == b)
```

Output:

```text
False
```

If a problem says that comparison should be case-insensitive, we need to explicitly normalize the strings.

For example:

```python
a = "Hello"
b = "hello"

print(a.lower() == b.lower())
```

Output:

```text
True
```

The important DSA lesson is:

> Always check whether the problem requires case-sensitive or case-insensitive comparison.


### Comparison complexity

If two strings have lengths `n` and `m`, comparison can take up to:

```text
O(min(n, m))
```

because the comparison may need to inspect characters until:

- A mismatch is found, or
- One string ends.

For example:

```text
"aaaaaaaaab"
"aaaaaaaaac"
```

The strings differ only near the end, so many characters must be examined.


## 8. Construction

String construction means creating a new string from characters or other strings.

This is extremely important in DSA because many problems require us to produce a transformed string.


### Basic concatenation

We can concatenate strings using `+`.

```python
a = "hello"
b = "world"

c = a + " " + b

print(c)
```

Output:

```text
hello world
```


### Building from characters

Suppose:

```text
characters:
h e l l o
```

We can construct:

```python
s = "h" + "e" + "l" + "l" + "o"

print(s)
```

Output:

```text
hello
```

However, repeatedly concatenating many characters can be inefficient for large inputs because strings are immutable.


### Building using a list

A common DSA technique is to collect characters in a list and then join them.

```python
chars = []

for ch in "hello":
    chars.append(ch)

result = "".join(chars)

print(result)
```

Output:

```text
hello
```

The important idea is:

```text
characters
    ↓
list
    ↓
join
    ↓
string
```

This is useful when an algorithm constructs a large result.


### `join()`

`join()` combines strings from an iterable.

For example:

```python
chars = ['h', 'e', 'l', 'l', 'o']

result = "".join(chars)

print(result)
```

Output:

```text
hello
```

We can also specify a separator:

```python
words = ["I", "love", "DSA"]

result = " ".join(words)

print(result)
```

Output:

```text
I love DSA
```

Another example:

```python
numbers = ["1", "2", "3"]

result = "-".join(numbers)

print(result)
```

Output:

```text
1-2-3
```


### Construction using slicing

Because strings are immutable, we often construct new strings using slices.

For example, suppose:

```python
s = "hello"
```

and we want:

```text
"heXlo"
```

We can construct:

```python
s = s[:2] + "X" + s[3:]

print(s)
```

Output:

```text
heXlo
```

We didn't modify the original characters.

Instead:

```text
"he" + "X" + "lo"
```

created a new string.


### Construction and complexity

When constructing strings, we need to consider how many characters are being copied.

For example:

```python
result = ""

for ch in s:
    result += ch
```

The repeated concatenation can lead to unnecessary copying and poor performance for large strings.

A more appropriate pattern for large construction is often:

```python
chars = []

for ch in s:
    chars.append(ch)

result = "".join(chars)
```

The exact complexity depends on the operations performed, but the important principle is:

> Avoid repeatedly rebuilding large immutable strings when you can accumulate pieces and construct the final string once.

## 9. Python String Methods

These are the most useful Python string methods for DSA problems.

| Method | Purpose | Example | Result |
|---|---|---|---|
| `len()` | Get string length | `len("hello")` | `5` |
| `lower()` | Convert to lowercase | `"Hello".lower()` | `"hello"` |
| `upper()` | Convert to uppercase | `"hello".upper()` | `"HELLO"` |
| `strip()` | Remove leading/trailing whitespace | `" hi ".strip()` | `"hi"` |
| `lstrip()` | Remove leading whitespace | `" hi".lstrip()` | `"hi"` |
| `rstrip()` | Remove trailing whitespace | `"hi ".rstrip()` | `"hi"` |
| `replace()` | Replace occurrences | `"hello".replace("l", "x")` | `"hexxo"` |
| `find()` | Find first occurrence | `"hello".find("l")` | `2` |
| `rfind()` | Find last occurrence | `"hello".rfind("l")` | `3` |
| `count()` | Count occurrences | `"hello".count("l")` | `2` |
| `startswith()` | Check prefix | `"hello".startswith("he")` | `True` |
| `endswith()` | Check suffix | `"hello".endswith("lo")` | `True` |
| `split()` | Split string into parts | `"a,b,c".split(",")` | `['a', 'b', 'c']` |
| `join()` | Join strings | `"-".join(["a","b","c"])` | `"a-b-c"` |
| `isalpha()` | Check if all characters are letters | `"abc".isalpha()` | `True` |
| `isdigit()` | Check if all characters are digits | `"123".isdigit()` | `True` |
| `isalnum()` | Check if all are letters/digits | `"abc123".isalnum()` | `True` |
| `isspace()` | Check if all are whitespace | `"   ".isspace()` | `True` |
