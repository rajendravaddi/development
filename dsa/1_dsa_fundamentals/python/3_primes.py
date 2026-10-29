# Finding prime numbers up to a given number

def find_primes(n):
    """
    Approach:
        Check every number from 2 to n individually.

        For each number, check whether it is divisible by any number between 2 and number - 1.

        If a divisor is found, the number is composite.
        If no divisor is found, the number is prime.

        The `for ... else` construct is used here:
            - `break` means a divisor was found.
            - `else` executes only when the loop completes without encountering a break.

    Example:
        find_primes(10)

        Checks:
            2 -> prime
            3 -> prime
            4 -> divisible by 2 -> not prime
            5 -> prime
            ...
            10 -> divisible by 2 -> not prime

        Result:
            [2, 3, 5, 7]

    Time Complexity:
        O(n²) approximately.

        We check up to n numbers, and for each number we may check up to n possible divisors.

        Note:
        The actual number of iterations is smaller because the loop breaks as soon as a divisor is found.

    Space Complexity:
        O(n)

        The `primes` list stores all prime numbers found. In the worst case, it can contain O(n) elements.
    """

    primes = []

    for number in range(2, n + 1):
        for i in range(2, number):
            if number % i == 0:
                break
        else:
            primes.append(number)

    return len(primes), primes


# Finding prime numbers using the Sieve of Eratosthenes

def find_primes_sieve(n):
    """
    Approach:
        Instead of checking every number individually, use the Sieve of Eratosthenes.

        1. Initially assume every number is prime.
        2. Start with 2.
        3. If a number is still marked as prime, mark all of its multiples as non-prime.
        4. Continue until all composite numbers have been eliminated.
        5. The numbers still marked as prime are the result.

        `is_prime[i]` represents whether i is prime.

        For example:

            is_prime[7] == True
                -> 7 is considered prime

            is_prime[9] == False
                -> 9 was marked as a multiple of 3

        The marking starts from `i * i` because all smaller multiples of i have already been marked by smaller factors.

    Example:
        find_primes_sieve(10)

        Initially:
            2 3 4 5 6 7 8 9 10

        Process 2:
            Mark 4, 6, 8, 10 as non-prime.

        Process 3:
            Mark 9 as non-prime.

        Remaining primes:
            2, 3, 5, 7

    Time Complexity:
        O(n log log n)

        The sieve efficiently marks the multiples of each prime number instead of testing every number separately.

    Space Complexity:
        O(n)

        The `is_prime` list contains one boolean value for every number from 0 through n.
    """

    # n + 1 because index n must also be represented.
    is_prime = [True] * (n + 1)

    # 0 and 1 are not prime numbers.
    is_prime[0] = is_prime[1] = False

    # Check each potential prime.
    for i in range(2, n + 1):

        # If i is still marked as prime,
        # mark all of its multiples as non-prime.
        if is_prime[i]:

            # Start from i*i because smaller multiples of i
            # have already been marked by smaller prime factors.
            for multiple in range(i * i, n + 1, i):
                is_prime[multiple] = False

    # Collect all numbers that are still marked as prime.
    result = [i for i in range(n + 1) if is_prime[i]]

    return len(result), result


print(find_primes(100))
print(find_primes_sieve(100))