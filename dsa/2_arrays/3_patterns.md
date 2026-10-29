# Patterns

Array problems often look different on the surface, but many of them can be solved using a small set of reusable **patterns**.

A pattern is not a specific algorithm. It is a **way of thinking about a class of problems**.

The patterns covered here are:

1. Two Pointers
2. Sliding Window
3. Prefix Sum
4. Difference Array
5. Rotation
6. Fast and Slow Pointers
7. Sorting + Scanning
8. In-Place Manipulation


## 1. Two Pointers

The **Two Pointers** pattern uses two indexes to process an array efficiently.

Instead of repeatedly scanning the entire array, we maintain two positions and move them according to the problem.

The pointers are commonly called:

```text
left
right
```

For example:

```text
[1, 2, 3, 4, 5, 6]
 ↑              ↑
left           right
```

The pointers can move:

* Toward each other
* In the same direction
* At different speeds


### Example: Find a Pair With a Given Sum

Suppose:

```text
arr = [1, 2, 3, 4, 6]
target = 7
```

We want to determine whether two elements add up to `7`.

Because the array is sorted, start with:

```text
[1, 2, 3, 4, 6]
 ↑           ↑
left        right
```

Calculate:

```text
1 + 6 = 7
```

We found the answer.


### What if the sum is too small?

Suppose:

```text
target = 8
```

Initially:

```text
1 + 6 = 7
```

The sum is too small.

Because the array is sorted, increasing `left` gives us a larger value:

```text
[1, 2, 3, 4, 6]
    ↑        ↑
   left     right
```

Now:

```text
2 + 6 = 8
```

Found.


### What if the sum is too large?

Suppose:

```text
target = 5
```

Initially:

```text
1 + 6 = 7
```

The sum is too large.

We need a smaller value, so move `right` left:

```text
[1, 2, 3, 4, 6]
 ↑        ↑
left     right
```

Now:

```text
1 + 4 = 5
```

Found.


### Python

```python
def has_pair_with_sum(arr, target):
    left = 0
    right = len(arr) - 1

    while left < right:
        current_sum = arr[left] + arr[right]

        if current_sum == target:
            return True

        elif current_sum < target:
            left += 1

        else:
            right -= 1

    return False
```

Example:

```python
arr = [1, 2, 3, 4, 6]

print(has_pair_with_sum(arr, 7))
```

Output:

```text
True
```


### Why Two Pointers Are Efficient

A brute-force approach might check every pair:

```text
(1,2)
(1,3)
(1,4)
(1,6)
(2,3)
(2,4)
...
```

That takes:

```text
O(n²)
```

With two pointers, each pointer moves through the array at most once:

```text
O(n)
```

So:

```text
Brute force → O(n²)
Two pointers → O(n)
```

when the problem has the properties that make the technique applicable, such as a sorted array for this pair-sum example.


### Common Two-Pointer Shapes

#### Opposite directions

```text
[1, 2, 3, 4, 5]
 ↑           ↑
left        right
```

Used for:

* Pair problems
* Reversal
* Partitioning
* Comparing elements from both ends

#### Same direction

```text
[1, 2, 3, 4, 5]
 ↑  ↑
slow fast
```

Used for:

* Removing duplicates
* Compressing arrays
* Maintaining a valid region

## 2. Sliding Window

The **Sliding Window** pattern processes a contiguous portion of an array while moving that portion through the array.

Imagine a window placed over part of the array:

```text
[1, 2, 3, 4, 5, 6]
 └───────┘
  window
```

The window moves:

```text
[1, 2, 3, 4, 5, 6]
   └───────┘
```

then:

```text
[1, 2, 3, 4, 5, 6]
      └───────┘
```

then:

```text
[1, 2, 3, 4, 5, 6]
         └───────┘
```

Instead of recalculating everything from scratch, we update the existing window.


### Fixed-Size Sliding Window

Suppose:

```text
arr = [2, 1, 5, 1, 3, 2]
```

Find the maximum sum of any subarray of size `3`.

Possible windows:

```text
[2, 1, 5] → sum = 8
[1, 5, 1] → sum = 7
[5, 1, 3] → sum = 9
[1, 3, 2] → sum = 6
```

Answer:

```text
9
```


#### Efficient Approach

Instead of calculating every window from scratch:

```text
[2, 1, 5] = 8
```

Move the window one position:

```text
[1, 5, 1]
```

Remove the outgoing element:

```text
8 - 2
```

Add the incoming element:

```text
6 + 1 = 7
```

So:

```text
new_sum = old_sum - outgoing + incoming
```


#### Python

```python
def max_sum_subarray(arr, k):
    if len(arr) < k:
        return None

    window_sum = sum(arr[:k])
    max_sum = window_sum

    for right in range(k, len(arr)):
        window_sum += arr[right]
        window_sum -= arr[right - k]

        max_sum = max(max_sum, window_sum)

    return max_sum
```

Example:

```python
arr = [2, 1, 5, 1, 3, 2]

print(max_sum_subarray(arr, 3))
```

Output:

```text
9
```


#### Why Sliding Window Is Efficient

Brute force:

```text
Calculate each window separately
```

can take:

```text
O(n × k)
```

With a sliding window:

```text
Build first window → O(k)

Move window → O(1) per move
```

Total:

```text
O(n)
```


### Variable-Size Sliding Window

The window does not always have a fixed size.

For example:

> Find the smallest contiguous subarray whose sum is at least `7`.

Consider:

```text
[2, 3, 1, 2, 4, 3]
```

We expand the window until its sum reaches the required value.

Then we shrink it from the left while maintaining the condition.

Conceptually:

```text
expand → expand → expand
                  ↓
                valid
                  ↓
              shrink
                  ↓
            smaller valid window
```

This is another important form of Sliding Window.

#### Important condition

The simple variable-size sum technique works particularly naturally when the array contains **non-negative values**. With negative values, the condition can behave differently and the basic shrinking logic may not work.


## 3. Prefix Sum

A **Prefix Sum** stores cumulative sums of an array.

Consider:

```text
arr = [2, 4, 6, 8, 10]
```

Build:

```text
prefix = [2, 6, 12, 20, 30]
```

Because:

```text
prefix[0] = 2

prefix[1] = 2 + 4
          = 6

prefix[2] = 2 + 4 + 6
          = 12

prefix[3] = 2 + 4 + 6 + 8
          = 20

prefix[4] = 2 + 4 + 6 + 8 + 10
          = 30
```


### Why Prefix Sum?

Suppose we repeatedly need the sum between two indexes.

For example:

```text
arr = [2, 4, 6, 8, 10]
```

Find:

```text
sum(1, 3)
```

That means:

```text
4 + 6 + 8 = 18
```

Using the prefix array:

```text
prefix[3] = 20
prefix[0] = 2
```

Therefore:

```text
20 - 2 = 18
```

So:

```text
sum(l, r) = prefix[r] - prefix[l - 1]
```

when `l > 0`.


### Python

A particularly convenient implementation uses a leading zero:

```python
def build_prefix_sum(arr):
    prefix = [0] * (len(arr) + 1)

    for i in range(len(arr)):
        prefix[i + 1] = prefix[i] + arr[i]

    return prefix
```

For:

```text
arr = [2, 4, 6, 8, 10]
```

we get:

```text
prefix = [0, 2, 6, 12, 20, 30]
```

Now a range sum is simply:

```python
def range_sum(prefix, left, right):
    return prefix[right + 1] - prefix[left]
```

Example:

```python
arr = [2, 4, 6, 8, 10]

prefix = build_prefix_sum(arr)

print(range_sum(prefix, 1, 3))
```

Output:

```text
18
```

### Complexity

Building:

```text
O(n)
```

Each range query:

```text
O(1)
```

Extra space:

```text
O(n)
```


## 4. Difference Array

A **Difference Array** is useful when you need to perform many updates over ranges.

Suppose:

```text
arr = [0, 0, 0, 0, 0]
```

We want to add `5` to indexes:

```text
1 through 3
```

The result should be:

```text
[0, 5, 5, 5, 0]
```

Instead of modifying all three positions individually, we record where the change starts and ends.


### Basic Rule

To add `x` to range `[l, r]`:

```text
diff[l] += x
diff[r + 1] -= x
```

For:

```text
l = 1
r = 3
x = 5
```

we do:

```text
diff[1] += 5
diff[4] -= 5
```

So:

```text
diff = [0, 5, 0, 0, -5]
```

Now take the prefix sum:

```text
[0, 5, 5, 5, 0]
```

That is our final array.


### Multiple Updates

Suppose:

```text
arr = [0, 0, 0, 0, 0]
```

Updates:

```text
Add 5 to [1, 3]
Add 2 to [2, 4]
```

Record the changes:

```text
diff = [0, 5, 2, 0, -5]
```

Take the prefix sum:

```text
[0, 5, 7, 7, 2]
```

Final result:

```text
[0, 5, 7, 7, 2]
```

### Python

```python
def apply_range_updates(n, updates):
    diff = [0] * (n + 1)

    for left, right, value in updates:
        diff[left] += value
        diff[right + 1] -= value

    result = [0] * n
    current = 0

    for i in range(n):
        current += diff[i]
        result[i] = current

    return result
```

Example:

```python
updates = [
    (1, 3, 5),
    (2, 4, 2)
]

print(apply_range_updates(5, updates))
```

Output:

```text
[0, 5, 7, 7, 2]
```


### Complexity

For `m` updates and an array of size `n`:

```text
Processing updates → O(m)

Building final array → O(n)

Total → O(n + m)
```

Without the difference-array technique, repeatedly modifying every element in every range can be much more expensive.


## 5. Rotation

**Rotation** means moving elements from one end of an array to the other.

Example:

```text
Original:

[1, 2, 3, 4, 5]
```

Right rotate by `2`:

```text
[4, 5, 1, 2, 3]
```

The last two elements move to the beginning.


### Left Rotation

Left rotate by `2`:

```text
[1, 2, 3, 4, 5]
```

becomes:

```text
[3, 4, 5, 1, 2]
```

The first two elements move to the end.


#### Important Observation

If the array has length `n`, rotating by `n` positions changes nothing.

```text
rotate by n → original array
```

Rotating by more than `n` can be reduced:

```text
k = k % n
```

For example:

```text
n = 5
k = 7

7 % 5 = 2
```

So rotating by `7` is equivalent to rotating by `2`.


### Right Rotation Using Slicing

In Python:

```python
def rotate_right(arr, k):
    if not arr:
        return arr

    k %= len(arr)

    return arr[-k:] + arr[:-k]
```

Example:

```python
arr = [1, 2, 3, 4, 5]

print(rotate_right(arr, 2))
```

Output:

```text
[4, 5, 1, 2, 3]
```

This is easy to understand, but it creates a new array.


### In-Place Rotation

Sometimes the problem requires:

> Rotate the array without using another array.

We can use the **reversal technique**.

For right rotation by `k`:

```text
1. Reverse the entire array
2. Reverse the first k elements
3. Reverse the remaining elements
```

Example:

```text
[1, 2, 3, 4, 5]
```

Rotate right by `2`.

#### Step 1 — Reverse everything

```text
[5, 4, 3, 2, 1]
```

#### Step 2 — Reverse first 2

```text
[4, 5, 3, 2, 1]
```

#### Step 3 — Reverse remaining elements

```text
[4, 5, 1, 2, 3]
```

Done.


#### Python

```python
def reverse_range(arr, left, right):
    while left < right:
        arr[left], arr[right] = arr[right], arr[left]

        left += 1
        right -= 1


def rotate_right_in_place(arr, k):
    n = len(arr)

    if n == 0:
        return

    k %= n

    reverse_range(arr, 0, n - 1)
    reverse_range(arr, 0, k - 1)
    reverse_range(arr, k, n - 1)
```


### Complexity

```text
Time:  O(n)
Space: O(1)
```


## 6. Fast and Slow Pointers

Fast and Slow Pointers use two indexes that move at different speeds.

For arrays, one common use is **removing duplicates from a sorted array in-place**.

Consider:

```text
[1, 1, 2, 2, 3, 4, 4]
```

We want:

```text
[1, 2, 3, 4, ...]
```

The useful values should occupy the beginning of the array.


### Two Pointers

Use:

```text
slow
fast
```

The `fast` pointer scans the array.

The `slow` pointer represents where the next useful value should be placed.

Conceptually:

```text
[1, 1, 2, 2, 3, 4, 4]
 ↑  ↑
slow fast
```

The first value is unique.

Move `fast` forward.

When we discover a new value:

```text
arr[fast] != arr[slow]
```

move `slow` forward and copy the new value there.


#### Python

```python
def remove_duplicates(arr):
    if not arr:
        return 0

    slow = 0

    for fast in range(1, len(arr)):
        if arr[fast] != arr[slow]:
            slow += 1
            arr[slow] = arr[fast]

    return slow + 1
```

Example:

```python
arr = [1, 1, 2, 2, 3, 4, 4]

length = remove_duplicates(arr)

print(arr[:length])
```

Output:

```text
[1, 2, 3, 4]
```


### What Are the Pointers Doing?

The important idea is:

```text
fast → explores
slow → builds the answer
```

For example:

```text
[1, 1, 2, 2, 3, 4, 4]
 ↑        ↑
slow     fast
```

`fast` keeps looking for the next useful element.

`slow` keeps track of where that element belongs.


### Complexity

```text
Time:  O(n)
Space: O(1)
```

## 7. Sorting + Scanning

Sometimes an array problem becomes much easier after sorting.

The pattern is:

```text
Unsorted array
      ↓
    Sort
      ↓
Scan once
      ↓
   Answer
```

Sorting creates useful structure.


### Example: Find Duplicate Values

Consider:

```text
[4, 2, 7, 2, 5, 4]
```

Without sorting, equal values can be far apart.

After sorting:

```text
[2, 2, 4, 4, 5, 7]
```

Now duplicates are next to each other.

So we can scan:

```text
2 == 2 → duplicate
4 == 4 → duplicate
```

### Python

```python
def contains_duplicate(arr):
    arr.sort()

    for i in range(1, len(arr)):
        if arr[i] == arr[i - 1]:
            return True

    return False
```

Example:

```python
arr = [4, 2, 7, 2, 5, 4]

print(contains_duplicate(arr))
```

Output:

```text
True
```

### Example: Find the Longest Consecutive Sequence

Consider:

```text
[100, 4, 200, 1, 3, 2]
```

Sort:

```text
[1, 2, 3, 4, 100, 200]
```

Now scan:

```text
1 → 2 → 3 → 4
```

We can immediately see the consecutive sequence.

The main idea is that sorting places related values next to each other.


### Complexity

If sorting takes:

```text
O(n log n)
```

and scanning takes:

```text
O(n)
```

then the total is:

```text
O(n log n) + O(n)
```

which simplifies to:

```text
O(n log n)
```


### When Is This Pattern Useful?

Think about sorting when:

* Order doesn't matter initially.
* You need to find duplicates.
* You need to compare neighboring values.
* You need consecutive values.
* You need to group similar values.
* Sorting makes relationships between elements obvious.


### Important Trade-off

Sorting changes the array's order.

So ask:

> **"Am I allowed to modify the array or make a copy?"**

If the original ordering matters, you may need another approach.


## 8. In-Place Manipulation

**In-place manipulation** means modifying the existing array instead of creating another array to store the result.

For example:

```text
arr = [1, 2, 3, 4]
```

Instead of creating:

```text
result = [4, 3, 2, 1]
```

we modify `arr` itself:

```text
arr = [4, 3, 2, 1]
```

The goal is usually to use:

```text
O(1)
```

extra space.


### Example: Reverse an Array

Instead of:

```python
result = arr[::-1]
```

we can modify the array directly.

```python
def reverse_in_place(arr):
    left = 0
    right = len(arr) - 1

    while left < right:
        arr[left], arr[right] = arr[right], arr[left]

        left += 1
        right -= 1
```

Example:

```python
arr = [1, 2, 3, 4, 5]

reverse_in_place(arr)

print(arr)
```

Output:

```text
[5, 4, 3, 2, 1]
```

No second array is required.


### Example: Move Zeroes

Consider:

```text
[0, 1, 0, 3, 12]
```

We want:

```text
[1, 3, 12, 0, 0]
```

We can do this in-place using a position pointer.

```python
def move_zeroes(arr):
    insert = 0

    for i in range(len(arr)):
        if arr[i] != 0:
            arr[insert], arr[i] = arr[i], arr[insert]
            insert += 1
```

Example:

```python
arr = [0, 1, 0, 3, 12]

move_zeroes(arr)

print(arr)
```

Output:

```text
[1, 3, 12, 0, 0]
```

### Why In-Place Matters

Suppose:

```text
n = 1,000,000
```

Creating another array containing one million elements requires additional memory.

An in-place algorithm may need only a few variables:

```text
left
right
temporary
```

So:

```text
Extra space = O(1)
```

This can be important when memory constraints are strict.
