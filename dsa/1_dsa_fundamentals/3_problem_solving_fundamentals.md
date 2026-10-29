# Problem-Solving Fundamentals
## 1. Understand Constraints

Constraints are the limits placed on the input of a problem, such as the *maximum size of an array* or *the range of values*.

The constraints help you decide which algorithm is fast enough.

**Example**

Suppose a problem says:

`1 ≤ n ≤ 100`

An `O(n²)` solution might be completely reasonable:

`100² = 10,000`

But if:

`1 ≤ n ≤ 100,000`

then:

`100,000² = 10,000,000,000`

An `O(n²)` solution may be too slow.

You might need something closer to:

`O(n)`

or:

`O(n log n)`

**Key idea**

Always look at the constraints before choosing an algorithm.

## 2. Brute-Force Approach

Brute force is a straightforward approach that tries all possible solutions or directly follows the problem statement without using advanced optimization.

**Example**

Problem:

Find whether an array contains two numbers whose sum is `10`.

Array: `[2, 7, 3, 8]`

A brute-force solution checks every pair:
```
2 + 7
2 + 3
2 + 8
7 + 3
7 + 8
3 + 8
```
If one equals `10`, we have the answer.

This might take: `O(n²)`

**Why brute force is useful**

Brute force is often your starting point.

You can:

- Understand the problem.
- Write a simple correct solution.
- Analyze its complexity.
- Look for ways to optimize it.

## 3. Optimization

Optimization is the process of improving an algorithm so that it uses less time, less memory, or both.

**Example**

Suppose we want to find two numbers that add to 10.

*Brute force*

Check every pair: `O(n²)`

*Optimized approach*

Use a set to remember numbers we've already seen.
```py
seen = set()

for x in arr:
    if 10 - x in seen:
        return True

    seen.add(x)
```
Now we can solve it in approximately:

Time:  `O(n)`

Space: `O(n)`

We traded extra memory for faster execution.

## 4. Time/Space Trade-Offs

A time/space trade-off means using more memory to reduce execution time, or using less memory at the cost of more execution time.

**Example**

Suppose you want to check whether a number exists in an array.

**Approach 1:** Search directly

Time:  `O(n)`

Space: `O(1)`

**Approach 2:** Put values into a set

Time:  `O(1)` average lookup

Space: `O(n)`

The second approach uses more memory but can make repeated lookups much faster.

**Key idea**

More memory → potentially less time

Less memory  → potentially more time

There isn't always one universally best choice.

## 5. Edge Cases

Edge cases are unusual, extreme, or boundary inputs that can cause an algorithm to behave differently or incorrectly.

**Example**

Suppose:
```py
def get_first(arr):
    return arr[0]
```
What happens if:
```
arr = []
```
There is no first element.

That's an edge case.

**Common edge cases**

When solving problems, check for:

- Empty array
- One element
- Two elements
- Very large input
- Very small input
-Duplicate values
- Negative values
- Zero
- Already sorted input
- Reverse-sorted input
- All values equal
- Target doesn't exist

**Key idea**

A solution shouldn't only work for the "normal" example. It should work for valid inputs at the boundaries too.

## 6. Invariants

An invariant is a condition or property that remains true throughout an algorithm or during every iteration of a process.

**Example:** Finding the maximum
```py
maximum = arr[0]

for x in arr:
    if x > maximum:
        maximum = x
```
After processing each element, this remains true:

maximum contains the largest value seen so far.

That's the invariant.

**Why invariants matter**

They help you:

- Understand algorithms
- Design algorithms
- Prove correctness
- Debug code

## 7. Dry Runs

A dry run is manually executing an algorithm step by step using a specific input without actually running the program.

**Here :**
- You pretend to be the computer.
- You track things like:
  - Variables
  - Loop counters
  - Array contents
  - Conditions
  - Output

**Why dry runs matter**

They help you:

- Find bugs
- Understand algorithms
- Understand loops
- Understand recursion
- Verify your logic
- Prepare for coding interviews

## 8. Choosing the Right Data Structure

Choosing the right data structure means selecting a structure that makes the required operations efficient.

Different data structures are good at different things.

**Ask:** What operations do I need to perform frequently?

**Example**

Suppose you need to check whether an item exists quickly.

A: `Set` is often a good choice because membership lookup is approximately: `O(1)` average

Instead of repeatedly searching a list: `O(n)`

**Common choices**

Need|Common choice
---|---
Ordered collection |Array/List
Fast membership checking | Set
Key → value mapping | Dictionary/Hash Map
First-in-first-out | Queue
Last-in-first-out | Stack
Hierarchical data | Tree
Relationships/networks | Graph
Priority-based retrieval | Heap/Priority Queue

**Key idea**

*Don't ask:* "Which data structure is the best?"

*Ask:* "Which data structure is best for the operations I need?"

## 9. Proving Correctness — Basic Level

Proving correctness means showing that an algorithm produces the correct result for every valid input.

It's not enough to say: "It worked on my examples."

You want to understand why it will always work.

**Example**

Consider:
```py
def find_max(arr):
    maximum = arr[0]

    for x in arr:
        if x > maximum:
            maximum = x

    return maximum
```
We can reason about it using three simple steps.

**Step 1:** Before the loop

Initially:

`maximum = arr[0]`

So `maximum` is the largest value among the elements we've considered so far.

**Step 2:** During the loop

For every new value x:
```py
if x > maximum:
    maximum = x
```
If `x` is larger, we update `maximum`.

Otherwise, the existing `maximum` remains correct.

Therefore:

`maximum` always contains the largest value seen so far.

**Step 3:** After the loop

Every element has been considered.

Therefore:

`maximum` contains the largest element in the entire array.

So the algorithm is correct.

## A Simple Problem-Solving Process

When you get a DSA problem, you can follow this process:
```
1. Understand the problem
        ↓
2. Read the constraints
        ↓
3. Identify edge cases
        ↓
4. Think of a brute-force solution
        ↓
5. Check its time and space complexity
        ↓
6. Look for optimization opportunities
        ↓
7. Choose suitable data structures
        ↓
8. Write the algorithm
        ↓
9. Dry run it
        ↓
10. Check correctness
        ↓
11. Analyze final time & space complexity
```

Think of DSA problem solving as:
```
Constraints → Brute Force → Analyze → Optimize → Choose Data Structure → Handle Edge Cases → Dry Run → Prove Correctness
```
Once you become comfortable with this process, learning individual algorithms becomes much easier.