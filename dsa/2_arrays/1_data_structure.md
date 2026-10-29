# Arrays

An **array** is a data structure used to store multiple values in a sequence.

The key idea is:

> **An array stores elements in a logical sequence, and each element can be accessed using its position (index).**

For example:

```text
Array:
[10, 20, 30, 40, 50]

Index:
  0   1   2   3   4
```

Here:

* `10` is at index `0`
* `20` is at index `1`
* `30` is at index `2`
* `50` is at index `4`

Arrays are one of the most fundamental data structures because many other data structures and algorithms are built around them.


## Types

### Static Arrays

A **static array** has a fixed size.

When the array is created, its capacity is determined and normally cannot be changed.

For example, suppose we create an array capable of storing 5 integers:

```text
[10, 20, 30, 40, 50]
```

Its capacity is:

```text
5
```

If we want to add another element:

```text
60
```

there is no free position.

We would need to create a larger array and copy the existing elements into it.

#### Memory representation

Conceptually:

```text
Array:
[10] [20] [30] [40] [50]
  ↓    ↓    ↓    ↓    ↓
Memory locations
```

If each integer occupies 4 bytes and the first element starts at address `1000`:

```text
Index  Address
  0     1000
  1     1004
  2     1008
  3     1012
  4     1016
```

This contiguous layout is what makes array indexing very fast.

#### Advantages

* Fast random access
* Simple structure
* Memory-efficient for fixed-size data
* Good cache locality

#### Disadvantages

* Fixed capacity
* Insertion can require shifting elements
* Deletion can require shifting elements


### Dynamic Arrays

A **dynamic array** can grow when more space is needed.

For example:

```text
Initial capacity = 4

[10, 20, 30, 40]
```

If we add:

```text
50
```

and the array is full, a larger memory block can be allocated.

Conceptually:

```text
Old array:

[10, 20, 30, 40]


New larger array:

[10, 20, 30, 40, 50, _, _, _]
```

The existing elements are copied into the new storage.

A common strategy is to increase capacity by a factor such as 2:

```text
Capacity:

4 → 8 → 16 → 32 → ...
```

This avoids allocating new memory every time an element is added.

#### Why not increase by only 1?

Suppose we started with capacity 4 and increased it by one every time:

```text
4 → 5 → 6 → 7 → 8 → ...
```

Every expansion would require copying the entire array.

That could make repeated insertion expensive.

With geometric growth:

```text
4 → 8 → 16 → 32
```

expansions happen much less frequently.

This gives dynamic arrays an **amortized O(1)** append operation.


### Python Lists

Python's `list` behaves like a **dynamic array**.

For example:

```python
numbers = [10, 20, 30, 40]
```

You can add elements:

```python
numbers.append(50)
```

Result:

```python
[10, 20, 30, 40, 50]
```

You don't normally need to specify the capacity yourself.

Python manages the underlying storage automatically.

#### Common Python list operations

```python
numbers = [10, 20, 30]
```

Access:

```python
numbers[0]
```

Result:

```text
10
```

Append:

```python
numbers.append(40)
```

Insert:

```python
numbers.insert(1, 15)
```

Remove:

```python
numbers.remove(20)
```

Delete by index:

```python
del numbers[1]
```

Update:

```python
numbers[0] = 100
```

Length:

```python
len(numbers)
```

## Indexing

**Indexing** means accessing an element using its position.

Consider:

```text
[10, 20, 30, 40, 50]
```

Indexes:

```text
  0   1   2   3   4
  ↓   ↓   ↓   ↓   ↓
[10, 20, 30, 40, 50]
```

So:

```python
arr[0]  # 10
arr[2]  # 30
arr[4]  # 50
```

### Why is indexing O(1)?

Arrays store elements at known memory offsets.

Conceptually:

```text
address(element) =
    base_address + index × element_size
```

For example:

```text
base address = 1000
element size = 4 bytes
index = 3
```

Then:

```text
address = 1000 + (3 × 4)
        = 1012
```

The computer can calculate this directly.

It does not need to examine:

```text
10 → 20 → 30 → 40
```

to reach the fourth element.

Therefore:

```text
Array indexing = O(1)
```


## Traversal

**Traversal** means visiting every element of an array.

Example:

```python
numbers = [10, 20, 30, 40, 50]

for number in numbers:
    print(number)
```

Output:

```text
10
20
30
40
50
```

If an array contains `n` elements, traversal visits all `n` elements.

Therefore:

```text
Time complexity = O(n)
```

### Index-based traversal

You can also traverse using indexes:

```python
for i in range(len(numbers)):
    print(numbers[i])
```

Conceptually:

```text
i = 0 → numbers[0]
i = 1 → numbers[1]
i = 2 → numbers[2]
...
```


## Insertion

**Insertion** means adding a new element.

The complexity depends on where the element is inserted.

### Insertion at the end

Suppose:

```text
[10, 20, 30, 40]
```

Add `50`:

```text
[10, 20, 30, 40, 50]
```

If there is free capacity, this can be:

```text
O(1)
```

For a dynamic array, append is **amortized O(1)**.

Occasionally, resizing requires copying elements, making that particular operation:

```text
O(n)
```


### Insertion at the beginning

Suppose:

```text
[10, 20, 30, 40]
```

Insert `5` at index `0`.

The existing elements need to move:

```text
Before:

[10, 20, 30, 40]

After:

[5, 10, 20, 30, 40]
```

Conceptually:

```text
40 → move right
30 → move right
20 → move right
10 → move right
5  → insert
```

If there are `n` elements, potentially all `n` elements must move.

Therefore:

```text
O(n)
```


### Insertion in the middle

For:

```text
[10, 20, 30, 40, 50]
```

insert `25` at index `2`:

```text
[10, 20, 25, 30, 40, 50]
```

Elements after index `2` must shift right.

Therefore:

```text
Time complexity = O(n)
```

in the general case.


## Deletion

**Deletion** means removing an element.

### Delete from the end

```text
[10, 20, 30, 40, 50]

remove 50

[10, 20, 30, 40]
```

No other elements need to move.

Therefore:

```text
O(1)
```


### Delete from the beginning

```text
[10, 20, 30, 40, 50]
```

Remove `10`.

The remaining elements shift left:

```text
20 → move left
30 → move left
40 → move left
50 → move left
```

Result:

```text
[20, 30, 40, 50]
```

Therefore:

```text
O(n)
```

### Delete from the middle

For:

```text
[10, 20, 30, 40, 50]
```

delete `30`:

```text
[10, 20, 40, 50]
```

Elements after `30` must shift left.

Therefore, in the general case:

```text
O(n)
```


## Updating

**Updating** means replacing an existing element.

For example:

```python
numbers = [10, 20, 30, 40]

numbers[2] = 100
```

Result:

```text
[10, 20, 100, 40]
```

Because the element can be directly accessed using its index:

```text
numbers[2]
```

the operation takes:

```text
O(1)
```

This is another major advantage of arrays.


## Searching

**Searching** means finding an element in an array.

There are two important cases.

### Linear Search

If the array is unsorted:

```text
[10, 50, 20, 80, 30]
```

To find `80`, we may need to check:

```text
10 → no
50 → no
20 → no
80 → yes
```

In the worst case, we may have to examine every element.

Therefore:

```text
Time complexity = O(n)
```

Example:

```python
numbers = [10, 50, 20, 80, 30]

target = 80

for i in range(len(numbers)):
    if numbers[i] == target:
        print("Found at index", i)
        break
```


### Binary Search

If the array is **sorted**, we can use binary search.

Example:

```text
[10, 20, 30, 40, 50, 60, 70, 80]
```

Instead of checking every element, binary search repeatedly divides the search range in half.

For example:

```text
Search for 70

[10, 20, 30, 40, 50, 60, 70, 80]
                    ↑
                 middle
```

Depending on the comparison, half of the elements can be discarded.

Its time complexity is:

```text
O(log n)
```

So:

```text
Unsorted array + linear search → O(n)

Sorted array + binary search   → O(log n)
```


## Memory Layout

One of the most important characteristics of traditional arrays is **contiguous memory**.

Consider:

```text
[10, 20, 30, 40]
```

Conceptually, the elements might be stored like:

```text
Memory

1000 → 10
1004 → 20
1008 → 30
1012 → 40
```

There are no gaps between the elements in this simplified model.

If each element occupies `4` bytes:

```text
Address of arr[i]
=
base address + i × 4
```

For a general element size `S`:

```text
Address(arr[i])
=
base + i × S
```

This is why direct indexing is efficient.
