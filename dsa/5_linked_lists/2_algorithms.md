# Algorithms

## Common Setup: Node and Helper Functions

We will reuse the same `Node` class throughout this section instead of redefining it for every algorithm.

```python
class Node:
    def __init__(self, data):
        self.data = data
        self.next = None
```

A node stores a value in `data` and a reference to the next node in `next`.

The following helper functions make it easier to create test lists and display results.

### Create a linked list from Python values

```python
def create_linked_list(values):
    head = None
    tail = None

    for value in values:
        new_node = Node(value)

        if head is None:
            head = new_node
            tail = new_node
        else:
            tail.next = new_node
            tail = new_node

    return head
```

Example:

```python
head = create_linked_list([10, 20, 30, 40])
```

The result is:

```text
head -> 10 -> 20 -> 30 -> 40 -> None
```

The helper maintains both `head` and `tail`, so constructing a list of `n` values takes `O(n)` time.

### Display a linked list

```python
def display(head):
    current = head

    while current is not None:
        print(current.data, end=" -> ")
        current = current.next

    print("None")
```

Example:

```python
head = create_linked_list([10, 20, 30])
display(head)
```

Output:

```text
10 -> 20 -> 30 -> None
```

This display helper assumes the list is acyclic. Do not use it on a circular list without adding a stopping condition.


## 1. Reversal

### 1.1 What is Linked-List Reversal?

**Reversal** changes the direction of the links so that the last node becomes the first node and the first node becomes the last.

Before:

```text
head
 |
 v
10 -> 20 -> 30 -> 40 -> None
```

After:

```text
head
 |
 v
40 -> 30 -> 20 -> 10 -> None
```

The nodes are not copied. Their `next` references are changed.

### 1.2 Iterative Reversal

The standard iterative approach uses three references:

- `previous`: The previously processed node.
- `current`: The node currently being processed.
- `next_node`: The next node, saved before reversing the current link.

#### Implementation

```python
def reverse_linked_list(head):
    previous = None
    current = head

    while current is not None:
        next_node = current.next

        current.next = previous

        previous = current
        current = next_node

    return previous
```

Usage:

```python
head = create_linked_list([10, 20, 30, 40])

head = reverse_linked_list(head)

display(head)
```

Output:

```text
40 -> 30 -> 20 -> 10 -> None
```

### 1.3 Step-by-Step Explanation

Initially:

```text
previous = None
current  = 10
```

The list is:

```text
10 -> 20 -> 30 -> 40 -> None
```

#### Step 1: Save the next node

```python
next_node = current.next
```

`next_node` refers to `20`.

This is important because the next assignment will overwrite the current link.

#### Step 2: Reverse the current link

```python
current.next = previous
```

Node `10` now points to `None`.

#### Step 3: Advance both references

```python
previous = current
current = next_node
```

Now `previous` points to `10`, and `current` points to `20`.

The same process repeats for every node.

#### Dry run

| Iteration | Current node | Previous after reversal |
|---|---:|---|
| 1 | 10 | 10 |
| 2 | 20 | 20 → 10 |
| 3 | 30 | 30 → 20 → 10 |
| 4 | 40 | 40 → 30 → 20 → 10 |

At the end, `current` becomes `None`, and `previous` references the new head.

### 1.4 Why Is `next_node` Necessary?

Suppose we wrote:

```python
current.next = previous
current = current.next
```

After the first assignment, `current.next` refers to `previous`, not the original successor.

The original link to the remaining list would be lost from `current`.

Saving `next_node` before changing the link prevents this problem.

### 1.5 Complexity

- Time: `O(n)`.
- Auxiliary space: `O(1)`.

The algorithm modifies existing links without allocating additional nodes.


## 2. Merge Two Lists

### 2.1 What Does Merging Mean?

Merging two sorted linked lists produces a single sorted linked list containing all the elements from both input lists.

For example:

```text
List A: 10 -> 30 -> 50 -> None

List B: 20 -> 40 -> 60 -> None
```

Merged result:

```text
10 -> 20 -> 30 -> 40 -> 50 -> 60 -> None
```

The standard two-pointer merge assumes both input lists are already sorted in non-decreasing order.

### 2.2 Iterative Merge Using a Dummy Node

A **dummy node** is a temporary starting node that simplifies list construction. It is not part of the final result.

#### Implementation

```python
def merge_sorted_lists(list1, list2):
    dummy = Node(0)
    tail = dummy

    while list1 is not None and list2 is not None:
        if list1.data <= list2.data:
            tail.next = list1
            list1 = list1.next
        else:
            tail.next = list2
            list2 = list2.next

        tail = tail.next

    if list1 is not None:
        tail.next = list1
    else:
        tail.next = list2

    return dummy.next
```

Usage:

```python
list1 = create_linked_list([10, 30, 50])
list2 = create_linked_list([20, 40, 60])

merged = merge_sorted_lists(list1, list2)

display(merged)
```

Output:

```text
10 -> 20 -> 30 -> 40 -> 50 -> 60 -> None
```

This implementation reuses the existing nodes rather than creating copies of them.

**Important:** Because the nodes are reused, the original lists' links are modified. If you need to preserve both input lists, use a copying implementation instead.

### 2.3 How the Algorithm Works

The algorithm maintains three important references:

- `list1`: The next unprocessed node in the first list.
- `list2`: The next unprocessed node in the second list.
- `tail`: The final node in the merged portion.

At each step, compare the first unprocessed values.

If `list1.data <= list2.data`, attach the node from `list1`. Otherwise, attach the node from `list2`.

Then advance the selected input reference and move the merged tail forward.

#### Example

Initially:

```text
List A: 10 -> 30 -> 50
List B: 20 -> 40 -> 60

Merged: dummy
```

Choose `10`:

```text
Merged: 10
```

Choose `20`:

```text
Merged: 10 -> 20
```

Choose `30`:

```text
Merged: 10 -> 20 -> 30
```

Continue until one input list is exhausted.

Finally, attach the remaining nodes from the other list.

### 2.4 Why Use a Dummy Node?

Without a dummy node, the first insertion needs special handling because the merged list does not yet have a head.

With a dummy node, every insertion follows the same basic pattern:

```python
tail.next = selected_node
tail = tail.next
```

The real head of the merged list is:

```python
dummy.next
```

### 2.5 Complexity

Let `n` and `m` be the lengths of the two input lists.

- Time: `O(n + m)`.
- Auxiliary space: `O(1)`, excluding the dummy node.

Each input node is processed at most once.


## 3. Find Middle

### 3.1 What Does Finding the Middle Mean?

Finding the middle means returning a reference to the central node of a linked list.

Consider:

```text
10 -> 20 -> 30 -> 40 -> 50 -> None
```

The middle node is `30`.

The challenge is that a singly linked list does not provide direct access to its length or its middle node.

Two common approaches are:

1. Count the nodes, then traverse to the middle.
2. Use slow and fast references.

We will use the second approach.

### 3.2 Slow and Fast Pointer Technique

The idea is simple:

- `slow` advances one node at a time.
- `fast` advances two nodes at a time.

When `fast` reaches the end, `slow` is at the middle.

#### Implementation

```python
def find_middle(head):
    slow = head
    fast = head

    while fast is not None and fast.next is not None:
        slow = slow.next
        fast = fast.next.next

    return slow
```

Usage:

```python
head = create_linked_list([10, 20, 30, 40, 50])

middle = find_middle(head)

if middle is not None:
    print(middle.data)
```

Output:

```text
30
```

### 3.3 Dry Run

For:

```text
10 -> 20 -> 30 -> 40 -> 50 -> None
```

Initially, both references point to `10`.

| Iteration | Slow | Fast |
|---|---:|---:|
| Start | 10 | 10 |
| 1 | 20 | 30 |
| 2 | 30 | 50 |
| Stop | 30 | Cannot advance two nodes |

The algorithm returns the node containing `30`.

### 3.4 What Happens With an Even Number of Nodes?

Consider:

```text
10 -> 20 -> 30 -> 40 -> None
```

With this implementation, `slow` ends at `30`, the second of the two middle nodes.

For an even-length list, there are two central nodes. This algorithm deliberately returns the second one.

If you need the first middle node instead, the loop condition must be adjusted.

### 3.5 Edge Cases

- Empty list: Returns `None`.
- One node: Returns that node.
- Odd number of nodes: Returns the unique middle node.
- Even number of nodes: Returns the second middle node.

### 3.6 Complexity

- Time: `O(n)`.
- Auxiliary space: `O(1)`.

The slow and fast references allow us to find the middle in a single traversal without first counting the nodes.


## 4. Detect Cycle

### 4.1 What Is a Cycle?

A linked list contains a **cycle** when following `next` references eventually returns to a node already visited.

A normal list:

```text
10 -> 20 -> 30 -> 40 -> None
```

A list with a cycle:

```text
10 -> 20 -> 30 -> 40
          ^         |
          |_________|
```

In the second example, the node containing `40` points back to the node containing `20`.

A traversal that only checks whether `current` is `None` will never terminate.

### 4.2 Floyd's Cycle Detection Algorithm

Floyd's cycle detection algorithm is also called the **tortoise and hare algorithm**.

It uses two references:

- `slow` advances one node at a time.
- `fast` advances two nodes at a time.

If a cycle exists, the two references eventually meet inside the cycle.

If there is no cycle, `fast` eventually reaches `None`.

#### Implementation

```python
def has_cycle(head):
    slow = head
    fast = head

    while fast is not None and fast.next is not None:
        slow = slow.next
        fast = fast.next.next

        if slow is fast:
            return True

    return False
```

### 4.3 Testing a List Without a Cycle

```python
head = create_linked_list([10, 20, 30, 40])

print(has_cycle(head))
```

Output:

```text
False
```

The fast reference eventually reaches `None`, so the function returns `False`.

### 4.4 Testing a List With a Cycle

```python
head = create_linked_list([10, 20, 30, 40])

node20 = head.next
tail = head

while tail.next is not None:
    tail = tail.next

tail.next = node20

print(has_cycle(head))
```

Output:

```text
True
```

The final assignment creates a cycle by connecting the tail back to the node containing `20`.

### 4.5 Why Do the References Meet?

Once both references are inside a cycle, the fast reference moves two positions for every one position moved by the slow reference.

Therefore, the fast reference gains one node per iteration relative to the slow reference.

Because the cycle contains a finite number of nodes, the relative distance eventually wraps around and becomes zero. At that point, both references point to the same node.

The identity comparison:

```python
slow is fast
```

checks whether both references point to the same node object.

### 4.6 Complexity

- Time: `O(n)`.
- Auxiliary space: `O(1)`.

The algorithm does not need a set of previously visited nodes.

A set-based approach can also detect cycles, but it generally requires `O(n)` additional space.


## 5. Remove Cycle

### 5.1 What Does Removing a Cycle Mean?

Removing a cycle means changing one reference so that the linked list becomes a normal, terminating singly linked list.

Before:

```text
10 -> 20 -> 30 -> 40
      ^         |
      |_________|
```

After:

```text
10 -> 20 -> 30 -> 40 -> None
```

The goal is to find the reference that creates the cycle and change it to `None`.

### 5.2 Approach

We can use Floyd's cycle detection algorithm in three stages:

1. Detect whether a cycle exists.
2. Find the node where the cycle begins.
3. Find the last node in the cycle and set its `next` to `None`.

#### Implementation

```python
def remove_cycle(head):
    slow = head
    fast = head

    # Stage 1: Detect a cycle
    while fast is not None and fast.next is not None:
        slow = slow.next
        fast = fast.next.next

        if slow is fast:
            break
    else:
        # No cycle exists
        return head

    # Stage 2: Find the cycle's starting node
    slow = head

    while slow is not fast:
        slow = slow.next
        fast = fast.next

    cycle_start = slow

    # Stage 3: Find the last node in the cycle
    current = cycle_start

    while current.next is not cycle_start:
        current = current.next

    current.next = None

    return head
```

### 5.3 Example

```python
head = create_linked_list([10, 20, 30, 40, 50])

node20 = head.next
tail = head

while tail.next is not None:
    tail = tail.next

tail.next = node20

print(has_cycle(head))  # True

remove_cycle(head)

print(has_cycle(head))  # False
display(head)
```

Output:

```text
True
False
10 -> 20 -> 30 -> 40 -> 50 -> None
```

### 5.4 Explanation

#### Stage 1: Detect the cycle

Use the slow and fast references as in the previous algorithm.

If the references never meet, the list has no cycle, and no changes are required.

#### Stage 2: Find the cycle start

After the references meet, reset `slow` to `head`, leaving `fast` at the meeting point.

Advance both one node at a time:

```python
while slow is not fast:
    slow = slow.next
    fast = fast.next
```

When they meet again, the node they reference is the cycle's starting node.

This works because the relative distances traveled by the two references align at the cycle entry after the reset.

#### Stage 3: Break the cycle

Starting at the cycle entry, walk around the cycle until finding the node whose `next` points back to the entry.

```python
while current.next is not cycle_start:
    current = current.next
```

Then remove the circular link:

```python
current.next = None
```

The list now terminates normally.

### 5.5 Complexity

- Time: `O(n)`.
- Auxiliary space: `O(1)`.

The algorithm modifies one link and does not allocate additional nodes.


## 6. Find Intersection

### 6.1 What Does Intersection Mean?

Two singly linked lists **intersect** when they share one or more actual node objects.

This is different from containing equal values.

Consider:

```text
List A: 10 -> 20 \
                 30 -> 40 -> None
List B: 15 ------/
```

Both lists share the same nodes containing `30` and `40`.

The intersection is the first shared node, containing `30`.

#### Important distinction

Two lists containing equal values do not necessarily intersect.

For example:

```text
List A: 10 -> 20 -> 30 -> None
List B: 10 -> 20 -> 30 -> None
```

If each list was independently constructed, they contain separate node objects and do not intersect.

Intersection is determined by **node identity**, not by comparing `data` values.

### 6.2 The Two-Pointer Approach

The most useful approach uses two references:

- `pointer_a` starts at the head of list A.
- `pointer_b` starts at the head of list B.

When a pointer reaches the end of its list, redirect it to the other list's head.

If the lists intersect, the pointers eventually meet at their first shared node.

If they do not intersect, both eventually become `None`.

#### Implementation

```python
def find_intersection(head_a, head_b):
    pointer_a = head_a
    pointer_b = head_b

    while pointer_a is not pointer_b:
        if pointer_a is None:
            pointer_a = head_b
        else:
            pointer_a = pointer_a.next

        if pointer_b is None:
            pointer_b = head_a
        else:
            pointer_b = pointer_b.next

    return pointer_a
```

The function returns the intersection node itself, or `None` if there is no intersection.

### 6.3 Creating Two Intersecting Lists

We must deliberately share node references to construct an intersection.

```python
shared = create_linked_list([30, 40, 50])

head_a = create_linked_list([10, 20])
head_b = create_linked_list([15, 25])

tail_a = head_a
while tail_a.next is not None:
    tail_a = tail_a.next

tail_b = head_b
while tail_b.next is not None:
    tail_b = tail_b.next

tail_a.next = shared
tail_b.next = shared
```

The resulting structure is:

```text
List A: 10 -> 20 --+
                   |
                   v
                  30 -> 40 -> 50 -> None
                   ^
                   |
List B: 15 -> 25 --+
```

Both lists now reference the same node containing `30`.

Test it:

```python
intersection = find_intersection(head_a, head_b)

if intersection is not None:
    print(intersection.data)
else:
    print("No intersection")
```

Output:

```text
30
```

### 6.4 Why Does the Two-Pointer Algorithm Work?

Suppose the lists have different lengths.

If pointer A traverses list A and then list B, and pointer B traverses list B and then list A, each pointer travels the same total distance.

For example:

```text
List A length: 5
List B length: 4
```

Each pointer traverses a combined distance of nine node positions if we count both lists' lengths.

If the lists share a tail, the pointers align at the first shared node after accounting for their different initial lengths.

If there is no intersection, each pointer eventually reaches `None` after traversing both lists and the pointers meet at `None`.

The comparison:

```python
pointer_a is pointer_b
```

is essential because intersection depends on object identity.

### 6.5 Complexity

Let the two list lengths be `n` and `m`.

- Time: `O(n + m)`.
- Auxiliary space: `O(1)`.

The algorithm uses only two pointer variables.

### 6.6 Edge Cases

- Both lists are empty: Returns `None`.
- One list is empty: Returns `None`.
- The lists intersect at their heads: Returns that shared head.
- The lists share only the final node: Returns the final node.
- The lists contain equal values but separate node objects: Returns `None`.

This implementation assumes the lists are acyclic.
