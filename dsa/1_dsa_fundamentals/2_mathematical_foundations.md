## Essential Mathematical Foundations

These mathematical concepts appear frequently in DSA, competitive programming, algorithms, cryptography, and problem solving.

### 1. Modular Arithmetic

Modular arithmetic is arithmetic based on the remainder after division by a number.

The `%` operator in programming gives us the remainder.

**For example:**
```
17 % 5 = 2
```
because:
```
17 = 5 × 3 + 2
```
So the remainder is `2`.

**Examples**
```
10 % 3 = 1
20 % 4 = 0
25 % 7 = 4
```

### 2. GCD — Greatest Common Divisor

GCD is the largest number that divides two or more numbers exactly.

**Example:**
```
12 → 1, 2, 3, 4, 6, 12
18 → 1, 2, 3, 6, 9, 18
```
Common factors:
```
1, 2, 3, 6
```
Largest one:
```
GCD(12, 18) = 6
```

**Python provides:**
```
import math

math.gcd(12, 18)
```
Result:
```
6
```

### 3. LCM — Least Common Multiple

LCM is the smallest positive number that is divisible by two or more numbers.

**For example:**

Multiples of 4: `4, 8, 12, 16, 20, 24, ...`

Multiples of 6: `6, 12, 18, 24, ...`

The first common multiple is: `12`

Therefore:
```
LCM(4, 6) = 12
```

**Important formula**

For two positive integers:
```
GCD(a, b) × LCM(a, b) = a × b
```
Therefore:
```
LCM(a, b) = (a × b) / GCD(a, b)
```

### 4. Prime Numbers

A prime number is a number greater than 1 that has exactly two factors: `1` and `itself`.

**Examples:**
```
2
3
5
7
11
13
17
19
```

### 5. Factors

A factor of a number is a number that divides it exactly, leaving remainder `0`.

**For example:**
```
12 = 1 × 12
12 = 2 × 6
12 = 3 × 4
```
Therefore the factors of 12 are: `1, 2, 3, 4, 6, 12`

**Important relationship**

Every number greater than 1 can be represented using prime factors.

For example:
```
60 = 2 × 2 × 3 × 5
```
This is called prime factorization.

### 6. Sieve of Eratosthenes

The Sieve of Eratosthenes is an algorithm used to find all prime numbers up to a given number n.

Instead of checking every number individually, we repeatedly mark the multiples of known prime numbers as non-prime.

Suppose we want primes up to `20`.

Start with:
```
2  3  4  5  6  7  8  9  10
11 12 13 14 15 16 17 18 19 20
```
Start with `2`.

Mark its multiples:
```
4, 6, 8, 10, 12, 14, 16, 18, 20
```
Next prime is `3`.

Mark:
```
6, 9, 12, 15, 18
```
Next prime is `5`.

Mark:
```
10, 15, 20
```
The numbers remaining unmarked are:
```
2, 3, 5, 7, 11, 13, 17, 19
```
These are the primes up to `20`.

**Complexity**

The Sieve of Eratosthenes runs in approximately:
```
O(n log log n)
```
This makes it very efficient for finding many primes.

### 7. Powers

A power represents repeated multiplication of the same number.

**For example:** 

`2³` means: `2 × 2 × 2`

Therefore:

`2³ = 8`

Here:
```
2 = base
3 = exponent
```

### 8. Logarithms

A logarithm tells us what exponent is needed to produce a particular number.

For example:
```
2³ = 8
```
Therefore:
```
log₂(8) = 3
```
Because `3` is the exponent required to turn `2` into `8`.

**Why logarithms matter in DSA**

Many algorithms repeatedly divide the problem.

For example:
```
n
n/2
n/4
n/8
...
```
This produces:
```
O(log n)
```
Binary search is the classic example.

### 9. Bit Representation

Bit representation is the representation of numbers using binary digits: `0` and `1`.

A bit can have only two values:
```
0
1
```
Computers internally represent data using bits.

**Example**

Decimal: `5`

Binary: `101`

Because:
```
101₂
= 1×2² + 0×2¹ + 1×2⁰
= 4 + 0 + 1
= 5
```

**Powers of 2**

Binary positions represent powers of 2:
```
128 64 32 16 8 4 2 1
```
For example:
```
13 = 8 + 4 + 1
```
Therefore:
```
13 = 1101₂
```

**Why it matters in DSA**

Bit representation is important for:

- Bit manipulation
- Bit masks
- Efficient storage
- Binary operations
- Powers of 2
- Some optimization techniques

Common operators include:
```
&   AND
|   OR
^   XOR
~   NOT
<<  Left shift
>>  Right shift
```

### 10. Basic Combinatorics

Combinatorics is the branch of mathematics concerned with counting and arranging objects.

It helps answer questions like:

"How many different ways can I do this?"

For example:

You have:
```
3 shirts
2 pants
```
For every shirt, you can choose either pair of pants.

Therefore:
```
3 × 2 = 6
```
possible outfits.

#### Common counting principles
**Addition Rule**

If you can do something in A ways or B ways:
```
Total = A + B
```
Example:
```
3 red choices
+
2 blue choices

= 5 choices
```

**Multiplication Rule**

If you perform two choices one after another:
```
Total = A × B
```
Example:
```
3 shirts × 2 pants = 6 outfits
```
These two principles are the foundation for many combinatorial problems.

### 11. Permutations

A permutation is an arrangement of objects where order matters.

If changing the order creates a different result, it is a permutation.

Suppose we have:
```
A, B, C
```
Possible arrangements include:
```
ABC
ACB
BAC
BCA
CAB
CBA
```
There are: `6` arrangements.

That is: `3! = 6`

**Formula**

The number of ways to arrange `r` objects from `n` objects is:
```
nPr = n! / (n-r)!
```

### 12. Combinations

A combination is a selection of objects where order does not matter.

If changing the order does not create a new result, it is a combination.

Suppose we choose 2 people from:
```
A, B, C
```
The possibilities are:
```
AB
AC
BC
```
We don't count:
```
BA
CA
CB
```
separately because the order doesn't matter.

Therefore there are: `3` combinations.

**Formula**

The number of ways to choose r objects from n objects is:
```
nCr = n! / (r!(n-r)!)
```

