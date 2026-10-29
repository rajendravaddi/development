# Hashing Fundamentals

## 1. Core Concepts

### 1.1 Hashing

#### What is hashing?

**Hashing** is a technique for storing and retrieving data efficiently by converting a **key** into a numeric value using a **hash function**.

The basic flow is:

```text
Key → Hash Function → Hash Value → Bucket/Index → Data
```

For example:

```text
"Raj" → hash("Raj") → 58321 → index 1
```

The purpose of hashing is to make operations such as searching, inserting, and deleting very fast on average.

Hashing is commonly used to implement:

- Hash tables
- Hash maps
- Hash sets

#### Why hashing is needed

Suppose we have an unsorted list:

```text
[15, 42, 7, 91, 23]
```

Searching for `91` may require checking elements one by one:

```text
15 → 42 → 7 → 91
```

This takes **O(n)** time.

With hashing, we calculate a location directly:

```text
91
 ↓
hash function
 ↓
index
```

This allows average-case lookup in:

```text
O(1)
```

Therefore, hashing is useful when we need frequent:

- Lookups
- Insertions
- Deletions
- Membership checks
- Frequency counting
- Duplicate detection

### 1.2 Hash function

A **hash function** converts a key into a hash value.

Formally:

```text
h(key) → hash value
```

For a simple integer hash function:

```text
h(k) = k % 10
```

we get:

```text
h(23) = 3
h(47) = 7
h(82) = 2
```

A hash function should consistently map the same key to the same hash result.

Different keys, however, can produce the same result. This is called a **collision**.


#### Key → hash value → bucket/index

A hash table usually has a fixed number of buckets or slots.

The complete process is:

```text
Key
 ↓
Hash Function
 ↓
Hash Value
 ↓
Bucket/Index
```

For example, if the table has `10` buckets:

```text
index = hash(key) % 10
```

For:

```text
key = 123
```

we get:

```text
123 % 10 = 3
```

Therefore:

```text
123 → index 3
```

The **hash value** and **index** are conceptually different:

- **Hash value**: output produced by the hash function.
- **Index**: position used in the hash table.

For example:

```text
hash("Raj") = 9283749283
index = 9283749283 % 10
      = 3
```

### 1.3 Hash table

A **hash table** is a data structure that stores elements using a hash function to determine their location.

Conceptually, it can be represented as an array:

```text
Index    Data
  0       -
  1       -
  2       -
  3       123
  4       -
  5       -
```

The hash function determines where a key should be placed.

Hash tables are designed to provide approximately:

```text
Search   → O(1) average
Insert   → O(1) average
Delete   → O(1) average
```

Collisions can cause these operations to become slower.


### 1.4 Hash map

A **hash map** stores data as **key-value pairs**.

```text
Key → Value
```

Example:

```text
101 → "Alice"
102 → "Bob"
103 → "Charlie"
```

The key is hashed to determine where the key-value pair is stored.

A hash map is useful when we need to retrieve a value using a key.

For example:

```text
student[102]
```

returns:

```text
"Bob"
```

Keys are normally unique. Assigning another value to an existing key updates that key's value.


### 1.5 Hash set

A **hash set** stores a collection of **unique elements**.

Example:

```text
{10, 20, 30, 40}
```

Adding `20` again does not create another copy.

Hash sets are particularly useful for fast membership testing:

```text
20 in numbers
```

which is typically:

```text
O(1) average
```

A hash map stores:

```text
Key → Value
```

while a hash set stores:

```text
Element
```


## 2. Hash Function Construction Methods

### 2.1 Division / Modulo method

The **division method** uses the remainder after dividing the key by the table size.

Formula:

```text
h(k) = k % m
```

where:

- `k` = key
- `m` = table size

Example:

```text
k = 123
m = 10

h(123) = 123 % 10
       = 3
```

So the key maps to index `3`.

This method is simple and fast.


### 2.2 Multiplication method

The **multiplication method** multiplies the key by a constant and uses the fractional part of the result to determine the index.

A common formula is:

```text
h(k) = floor(m × fractional_part(kA))
```

where:

- `k` = key
- `m` = table size
- `A` = constant between `0` and `1`

Example:

```text
k = 123
m = 10
A = 0.618
```

Then:

```text
123 × 0.618 = 76.014
```

Fractional part:

```text
0.014
```

Therefore:

```text
floor(10 × 0.014) = 0
```

So the resulting index is `0`.


### 2.3 Mid-Square method

The **mid-square method**:

1. Squares the key.
2. Extracts the middle digits.
3. Uses those digits as the hash value/index.

Example:

```text
key = 123

123² = 15129
```

Middle digits can be selected from `15129`, depending on the required table size.

If we extract `51`:

```text
51 % 10 = 1
```

the resulting table index is `1`.

The method attempts to mix information from different parts of the key through squaring.


### 2.4 Folding method

The **folding method** divides a key into smaller groups and combines those groups.

Example:

```text
Key = 123456789

123 | 456 | 789
```

Add the groups:

```text
123 + 456 + 789 = 1368
```

For a table of size `10`:

```text
1368 % 10 = 8
```

Therefore:

```text
123456789 → index 8
```

A variation called **folding with reversal** reverses alternate groups before combining them.


### 2.5 Digit extraction / digit analysis

**Digit extraction** selects particular digits from a key to construct a hash value.

This is useful when some digits vary more than others.

For example:

```text
20240101
20240115
20240203
20240321
```

The first four digits are always:

```text
2024
```

so they provide little useful variation.

Digits from the later part of the keys may provide better distribution.

**Digit analysis** involves examining the keys and choosing digit positions that are likely to distribute the keys well.

### 2.6 Basic properties of a good hash function

A good hash function should be:

#### 2.6.1 Deterministic

The same key should produce the same hash value under the same conditions.

```text
h("Raj") = X

h("Raj") = X
```

It should not randomly produce a different result each time.


#### 2.6.2 Fast to compute

The hash function should be inexpensive to calculate.

Hashing may occur during:

```text
Insertion
Lookup
Deletion
```

so an expensive hash function can reduce overall performance.


#### 2.6.3 Good distribution

A good hash function should distribute keys reasonably evenly across the available buckets.

Good distribution:

```text
Bucket 0 → ● ●
Bucket 1 → ● ● ●
Bucket 2 → ● ●
Bucket 3 → ● ● ●
```

Poor distribution:

```text
Bucket 0 → ● ● ● ● ● ● ● ●
Bucket 1 →
Bucket 2 → ●
Bucket 3 →
```

Poor distribution creates more collisions and reduces performance.

#### 2.6.4 Minimize collisions

A good hash function should minimize unnecessary collisions.

However, it **cannot eliminate collisions completely** because there are usually more possible keys than available buckets.

Its goal is therefore:

```text
Good distribution → Fewer collisions → Better performance
```

## 3. Collision

### 3.1 What is a collision?

A **collision** occurs when two or more different keys map to the same bucket/index.

For example:

```text
h(k) = k % 10

21 % 10 = 1
31 % 10 = 1
```

Therefore:

```text
21 → index 1
31 → index 1
```

This is a collision.


### 3.2 Why collisions are unavoidable

There are usually many more possible keys than available buckets.

For example:

```text
Possible keys = 1,000
Buckets       = 10
```

It is impossible to assign every key a unique bucket.

This is a consequence of the **pigeonhole principle**.

Therefore:

> Collisions are a normal part of hash-table design, not necessarily a sign of a bad hash function.

A good hash function attempts to distribute collisions evenly rather than eliminate them completely.


### 3.3 Effect of collisions on performance

Collisions require additional work to determine which key is actually being searched for.

With few collisions:

```text
Key → Bucket → Small amount of work
```

With many collisions:

```text
Key → Bucket → Many candidate entries → More comparisons
```

As collisions increase, operations become slower.

In the extreme worst case, all keys could map to the same location, resulting in:

```text
O(n)
```

instead of the expected:

```text
O(1)
```


## 4. Collision Resolution

Collision resolution defines how a hash table handles multiple keys that map to the same location.

The two major approaches are:

```text
1. Chaining
2. Open Addressing
```

### 4.1 Chaining

**Chaining** allows multiple elements to be associated with the same bucket.

A common implementation uses a linked list or another collection.

Example:

```text
Bucket 0 → 10 → 15 → 20
Bucket 1
Bucket 2 → 12
Bucket 3
```

If:

```text
h(10) = 0
h(15) = 0
h(20) = 0
```

all three elements are stored in the chain belonging to bucket `0`.

#### Lookup

To find `20`:

```text
20
 ↓
hash(20)
 ↓
bucket 0
 ↓
search chain
```

#### Advantages

- Simple collision handling
- Multiple elements can occupy one bucket
- Deletion is relatively straightforward
- Load factor can exceed `1`

#### Disadvantage

Long chains increase lookup time.

Typical performance:

```text
Average → O(1)
Worst   → O(n)
```

### 4.2  Open addressing

**Open addressing** stores all elements directly inside the hash-table array.

When the calculated position is occupied, another position is searched using a **probing strategy**.

The main probing techniques are:

```text
Linear probing
Quadratic probing
Double hashing
```

Example:

```text
Initial index = 3

Index 3 → occupied
Index 4 → occupied
Index 5 → empty

Store element at index 5
```

Unlike chaining, open addressing does not maintain a separate chain outside the table.

#### 4.2.1 Linear probing

**Linear probing** checks consecutive positions after a collision.

Formula:

```text
index = (h(k) + i) % m
```

where:

```text
i = 0, 1, 2, ...
```

Example:

```text
Initial index = 3

3 → occupied
4 → occupied
5 → empty
```

The element is placed at index `5`.

##### Main issue: primary clustering

Linear probing can create groups of consecutive occupied positions:

```text
[10][17][24][31][ ][ ]
```

New collisions may join the same group, increasing search time.

#### 4.2.3 Quadratic probing

**Quadratic probing** increases the probe distance quadratically.

A common formula is:

```text
index = (h(k) + i²) % m
```

The offsets are:

```text
i = 0 → 0
i = 1 → 1
i = 2 → 4
i = 3 → 9
```

For an initial index of `3` and table size `10`, the sequence could be:

```text
3 → 4 → 7 → 2 → ...
```

Quadratic probing reduces the primary clustering associated with linear probing.

However, the table size and probing parameters affect whether all positions can be reached.


#### 4.2.4 Double hashing

**Double hashing** uses two hash functions.

Formula:

```text
index = (h₁(k) + i × h₂(k)) % m
```

where:

- `h₁(k)` determines the initial position.
- `h₂(k)` determines the step size.
- `i` is the probe number.

Example:

```text
h₁(k) = k % 7
h₂(k) = 1 + (k % 5)
```

If:

```text
h₁(k) = 3
h₂(k) = 2
```

the probing sequence becomes:

```text
3
5
0
2
4
...
```

The key advantage is that different keys can have different probe sequences, which can reduce clustering.

## 5. Hash Table Performance

### 5.1 Load factor

The **load factor** measures how full a hash table is.

Formula:

```text
α = n / m
```

where:

- `n` = number of stored elements
- `m` = number of buckets/slots

Example:

```text
n = 7
m = 10

α = 7 / 10
  = 0.7
```

A higher load factor generally means:

```text
More occupied space
      ↓
More collisions
      ↓
More work per operation
```

For **chaining**, the load factor can exceed `1`.

For **open addressing**, the load factor must remain below `1` because every element needs an actual table slot.


### 5.2 Resizing

**Resizing** means increasing or decreasing the capacity of a hash table.

When a table becomes too full, it may grow:

```text
8 → 16
16 → 32
32 → 64
```

Increasing the number of buckets reduces the load factor.

For example:

```text
Before:
n = 6
m = 8

α = 0.75
```

After growing to `16` buckets:

```text
n = 6
m = 16

α = 0.375
```

Resizing is expensive because existing elements generally need to be moved.


### 5.3 Rehashing

**Rehashing** means recalculating the positions of existing elements after the table configuration changes.

Suppose:

```text
index = key % m
```

For key `7`:

```text
m = 5:
7 % 5 = 2
```

After resizing:

```text
m = 10:
7 % 10 = 7
```

The key therefore needs to move from index `2` to index `7`.

Conceptually:

```text
Old table
    ↓
Resize
    ↓
Recalculate hash/index
    ↓
Move existing elements
    ↓
New table
```

**Resizing** changes the table capacity.

**Rehashing** recalculates and redistributes existing entries.

They commonly occur together.


### 5.4 Average-case complexity

With a good hash function and a controlled load factor:

| Operation | Average-case |
|---|---:|
| Search | O(1) |
| Insert | O(1) |
| Delete | O(1) |

This is the primary performance advantage of hash tables.

The `O(1)` represents **average/expected behavior**, not a guarantee for every individual operation.


### 5.5 Worst-case complexity

The worst case occurs when many or all keys map to the same location.

For example:

```text
Bucket 0 → A → B → C → D → E → F
```

Searching may require examining all `n` elements.

Therefore:

| Operation | Worst-case |
|---|---:|
| Search | O(n) |
| Insert | O(n)* |
| Delete | O(n)* |

\*Exact behavior depends on the collision-resolution strategy and implementation.

The important distinction is:

```text
Average case → O(1)
Worst case   → O(n)
```

### 5.6 Amortized cost of resizing

A resize can take **O(n)** because many existing elements must be rehashed and moved.

However, resizing normally happens only occasionally.

For example:

```text
8 → 16 → 32 → 64 → 128
```

The table grows substantially between expensive resize operations.

Across a long sequence of insertions, the total resizing work is distributed over many operations.

Therefore:

```text
Normal insertion       → O(1)
Individual resize      → O(n)
Amortized insertion    → O(1)
```

**Amortized O(1)** means that although some individual insertions are expensive because they trigger resizing, the average cost over a large sequence of insertions remains constant.
