# Array Algorithms

## 1. Linear Search

**Linear search** checks elements one by one until the target is found or the array ends.

Consider:

```text
[10, 25, 30, 45, 60]
```

Suppose we want to find `45`.

We check:

```text
10 → not 45
25 → not 45
30 → not 45
45 → found
```

### Algorithm

```text
Start from index 0.

For every element:
    If element == target:
        return its index

If the loop finishes:
    target does not exist
```

### Python

```python
def linear_search(arr, target):
    for i in range(len(arr)):
        if arr[i] == target:
            return i

    return -1
```

Example:

```python
arr = [10, 25, 30, 45, 60]

print(linear_search(arr, 45))
```

Output:

```text
3
```

### Complexity

If there are `n` elements:

| Case         | Complexity |
| ------------ | ---------: |
| Best case    |       O(1) |
| Average case |       O(n) |
| Worst case   |       O(n) |
| Space        |       O(1) |

The best case occurs when the target is the first element.

The worst case occurs when the target is the last element or does not exist.

### When to use

Linear search is useful when:

* The array is unsorted.
* The array is small.
* You only need to search once.
* Sorting the data first would not be worthwhile.


## 2. Binary Search

**Binary search** searches a **sorted** array by repeatedly eliminating half of the remaining search space.

Consider:

```text
[10, 20, 30, 40, 50, 60, 70, 80]
```

Search for:

```text
70
```

Instead of checking every element, we look at the middle.

```text
[10, 20, 30, 40, 50, 60, 70, 80]
              ↑
            middle
```

Middle is `40`.

Since:

```text
70 > 40
```

we know the target cannot be in the left half.

So we discard:

```text
[10, 20, 30, 40]
```

and continue with:

```text
[50, 60, 70, 80]
```

Now the middle is `60`.

```text
70 > 60
```

Discard the left portion again.

We eventually find `70`.

### Algorithm

```text
left = 0
right = n - 1

while left <= right:

    mid = (left + right) // 2

    if arr[mid] == target:
        return mid

    if arr[mid] < target:
        left = mid + 1

    else:
        right = mid - 1

return -1
```

### Python

```python
def binary_search(arr, target):
    left = 0
    right = len(arr) - 1

    while left <= right:
        mid = (left + right) // 2

        if arr[mid] == target:
            return mid

        elif arr[mid] < target:
            left = mid + 1

        else:
            right = mid - 1

    return -1
```

Example:

```python
arr = [10, 20, 30, 40, 50, 60, 70, 80]

print(binary_search(arr, 70))
```

Output:

```text
6
```

### mid - calculation

Calculating `mid` value is the crucial thing in Binary Search.

The following calculation can be valid in python, due to python's dynamic typing.
```text
mid = (left + right) // 2
```

For languages like C and Java, where we declare variables first, the value of mid can overflow for larger values. So we can calculate `mid` using:

```text
mid = left + (right - left) / 2
```

### Why O(log n)?

Every iteration approximately halves the search space:

```text
n
n/2
n/4
n/8
n/16
...
```

After `k` divisions:

```text
n / 2^k = 1
```

Therefore:

```text
2^k = n

k = log₂(n)
```

So binary search is:

```text
O(log n)
```

### Complexity

| Case              | Complexity |
| ----------------- | ---------: |
| Best case         |       O(1) |
| Average case      |   O(log n) |
| Worst case        |   O(log n) |
| Space — iterative |       O(1) |


## 3. Basic Sorting

**Sorting** means arranging elements according to some order.

For example:

```text
Before:

[40, 10, 30, 20]

After:

[10, 20, 30, 40]
```

Sorting is important because many algorithms become easier or faster after data has been sorted.

Three basic sorting algorithms are particularly useful for understanding the fundamentals:

* Bubble sort
* Selection sort
* Insertion sort


### 3.1 Bubble Sort

Bubble sort repeatedly compares neighboring elements and swaps them if they are in the wrong order.

Example:

```text
[5, 3, 4, 1]
```

First comparison:

```text
5 > 3
```

Swap:

```text
[3, 5, 4, 1]
```

Next:

```text
5 > 4
```

Swap:

```text
[3, 4, 5, 1]
```

Next:

```text
5 > 1
```

Swap:

```text
[3, 4, 1, 5]
```

The largest element has "bubbled" to the end.

After additional passes:

```text
[1, 3, 4, 5]
```

#### Python

```python
def bubble_sort(arr):
    n = len(arr)

    for i in range(n):
        swapped = False

        for j in range(0, n - i - 1):
            if arr[j] > arr[j + 1]:
                arr[j], arr[j + 1] = arr[j + 1], arr[j]
                swapped = True

        if not swapped:
            break

    return arr
```

#### Complexity

```text
Worst case: O(n²)
Average:    O(n²)
Best case:  O(n)   # with optimization
Space:      O(1)
```

Bubble sort is mainly useful for learning sorting mechanics.


### 3.2 Selection Sort

Selection sort repeatedly finds the smallest remaining element and puts it in the correct position.

Example:

```text
[40, 10, 30, 20]
```

Find minimum:

```text
10
```

Put it at the beginning:

```text
[10, 40, 30, 20]
```

Now consider:

```text
[40, 30, 20]
```

Minimum is:

```text
20
```

Swap:

```text
[10, 20, 30, 40]
```

#### Python

```python
def selection_sort(arr):
    n = len(arr)

    for i in range(n):
        min_index = i

        for j in range(i + 1, n):
            if arr[j] < arr[min_index]:
                min_index = j

        arr[i], arr[min_index] = arr[min_index], arr[i]

    return arr
```

#### Complexity

```text
Best:    O(n²)
Average: O(n²)
Worst:   O(n²)
Space:   O(1)
```


### 3.3 Insertion Sort

Insertion sort builds the sorted portion one element at a time.

Consider:

```text
[5, 2, 4, 1]
```

Start:

```text
[5]
```

Insert `2`:

```text
[2, 5]
```

Insert `4`:

```text
[2, 4, 5]
```

Insert `1`:

```text
[1, 2, 4, 5]
```

#### Python

```python
def insertion_sort(arr):
    for i in range(1, len(arr)):
        key = arr[i]
        j = i - 1

        while j >= 0 and arr[j] > key:
            arr[j + 1] = arr[j]
            j -= 1

        arr[j + 1] = key

    return arr
```

#### Complexity

```text
Best:    O(n)
Average: O(n²)
Worst:   O(n²)
Space:   O(1)
```

Insertion sort performs particularly well when the array is already nearly sorted.


## 4. Kadane's Algorithm

**Kadane's algorithm** finds the maximum sum of a contiguous subarray.

Example:

```text
[-2, 1, -3, 4, -1, 2, 1, -5, 4]
```

The maximum-sum contiguous subarray is:

```text
[4, -1, 2, 1]
```

Its sum is:

```text
4 + (-1) + 2 + 1 = 6
```

So the answer is:

```text
6
```

### Key idea

At each position, ask:

> Should I extend the current subarray, or start a new subarray here?

Maintain:

```text
current_sum
best_sum
```

For each element:

```text
current_sum = max(element, current_sum + element)

best_sum = max(best_sum, current_sum)
```

### Python

```python
def max_subarray_sum(arr):
    if not arr:
        return 0

    current_sum = arr[0]
    best_sum = arr[0]

    for i in range(1, len(arr)):
        current_sum = max(arr[i], current_sum + arr[i])
        best_sum = max(best_sum, current_sum)

    return best_sum
```

Example:

```python
arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4]

print(max_subarray_sum(arr))
```

Output:

```text
6
```

### Why does it work?

Suppose our current subarray has a negative contribution.

If continuing the existing subarray is worse than starting fresh, we discard the previous portion.

For every element:

```text
Best subarray ending here
=
max(
    current element,
    previous best ending here + current element
)
```

### Complexity

```text
Time:  O(n)
Space: O(1)
```

This is much better than checking every possible subarray, which would require significantly more work.


## 5. Dutch National Flag Algorithm

The **Dutch National Flag algorithm** is a specialized partitioning algorithm for an array containing three categories of values.

The classic example uses:

```text
0, 1, 2
```

For example:

```text
[2, 0, 2, 1, 1, 0]
```

We want:

```text
[0, 0, 1, 1, 2, 2]
```

The important requirement is that we can do this in: `O(n)` time and `O(1)` extra space.


### Three pointers

Maintain three regions:

```text
0 region | 1 region | unknown region | 2 region
```

Use:

```text
low
mid
high
```

Initially:

```text
low = 0
mid = 0
high = n - 1
```

The invariant is:

```text
[0 ... low-1]     → all 0s
[low ... mid-1]   → all 1s
[mid ... high]    → unknown
[high+1 ... n-1]  → all 2s
```

#### Rules

Look at:

```text
arr[mid]
```

##### If it is `0`

Swap it with `arr[low]`.

Then:

```text
low += 1
mid += 1
```

##### If it is `1`

It is already in the correct middle region.

So:

```text
mid += 1
```

##### If it is `2`

Swap it with `arr[high]`.

Then:

```text
high -= 1
```

**Do not increment `mid` in this case.**

Why?

Because the element swapped from the `high` position has not yet been examined.


#### Example

Start:

```text
[2, 0, 2, 1, 1, 0]
 ↑              ↑
mid            high
```

The current element is `2`.

Swap with `high`:

```text
[0, 0, 2, 1, 1, 2]
 ↑        ↑
mid      high
```

Now inspect `arr[mid]` again.

It is `0`.

Move it to the low region.

Continue this process until:

```text
mid > high
```

The final result becomes:

```text
[0, 0, 1, 1, 2, 2]
```

#### Python

```python
def dutch_national_flag(arr):
    low = 0
    mid = 0
    high = len(arr) - 1

    while mid <= high:
        if arr[mid] == 0:
            arr[low], arr[mid] = arr[mid], arr[low]
            low += 1
            mid += 1

        elif arr[mid] == 1:
            mid += 1

        else:  # arr[mid] == 2
            arr[mid], arr[high] = arr[high], arr[mid]
            high -= 1

    return arr
```

Example:

```python
arr = [2, 0, 2, 1, 1, 0]

print(dutch_national_flag(arr))
```

Output:

```text
[0, 0, 1, 1, 2, 2]
```

### Complexity

```text
Time:  O(n)
Space: O(1)
```

Every element is processed a constant number of times.
