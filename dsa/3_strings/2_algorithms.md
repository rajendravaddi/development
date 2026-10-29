# String Algorithms

String algorithms are techniques for processing, searching, comparing, transforming, and analyzing sequences of characters.

Since a string is an ordered sequence of characters, many string algorithms build directly on concepts we already know from arrays:

- Traversal
- Indexing
- Two pointers
- Counting
- Searching
- Complexity analysis

The important difference is that Python strings are **immutable**, so algorithms that modify strings usually construct a new string rather than changing the original.


## 1. Basic String Algorithms

### 1.1 String Reversal

String reversal means producing a string whose characters appear in the opposite order.

Example:

```text
Input:   "hello"
Output:  "olleh"
```

#### Using slicing

Python provides a convenient way to reverse a string:

```python
s = "hello"

reversed_s = s[::-1]

print(reversed_s)
```

Output:

```text
olleh
```

The `-1` step means traverse from right to left.


#### Using two pointers

From a DSA perspective, it is important to understand how reversal works without relying on a built-in shortcut.

Because strings are immutable, we can build the result:

```python
s = "hello"

result = []

left = 0
right = len(s) - 1

while left <= right:
    result.append(s[right])
    right -= 1

result = "".join(result)

print(result)
```

A simpler traversal from the end is:

```python
s = "hello"

result = []

for i in range(len(s) - 1, -1, -1):
    result.append(s[i])

result = "".join(result)

print(result)
```

#### Complexity

For a string of length `n`:

```text
Time:  O(n)
Space: O(n)
```

The `O(n)` space comes from constructing the reversed string.


### 1.2 Palindrome Checking

A palindrome is a string that reads the same from left to right and right to left.

Examples:

```text
"madam"   → palindrome
"racecar" → palindrome
"level"   → palindrome
```

Non-examples:

```text
"hello"   → not a palindrome
"python"  → not a palindrome
```

#### Basic approach

One way is to reverse the string and compare:

```python
s = "madam"

if s == s[::-1]:
    print("Palindrome")
else:
    print("Not a palindrome")
```

Output:

```text
Palindrome
```

This is simple, but it creates a reversed string.


#### Two-pointer approach

A more important DSA approach is to compare characters from both ends.

```python
s = "madam"

left = 0
right = len(s) - 1

is_palindrome = True

while left < right:
    if s[left] != s[right]:
        is_palindrome = False
        break

    left += 1
    right -= 1

print(is_palindrome)
```

The comparison happens like this:

```text
m a d a m
↑       ↑

m == m

  ↑   ↑
  a == a

    ↑
    d
```

If any pair differs, the string is not a palindrome.

#### Complexity

```text
Time:  O(n)
Space: O(1)
```

Only two pointers and a few variables are used.

This is an important example of reusing an **array pattern on a string**.


### 1.3 Character Counting

Character counting means counting how many times a particular character appears.

Example:

```text
Input:     "banana"
Character: "a"
Output:    3
```

#### Simple traversal

```python
s = "banana"
target = "a"

count = 0

for ch in s:
    if ch == target:
        count += 1

print(count)
```

Output:

```text
3
```

The algorithm examines every character.

#### Complexity

```text
Time:  O(n)
Space: O(1)
```


#### Using `count()`

Python also provides:

```python
s = "banana"

print(s.count("a"))
```

Output:

```text
3
```

For DSA learning, however, it is important to understand the traversal-based implementation rather than relying only on built-in methods.


### 1.4 Frequency Counting

Character counting asks:

> How many times does one particular character occur?

Frequency counting asks:

> How many times does every character occur?

Example:

```text
Input:

"banana"
```

Frequency:

```text
b → 1
a → 3
n → 2
```


#### Using a dictionary

```python
s = "banana"

freq = {}

for ch in s:
    if ch in freq:
        freq[ch] += 1
    else:
        freq[ch] = 1

print(freq)
```

Output:

```text
{'b': 1, 'a': 3, 'n': 2}
```

A shorter version is:

```python
s = "banana"

freq = {}

for ch in s:
    freq[ch] = freq.get(ch, 0) + 1

print(freq)
```

The idea is:

```text
character → frequency
```

This is the foundation of many string problems.


#### Using an array for lowercase letters

If the problem guarantees only lowercase English letters, we can use an array of size `26`.

```python
s = "banana"

freq = [0] * 26

for ch in s:
    index = ord(ch) - ord('a')
    freq[index] += 1

print(freq)
```

Conceptually:

```text
index 0 → a
index 1 → b
index 2 → c
...
index 25 → z
```

For `"banana"`:

```text
a → 3
b → 1
n → 2
```

This approach is useful when the character set is fixed and small.


#### Complexity

For a string of length `n`:

```text
Time:  O(n)
```

Using a fixed-size frequency array:

```text
Auxiliary Space: O(1)
```

because the array always has size `26`.

Using a dictionary:

```text
Space: O(k)
```

where `k` is the number of distinct characters.


### 1.5 String Searching

String searching means finding whether a character or substring exists inside another string.

For example:

```text
Text:    "hello world"
Pattern: "world"
```

The pattern occurs starting at index `6`.


#### Searching for a character

```python
s = "hello"

target = "l"

for i in range(len(s)):
    if s[i] == target:
        print("Found at index:", i)
        break
```

Output:

```text
Found at index: 2
```


#### Searching for a substring

Python provides:

```python
s = "hello world"

print("world" in s)
```

Output:

```text
True
```

We can also use:

```python
s.find("world")
```

which returns:

```text
6
```

If the substring does not exist:

```python
s.find("python")
```

returns:

```text
-1
```


#### Basic manual substring search

Suppose:

```text
Text:
"abcdef"

Pattern:
"cde"
```

We can check every possible starting position.

```python
text = "abcdef"
pattern = "cde"

n = len(text)
m = len(pattern)

found = False

for i in range(n - m + 1):

    match = True

    for j in range(m):
        if text[i + j] != pattern[j]:
            match = False
            break

    if match:
        found = True
        print("Found at index:", i)
        break

print(found)
```

Output:

```text
Found at index: 2
True
```

This is the basic idea behind **naive pattern matching**, which we will study formally later.


### 1.6 Basic String Transformations

String transformation means producing a new string by modifying, removing, or changing characters according to some rule.

Examples:

```text
"hello" → "HELLO"
"hello" → "Hello"
"abcabc" → "abc"
"12345" → "123"
```

#### Changing case

```python
s = "Hello World"

print(s.lower())
print(s.upper())
```

Output:

```text
hello world
HELLO WORLD
```


#### Replacing characters

```python
s = "banana"

result = s.replace("a", "x")

print(result)
```

Output:

```text
bxnxnx
```

The original string is not modified.


#### Removing characters

For example, remove spaces:

```python
s = "hello world"

result = s.replace(" ", "")

print(result)
```

Output:

```text
helloworld
```


#### Transforming using traversal

We can also build a result according to our own logic.

For example, convert lowercase letters to uppercase:

```python
s = "hello"

result = []

for ch in s:
    result.append(ch.upper())

result = "".join(result)

print(result)
```

Output:

```text
HELLO
```

The general pattern is:

```text
Input string
     ↓
Traverse characters
     ↓
Apply transformation
     ↓
Store transformed characters
     ↓
Construct result string
```

This pattern appears frequently in DSA.


## 2. Pattern Matching Algorithms

Pattern matching is the problem of finding a pattern string inside a larger text string.

Given:

```text
Text:
"ABABDABACDABABCABAB"

Pattern:
"ABABCABAB"
```

we want to find where the pattern occurs.

The basic question is:

> Does the pattern occur in the text, and if so, at which positions?

We will study increasingly efficient algorithms.


### 2.1 Naive Pattern Matching

The naive approach is straightforward:

> Try matching the pattern at every possible position in the text.

Suppose:

```text
Text:    ABCABC
Pattern: BCA
```

We start at index `0`:

```text
ABCABC
BCA
```

Mismatch.

Move to index `1`:

```text
ABCABC
 BCA
```

Match:

```text
B == B
C == C
A == A
```

Therefore the pattern occurs at index `1`.


#### Algorithm

```python
def naive_search(text, pattern):

    n = len(text)
    m = len(pattern)

    for i in range(n - m + 1):

        j = 0

        while j < m and text[i + j] == pattern[j]:
            j += 1

        if j == m:
            return i

    return -1
```

Example:

```python
text = "ABCABC"
pattern = "BCA"

print(naive_search(text, pattern))
```

Output:

```text
1
```


#### How it works

For every possible starting index `i`:

```text
i
↓
Text:    A B C A B C
Pattern: B C A
```

Compare:

```text
text[i]     with pattern[0]
text[i + 1] with pattern[1]
text[i + 2] with pattern[2]
...
```

If a mismatch occurs, shift the pattern by one position and try again.


#### Complexity

There can be approximately:

```text
n - m + 1
```

possible starting positions.

At each position, we may compare up to `m` characters.

Therefore:

```text
Worst-case Time: O(n × m)
```

If:

```text
n = text length
m = pattern length
```

then:

```text
O(nm)
```

Auxiliary space:

```text
O(1)
```


### 2.2 KMP Algorithm

KMP stands for:

**Knuth-Morris-Pratt**

The problem with naive matching is that after a mismatch, it throws away information about characters that have already matched.

KMP avoids unnecessary comparisons.

The key idea is:

> Use information from the pattern itself to determine how far we can shift the pattern after a mismatch.


#### Example

Consider:

```text
Text:
ABABDABABCABAB

Pattern:
ABABCABAB
```

Suppose several characters match and then a mismatch occurs.

The naive algorithm may restart from the next position.

KMP asks:

> Did the part we already matched contain a prefix that is also a suffix?

If yes, we can reuse that information.


#### Prefix and suffix

For:

```text
"ABAB"
```

Prefixes:

```text
A
AB
ABA
```

Suffixes:

```text
B
AB
BAB
```

The longest prefix that is also a suffix is:

```text
"AB"
```

Length:

```text
2
```

This information is stored in the **LPS array**.


#### LPS

LPS means:

**Longest Proper Prefix which is also a Suffix**

For:

```text
Pattern: ABAB
```

the LPS array is:

```text
[0, 0, 1, 2]
```

Meaning:

```text
A     → 0
AB    → 0
ABA   → 1
ABAB  → 2
```

The LPS array allows KMP to skip unnecessary comparisons.


#### KMP structure

KMP has two main phases:

```text
Pattern
   ↓
Build LPS array
   ↓
Use LPS during text matching
```

The LPS construction itself takes:

```text
O(m)
```

The actual matching takes:

```text
O(n)
```

Therefore total complexity is:

```text
O(n + m)
```

Auxiliary space:

```text
O(m)
```

because of the LPS array.


### 2.3 Z Algorithm

The Z algorithm is another linear-time string pattern matching algorithm.

It constructs a **Z-array**.

For a string:

```text
s
```

`Z[i]` represents:

> The length of the longest substring starting at index `i` that matches the prefix of the entire string.


#### Example

Consider:

```text
s = "aaaaa"
```

The prefix is:

```text
"aaaaa"
```

At index `1`:

```text
"aaaa"
```

matches the prefix `"aaaa"`.

Therefore:

```text
Z[1] = 4
```

Similarly:

```text
Z = [0, 4, 3, 2, 1]
```


#### Pattern matching using Z

To search for:

```text
Pattern: "abc"
Text:    "xabcabc"
```

we can construct:

```text
pattern + separator + text
```

For example:

```text
"abc$xabcabc"
```

Then calculate the Z-array.

Whenever:

```text
Z[i] == len(pattern)
```

the pattern occurs at that position in the text.


#### Z-box

The efficient Z algorithm maintains a window:

```text
[L, R]
```

called the Z-box.

It represents a region that is already known to match a prefix of the string.

This allows the algorithm to avoid recomputing matches unnecessarily.


#### Complexity

Building the Z-array:

```text
Time:  O(n)
Space: O(n)
```

Pattern matching can therefore be performed in:

```text
O(n + m)
```

where:

```text
n = text length
m = pattern length
```

Like KMP, the Z algorithm achieves linear-time pattern matching.


### 2.4 Rabin-Karp Algorithm

Rabin-Karp uses **hashing** to search for a pattern.

Instead of comparing every character at every position, we calculate a numeric hash for:

- The pattern
- Each text window of the same length

If the hashes are different:

```text
hash(pattern) != hash(window)
```

then the strings definitely do not match.

If the hashes are equal, we may have a match.


#### Basic idea

Suppose:

```text
Text:
ABCDE

Pattern:
BCD
```

The pattern has length `3`.

We calculate:

```text
hash("BCD")
```

Then calculate:

```text
hash("ABC")
hash("BCD")
hash("CDE")
```

If:

```text
hash("BCD") == hash(pattern)
```

we have a potential match.


#### Why hashing helps

Without hashing, we may compare:

```text
pattern character-by-character
```

for every window.

With hashing, we can first compare:

```text
numeric hash values
```

This can quickly eliminate most windows.


#### Rolling hash

The important optimization is that we don't calculate every window's hash from scratch.

Consider:

```text
Text: ABCDE
```

Windows of length `3`:

```text
ABC
BCD
CDE
```

When moving:

```text
ABC → BCD
```

we remove:

```text
A
```

and add:

```text
D
```

So the next hash can be calculated from the previous hash.

This is called a:

> **Rolling Hash**


#### Rabin-Karp structure

The general process is:

```text
Pattern
   ↓
Calculate hash

Text
   ↓
Calculate first window hash
   ↓
Slide window
   ↓
Update hash
   ↓
Compare hashes
   ↓
Verify characters if hashes match
```

#### Complexity

With a good rolling hash:

```text
Average-case Time: O(n + m)
```

The worst case can degrade because different strings can have the same hash.

Therefore Rabin-Karp is commonly described as:

```text
Average: O(n + m)
Worst:   O(nm)
```

Space:

```text
O(1)
```

apart from implementation details.


### Algorithm Comparison

The main string-searching algorithms we have covered can be compared as follows:

| Algorithm | Main Idea | Time | Extra Space |
|---|---|---:|---:|
| Naive | Try every position | O(nm) worst case | O(1) |
| KMP | Use LPS/prefix information | O(n + m) | O(m) |
| Z Algorithm | Use Z-array/prefix matches | O(n + m) | O(n + m) |
| Rabin-Karp | Compare rolling hashes | O(n + m) average | O(1) |

Where:

```text
n = text length
m = pattern length
```

## 3. String Hashing

String hashing converts a string into a numeric value called a **hash**.

The purpose is to allow strings or substrings to be compared efficiently.

The fundamental idea is:

```text
String
   ↓
Hash function
   ↓
Number
```

For example, conceptually:

```text
"abc" → 12345
```

The actual hash function determines how the number is calculated.

A simple conceptual hash might assign values to characters:

```text
a → 1
b → 2
c → 3
...
```

and combine them.

For example:

```text
"abc"
```

could conceptually become:

```text
1 + 2 + 3 = 6
```

But this is a poor hash because:

```text
"abc" → 6
"cba" → 6
```

Different strings produce the same value.

This is called a **collision**.

A better string hash usually considers character positions.

### 3.1 Polynomial Hashing

A common technique is polynomial hashing.

For:

```text
s = "abc"
```

we can assign:

```text
a → 1
b → 2
c → 3
```

and calculate something like:

```text
1 × p² + 2 × p¹ + 3 × p⁰
```

where `p` is a chosen base.

In practical implementations, we also use a modulus:

```text
hash(s) = Σ value(s[i]) × p^i mod M
```

The exact choice of base and modulus matters for reducing collisions.


#### Why hashing is useful

Suppose we want to compare two long strings.

Without hashing:

```text
Compare character 0
Compare character 1
Compare character 2
...
```

This can take:

```text
O(n)
```

for each comparison.

Hashing allows us to compare their hash values first:

```text
hash(A) == hash(B)
```

If the hashes differ:

```text
A != B
```

If the hashes are equal:

```text
A may equal B
```

The word **may** is important.

Equal hashes do not mathematically guarantee equal strings unless the hash function has a collision-free mapping over the relevant input set.


### 3.2 Rolling Hash

A rolling hash allows us to update the hash when a window moves instead of recalculating the entire hash.

Consider:

```text
Text:

ABCDE
```

and a window size of `3`.

The windows are:

```text
ABC
BCD
CDE
```

Suppose we already know:

```text
hash("ABC")
```

When moving to:

```text
"BCD"
```

we:

1. Remove the contribution of `A`
2. Shift the remaining contributions
3. Add the contribution of `D`

Conceptually:

```text
ABC
 ↓
BCD

remove A
shift BC
add D
```

This allows the next hash to be computed efficiently.


#### Why rolling hash matters

Without rolling hash:

```text
ABC → calculate hash from scratch
BCD → calculate hash from scratch
CDE → calculate hash from scratch
```

With rolling hash:

```text
ABC → calculate
BCD → update previous hash
CDE → update previous hash
```

This is what makes Rabin-Karp efficient.


#### General rolling-hash idea

For a window:

```text
s[i ... i + m - 1]
```

we calculate:

```text
H
```

When moving to:

```text
s[i + 1 ... i + m]
```

we update `H` using:

```text
old hash
- outgoing character
+ incoming character
```

with the appropriate positional adjustments.

The exact formula depends on the chosen hash function.


### 3.3 Hash Collisions

A **hash collision** occurs when two different strings produce the same hash.

For example:

```text
String A → hash 123
String B → hash 123
```

even though:

```text
A != B
```

This is possible because there are usually far more possible strings than available hash values.

#### Why collisions matter

Suppose:

```text
hash(pattern) == hash(window)
```

We might conclude:

```text
pattern == window
```

But that is not guaranteed.

The two strings could be different but have the same hash.

Therefore Rabin-Karp commonly performs a final character-by-character verification after a hash match.

Conceptually:

```text
Hash different?
    ↓
Definitely not equal

Hash same?
    ↓
Possibly equal
    ↓
Verify characters
```


#### Reducing collisions

We can reduce the probability of collisions by choosing a good hash function.

Common techniques include:

- A large modulus
- A suitable base
- Multiple independent hashes

Using two hashes:

```text
hash1(s)
hash2(s)
```

makes accidental collisions much less likely.

The important point is:

> Hashing gives a fast probabilistic comparison unless the hash function and domain provide stronger guarantees.

