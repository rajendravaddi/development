import math
# 1. LCM using Python's math.lcm()
def lcm_math(a, b):
    """
    Use Python's built-in math.lcm() function to calculate the least common multiple of two integers.

    Example:
        lcm_math(12, 18) -> 36

    The LCM is the smallest positive integer that is divisible by both a and b.

    Complexity:
        - Time: approximately O(log(min(a, b)))
        The implementation internally uses an efficient GCD algorithm.
        - Space: O(1) auxiliary space.
    """
    return math.lcm(a, b)


# 2. LCM using Python's math.lcm() with arbitrary arguments

def lcm_math_args(*args):
    """
    Use math.lcm() with *args so that any number of integers can be passed to the function.

    Example:
        lcm_math_args(4, 6, 8) -> 24

    The mathematical relationship is:

        lcm(a, b, c) = lcm(lcm(a, b), c)

    Therefore, the LCM can be calculated by combining numbers two at a time.

    Complexity:
        Let:
            n = number of arguments
            M = magnitude of the largest intermediate value

        - Time: approximately O(n * log(M))
        - Space: O(n) for the *args tuple.

    """
    return math.lcm(*args)

# 3. LCM using GCD

"""
LCM using GCD
-------------

The most important relationship between GCD and LCM is:

    LCM(a, b) = |a * b| / GCD(a, b)

For integer arithmetic, we normally write:

    LCM(a, b) = abs(a * b) // GCD(a, b)

Example:

    a = 12
    b = 18

    GCD(12, 18) = 6

    LCM = (12 * 18) // 6
        = 216 // 6
        = 36

This approach is much more efficient than searching through possible multiples.

"""


def lcm(a, b):
    """
    Approach:
        Calculate the GCD using the Euclidean algorithm and then
        use the GCD-LCM relationship:

            LCM(a, b) = abs(a * b) // GCD(a, b)

    Example:

        lcm(12, 18)

        GCD(12, 18) = 6

        LCM = (12 * 18) // 6
            = 36

    Complexity:
        - Time: O(log(min(a, b)))
          The GCD is calculated using the Euclidean algorithm.

        - Space: O(1)
          Only a constant number of variables are required.

    Important:
        We use integer division (//) because the LCM of two integers is always an integer.
    """

    gcd = math.gcd(a, b)

    return abs(a * b) // gcd

print(lcm(12, 18))
print(lcm_math_args(14, 2, 7))