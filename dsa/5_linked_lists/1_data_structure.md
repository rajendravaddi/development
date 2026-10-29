# Linked Lists

## 1. Singly Linked List

### 1.1 What Is a Linked List?

A **linked list** is a linear data structure consisting of nodes connected through references. Unlike an array, its nodes do not need to occupy contiguous memory locations.

Each node in a singly linked list contains:

- **Data:** The value stored in the node.
- **Next:** A reference to the next node.

The last node points to `None`.

```text
head
 |
 v
[10 | next] -> [20 | next] -> [30 | None]
```

Here, `head` references the first node. Following the `next` references provides access to the remaining nodes.

### 1.2 Node: Structure and Python Implementation

A **node** is the fundamental building block of a linked list. It stores data and references that connect it to other nodes.

For a singly linked list, define the node class as follows:

```python
class Node:
    def __init__(self, data):
        self.data = data
        self.next = None
```

- `self.data` stores the node's value.
- `self.next` stores a reference to the next node.
- `None` indicates that the node currently has no successor.

Create and connect nodes:

```python
first = Node(10)
second = Node(20)
third = Node(30)

first.next = second
second.next = third
```

The resulting structure is:

```text
first
  |
  v
[10 | next] -> [20 | next] -> [30 | None]
```

#### Understanding Python references

Variables refer to objects rather than containing copies of those objects.

```python
first = Node(10)
second = first

second.data = 99

print(first.data)   # 99
print(second.data)  # 99
print(first is second)  # True
```

Both variables refer to the same node. Changing its data through either reference affects the same object.

In contrast:

```python
first = Node(10)
second = Node(10)

print(first.data == second.data)  # True
print(first is second)            # False
```

These nodes contain equal values but are distinct objects.

**Important distinction:** A node represents one element; a linked list is the complete structure formed by connecting nodes.

### 1.3 Traversing a Singly Linked List

Traversal means visiting nodes sequentially, starting from `head` and following their `next` references.

```python
head = first
current = head

while current is not None:
    print(current.data)
    current = current.next
```

Output:

```text
10
20
30
```

During each iteration:

1. Process the current node.
2. Move to the next node.
3. Stop when `current` becomes `None`.

Assigning `current = current.next` changes the local reference; it does not modify the list's connections.

**Complexity:**
- Time: `O(n)`
- Auxiliary space: `O(1)`

### 1.4 Advantages and Limitations

**Advantages**
- Insertion and deletion can take `O(1)` time when the required references are already available.
- The list can grow or shrink without reallocating one contiguous array.
- Nodes can be rearranged by changing references.

**Limitations**
- Accessing a node by index takes `O(n)` time in the worst case.
- Each node requires additional memory for its reference.
- Searching for a value takes `O(n)` time.
- Traversal is forward-only.

### 1.5 Complexity Summary

| Operation | Time complexity |
|---|---|
| Access the head reference | `O(1)` |
| Access a node by index | `O(n)` worst case |
| Search for a value | `O(n)` |
| Traverse the list | `O(n)` |
| Insert after a known node | `O(1)` |
| Delete after a known predecessor | `O(1)` |
| Append without a tail reference | `O(n)` |
| Append with a maintained tail reference | `O(1)` |

The constant-time insertion and deletion operations assume the necessary node references are already available.


## 2. Doubly Linked List

### 2.1 What Is a Doubly Linked List?

A **doubly linked list** is a linked list in which each node contains references to both the previous and next nodes.

```text
None <- 10 <-> 20 <-> 30 -> None
```

Its nodes have three components:

- `prev`: Reference to the previous node.
- `data`: Stored value.
- `next`: Reference to the next node.

The first node's `prev` is `None`, and the last node's `next` is `None`.

### 2.2 Python Implementation

For a doubly linked list, use this node structure instead of the singly linked node structure defined earlier:

```python
class Node:
    def __init__(self, data):
        self.data = data
        self.prev = None
        self.next = None
```

Create and connect nodes:

```python
first = Node(10)
second = Node(20)
third = Node(30)

first.next = second
second.prev = first

second.next = third
third.prev = second
```

Both directions must be assigned explicitly. Setting `first.next = second` does not automatically set `second.prev = first`.

### 2.3 Forward and Backward Traversal

Forward traversal follows `next`:

```python
current = first

while current is not None:
    print(current.data)
    current = current.next
```

Output:

```text
10
20
30
```

Backward traversal follows `prev` from the last node:

```python
current = third

while current is not None:
    print(current.data)
    current = current.prev
```

Output:

```text
30
20
10
```

**Complexity:** Each traversal takes `O(n)` time and `O(1)` auxiliary space.

### 2.4 Inserting Between Two Nodes

Suppose we want to insert `15` between `10` and `20`.

```python
new_node = Node(15)

new_node.prev = first
new_node.next = second

first.next = new_node
second.prev = new_node
```

Result:

```text
10 <-> 15 <-> 20
```

The four assignments establish both forward and backward connections.

**Time complexity:** `O(1)`, assuming the neighboring nodes are already known.

### 2.5 Deleting a Known Node

Suppose `current` references the node containing `20` in:

```text
10 <-> 20 <-> 30
```

```python
previous_node = current.prev
next_node = current.next

if previous_node is not None:
    previous_node.next = next_node

if next_node is not None:
    next_node.prev = previous_node

current.prev = None
current.next = None
```

Result:

```text
10 <-> 30
```

A complete linked-list implementation must also update `head` or `tail` when deleting a boundary node.

**Time complexity:** `O(1)` when the node is already known.

### 2.6 Advantages and Limitations

**Advantages**
- Supports traversal in both directions.
- Makes deletion of a known node efficient.
- Allows insertion before or after a known node.

**Limitations**
- Requires additional memory for `prev`.
- Requires more reference updates during insertion and deletion.
- Searching still takes `O(n)` time.

### 2.7 Complexity Summary

| Operation | Time complexity |
|---|---|
| Access head or tail reference | `O(1)` |
| Forward traversal | `O(n)` |
| Backward traversal | `O(n)` |
| Search for a value | `O(n)` |
| Insert before or after a known node | `O(1)` |
| Delete a known node | `O(1)` |


## 3. Circular Linked List

### 3.1 What Is a Circular Linked List?

A **circular linked list** is a linked list in which the last node points back to an earlier node, typically the head.

Unlike an ordinary singly linked list, the final node does not point to `None`.

```text
       +----------------------+
       |                      |
       v                      |
head -> 10 -> 20 -> 30 -------+
```

Circularity is a property of the connections between nodes, not necessarily a separate node class.

### 3.2 Creating a Circular Singly Linked List

Use the singly linked `Node` class defined in Section 1.2.

```python
first = Node(10)
second = Node(20)
third = Node(30)

first.next = second
second.next = third
third.next = first
```

The final assignment creates the circular connection:

```python
third.next is first  # True
```

For an empty circular linked list, `head` is typically `None`.

### 3.3 Traversing a Circular Linked List

This loop is inappropriate for a non-empty circular list:

```python
current = first

while current is not None:
    print(current.data)
    current = current.next
```

Because the last node points back to the first, `current` never becomes `None`.

Instead, stop after returning to the starting node:

```python
current = first

if current is not None:
    while True:
        print(current.data)
        current = current.next

        if current is first:
            break
```

Output:

```text
10
20
30
```

The identity comparison `current is first` checks whether we have returned to the exact same node object.

**Complexity:** `O(n)` time and `O(1)` auxiliary space for a single traversal.

### 3.4 Advantages and Limitations

**Advantages**
- Supports repeated cyclic traversal.
- Useful for round-robin processing.
- With a maintained tail reference, certain end and beginning insertions can be performed in `O(1)` time.

**Limitations**
- Traversal needs an explicit stopping condition.
- Incorrect links can create unintended cycles or disconnect nodes.
- Searching still takes `O(n)` time in the worst case.

### 3.5 Complexity Summary

| Operation | Time complexity |
|---|---|
| Access the head reference | `O(1)` |
| Traverse all nodes once | `O(n)` |
| Search for a value | `O(n)` |
| Insert after a known node | `O(1)` |
| Insert at the end with a maintained tail | `O(1)` |
| Delete a node when its predecessor is known | `O(1)` |

Deleting a node in a singly linked list may require finding its predecessor, which can take `O(n)` time.

## 4. Pointer/Reference Manipulation

### 4.1 What Is Reference Manipulation?

Reference manipulation means changing the connections between nodes by assigning new references.

Python does not require raw pointer arithmetic for linked lists. Variables and node attributes refer to objects.

For example:

```python
first.next = second
```

connects the first node to the second node.

By contrast:

```python
current = second
```

only changes which node the local variable `current` references.

These operations have different effects.

### 4.2 Inserting a Node After a Known Node

Suppose we have:

```text
10 -> 30 -> None
```

We want to insert `20` after the node containing `10`.

Assume `current` refers to the node containing `10`.

```python
new_node = Node(20)

new_node.next = current.next
current.next = new_node
```

Result:

```text
10 -> 20 -> 30 -> None
```

The first assignment preserves the existing successor. The second connects the current node to the new node.

**Complexity:** `O(1)` time and `O(1)` auxiliary space, excluding the new node's storage.

### 4.3 Deleting a Node After a Known Predecessor

Given:

```text
10 -> 20 -> 30 -> None
```

If `current` references the node containing `10`, we can remove the following node:

```python
if current is not None and current.next is not None:
    current.next = current.next.next
```

Result:

```text
10 -> 30 -> None
```

**Complexity:** `O(1)` time and `O(1)` auxiliary space.

This assumes the predecessor is already known. Finding the predecessor may require `O(n)` traversal.

## 4.4 Avoiding Accidental Disconnection

Before overwriting a link, consider:

1. Which node does the current reference point to?
2. Which node does the existing link point to?
3. Will the original successor be needed later?
4. Have all necessary references been preserved?
5. Does the operation require updating `head` or `tail`?

For example:

```python
current.next = None
```

disconnects the current node from its successor. If no other reference provides access to the remaining nodes, those nodes may become unreachable.

### 4.5 Python Memory Management

Python automatically manages object memory.

```python
first = Node(10)
second = Node(20)

first.next = second
saved_reference = second

first.next = None
```

After the final assignment, `first` and `second` are no longer connected through `first.next`, but `saved_reference` still refers to the second node.

Removing a link does not automatically destroy the referenced object. An object can be reclaimed when it is no longer reachable through any live references, subject to Python's memory-management mechanisms.

### 4.6 Reference Manipulation Complexity

| Operation | Time complexity |
|---|---|
| Read a node's data | `O(1)` |
| Follow one reference | `O(1)` |
| Assign a reference | `O(1)` |
| Insert after a known node | `O(1)` |
| Remove the node after a known predecessor | `O(1)` |
| Find a node through traversal | `O(n)` |

The key distinction is between **finding a node** and **changing a reference**. Finding a node may require traversal; changing a known reference is typically constant-time.
