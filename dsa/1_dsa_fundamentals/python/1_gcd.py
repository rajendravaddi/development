import math

# 1. GCD using Python's math.gcd()
def gcd_math(a, b):
    """
    Use Python's built-in math.gcd() function to calculate the greatest common divisor of two integers.

    Complexity:
        - Time: O(log(min(a, b))) approximately
          The underlying implementation uses an efficient GCD algorithm based on the Euclidean algorithm.
        - Space: O(1) auxiliary space for the two-argument call.
    """
    return math.gcd(a, b)


# 2. GCD using Python's math.gcd() with arbitrary arguments
def gcd_math_args(*args):
    """
    Use math.gcd() with *args so that any number of integers can be passed to the function.

    The mathematical idea is:

        gcd(a, b, c) = gcd(gcd(a, b), c)

    Therefore, the GCD can be reduced one argument at a time.

    Complexity:
        - Time: O(n * log(M))
          where n is the number of arguments and M is the approximate size of the largest number.
        - Space: O(n) for the *args tuple created by Python.

    """
    return math.gcd(*args)


# 3. GCD using a loop
def gcd_loop(a, b):
    """
    Check every possible divisor from 1 up to the smaller of the two numbers.

    If a number divides both a and b exactly, it is a common divisor. Since we continue increasing the divisor, the last common divisor found is the GCD.

    Example:

        gcd_loop(48, 18)

        Common divisors:
            1, 2, 3, 6

        Therefore:
            GCD = 6

    Complexity:
        - Time: O(min(a, b))
          In the worst case, the loop checks every number from 1 through min(a, b).
        - Space: O(1)

    """

    result = 1

    for i in range(1, min(a, b) + 1):
        if a % i == 0 and b % i == 0:
            result = i

    return result



"""
Euclidean Algorithm
-------------------

The Euclidean algorithm is based on the mathematical property:

    gcd(a, b) = gcd(b, a % b)

Why does this work?

Suppose:

    a = b * q + r

where r is the remainder of a divided by b.

Any number that divides both a and b must also divide r.

Therefore, the common divisors of (a, b) are the same as the common divisors of (b, r).

So:

    gcd(a, b) = gcd(b, a % b)

We repeatedly replace:

    (a, b)

with:

    (b, a % b)

until b becomes 0.

At that point, a contains the GCD.

Example:

    gcd(48, 18)

    48 % 18 = 12
    18 % 12 = 6
    12 % 6  = 0

    Therefore:
        GCD = 6

This is much more efficient than checking every possible divisor.
"""


# 4. GCD using the Euclidean algorithm
def gcd_euclidean(a, b):
    """
    Approach:
        Use the Euclidean algorithm iteratively.

        While b is not zero:
            1. Replace a with b.
            2. Replace b with a % b.

        When b becomes zero, a is the GCD.

    Example:

        gcd_euclidean(48, 18)

            a = 48, b = 18
            a = 18, b = 12
            a = 12, b = 6
            a = 6,  b = 0

        Return 6.

    The statement:

        a, b = b, a % b

    performs both updates simultaneously.

    Complexity:
        - Time: O(log(min(a, b)))
          The Euclidean algorithm reduces the problem very
          quickly at every iteration.

        - Space: O(1)
          Only a constant number of variables are used.

    Note:
        This is the preferred manual implementation when
        implementing GCD yourself.
    """

    while b:
        a, b = b, a % b

    return a


# 5. GCD using the Euclidean algorithm with recursion
def gcd_euclidean_recusrion(a, b):
    """
    Approach:
        Use the recursive form of the Euclidean algorithm.

        The recursive relationship is:

            gcd(a, b) = gcd(b, a % b)

        Base case:
            If b == 0, return a.

        Recursive case:
            Calculate gcd(b, a % b).

    Example:

        gcd_euclidean_recusrion(48, 18)

            gcd(48, 18)
                ↓
            gcd(18, 12)
                ↓
            gcd(12, 6)
                ↓
            gcd(6, 0)
                ↓
                6

    Complexity:
        - Time: O(log(min(a, b)))
          The number of recursive calls follows the same logarithmic behavior as the iterative algorithm.

        - Space: O(log(min(a, b)))
          Each recursive call remains on the call stack until the base case is reached.

    Important:
        Unlike the iterative version, recursion requires
        additional call-stack memory.
    """

    if b == 0:
        return a

    return gcd_euclidean_recusrion(b, a % b)



"""
GCD for Arbitrary Arguments
---------------------------

The two-number GCD can be extended to any number of arguments.

The key mathematical property is:

    gcd(a, b, c) = gcd(gcd(a, b), c)

For example:

    gcd(48, 18, 30)

First:

    gcd(48, 18) = 6

Then:

    gcd(6, 30) = 6

Therefore:

    gcd(48, 18, 30) = 6

The same idea can be continued for any number of arguments:

    gcd(a, b, c, d)
        = gcd(gcd(gcd(a, b), c), d)

The implementation below repeatedly applies the
two-number Euclidean algorithm.
"""


# 6. GCD using the Euclidean algorithm with arbitrary arguments
def gcd_euclidean_args(*args):

    result = 0

    for i in args:
        result = gcd_euclidean(result, i)

    return result

print(gcd_euclidean(12, 18))
print(gcd_euclidean_args(12,18,24))