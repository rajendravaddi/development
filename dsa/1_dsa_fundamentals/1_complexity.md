# Complexity

## Time & Space Complexities

### 1. Time Complexity
Time complexity describes how the number of operations performed by an algorithm grows as the input size increases.

**Example :**
```py
for i in range(n):
    print(i)
```

The loop runs `n` times

**Time Complexity :** `O(n)`

### 2. Space Complexity

Space complexity describes how the total memory required by an algorithm grows as the input size increases.

**Example:**
```py
new_array = []

for x in arr:
    new_array.append(x)
```

If `arr` contains `n` elements, `new_array` can also contain `n` elements.

**Space complexity:** `O(n)`

### 3. Auxiliary Space

Auxiliary space is the extra memory used by an algorithm, excluding the memory occupied by the input.

**Example:**
```py
def find_max(arr):
    maximum = arr[0]

    for x in arr:
        maximum = max(maximum, x)

    return maximum
```

The input `arr` already exists.

The algorithm only creates `maximum`.

Therefore:

Auxiliary space: O(1)

### 4. Input Size

Input size is the amount of data given to an algorithm.

Usually we represent input size using n.

**Examples:**
```
Array with 100 elements → n = 100
String with 50 characters → n = 50
```
For a graph, we often use:
```
V = number of vertices
E = number of edges
```

### 5. Growth Rate

Growth rate describes how quickly the time or memory requirements increase as the input size increases.

**For example:**
```
O(1)       → barely grows
O(log n)   → grows slowly
O(n)       → grows linearly
O(n²)      → grows quickly
O(2ⁿ)      → grows extremely quickly
```

## Cases of Complexity

### 1. Best-Case Complexity

Best-case complexity describes the minimum amount of work an algorithm needs for an input of size `n`.

**Example:** Linear search:
```py
arr = [10, 20, 30, 40]
```
Searching for 10 finds it immediately.

Best case: `O(1)`

### 2. Average-Case Complexity

Average-case complexity describes the expected amount of work an algorithm performs for inputs of size n.

**Example:** Linear search:
```
Target at position 1 → 1 comparison
Target at position 2 → 2 comparisons
Target at position 3 → 3 comparisons
...
```

On average, we may inspect roughly half the elements.

Average case: `O(n)`

### 3. Worst-Case Complexity

Worst-case complexity describes the maximum amount of work an algorithm may need for an input of size n.

**Example:** Linear Search
```py
arr = [10, 20, 30, 40]
```
Searching for 40 requires checking every element.

Worst case: `O(n)`

### 4. Amortized Complexity

Amortized complexity describes the average cost of operations over a sequence of operations, even when some individual operations are expensive.

**Example:** Dynamic array/list:

```
arr.append(x)
```

Most appends are approximately: `O(1)`

Occasionally, the array needs to resize and copy elements: `O(n)`

But over many append operations, the amortized cost is: `O(1)`


## Complexity Classes
### 1. O(1) — Constant Time

`O(1)` means the algorithm takes a constant amount of work regardless of input size.

**Example:**
```
arr[0]
```
Accessing one array element:
```
O(1)
```

### 2. O(log n) — Logarithmic Time

`O(log n)` means the amount of work increases logarithmically as the input grows.

**Example:** Binary search
```
1000 elements
↓
500
↓
250
↓
125
↓
...
```

Because the search space is repeatedly divided, its complexity is:
```
O(log n)
```

### 3. O(n) — Linear Time

`O(n)` means the amount of work grows directly in proportion to the input size.

**Example:**
```py
for x in arr:
    print(x)
```

If there are `n` elements, the loop runs `n` times.

Complexity: `O(n)`

### 4. O(n log n) — Linearithmic Time

`O(n log n)` means the work grows approximately as `n × log n`.

**Common example:** Merge sort
```
Divide → log n levels
Process → n elements per level
```

Therefore: `O(n log n)`

### 5. O(n²) — Quadratic Time

`O(n²)` means the work grows approximately as `n × n`.

**Example:**
```py
for i in range(n):
    for j in range(n):
        print(i, j)
```

Number of operations: `n × n = n²`

Complexity: `O(n²)`

### 6. O(n³) — Cubic Time

`O(n³)` means the work grows approximately as `n × n × n`.

**Example:**
```py
for i in range(n):
    for j in range(n):
        for k in range(n):
            print(i, j, k)
```
Operations: `n × n × n = n³`

Complexity: `O(n³)`

### 7. O(2ⁿ) — Exponential Time

`O(2ⁿ)` means the amount of work approximately doubles whenever the input size increases by one.

**Example:**

Generating all subsets of n elements.

For: `n = 3` there are: `2³ = 8` subsets.

For: `n = 10` there are: `2¹⁰ = 1024` subsets.

Complexity: `O(2ⁿ)`

### 8. O(n!) — Factorial Time

`O(n!)` means the amount of work grows according to the factorial of the input size.

Factorial means:
```
n! = n × (n-1) × (n-2) × ... × 1
```

**For example:**
```
3! = 3 × 2 × 1 = 6

5! = 5 × 4 × 3 × 2 × 1 = 120

10! = 3,628,800
```

A common example is generating all permutations of n elements.

For: `[A, B, C]` there are:  `3! = 6` possible arrangements.

Complexity: `O(n!)`

## Order

The easiest order to memorize
```
O(1)
 ↓
O(log n)
 ↓
O(n)
 ↓
O(n log n)
 ↓
O(n²)
 ↓
O(n³)
 ↓
O(2ⁿ)
 ↓
O(n!)
```

**General rule:** the further down this list you go, the faster the complexity grows as n becomes large.