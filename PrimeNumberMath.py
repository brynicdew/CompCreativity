import math

def is_prime(n: int) -> bool:
    """Return True if n is prime, else False."""
    if n <= 1:
        return False
    for i in range(2, int(math.sqrt(n))+1):
        if n % i == 0:
            return False
    return True

def nth_prime(n: int) -> int:
    """Return the nth prime number."""
    count = 0
    i = 2
    while True:
        if is_prime(i):
            count += 1
            if count == n:
                return i
        i += 1

def add_primes(a: int, b: int) -> int:
    """Return the sum of the a-th and b-th prime numbers."""
    return nth_prime(a) + nth_prime(b)

def subtract_primes(a: int, b: int) -> int:
    """Return the difference of the a-th and b-th prime numbers."""
    return nth_prime(a) - nth_prime(b)

def multiply_primes(a: int, b: int) -> int:
    """Return the product of the a-th and b-th prime numbers."""
    return nth_prime(a) * nth_prime(b)

def divide_primes(a: int, b: int) -> float:
    """Return the quotient of the a-th and b-th prime numbers."""
    return nth_prime(a) / nth_prime(b)

if __name__ == '__main__':
    a = int(input("Enter the position of the first prime number: "))
    b = int(input("Enter the position of the second prime number: "))
    
    print(f"{a}-th prime number: {nth_prime(a)}")
    print(f"{b}-th prime number: {nth_prime(b)}")
    print(f"Sum: {add_primes(a, b)}")
    print(f"Difference: {subtract_primes(a, b)}")
    print(f"Product: {multiply_primes(a, b)}")
    print(f"Quotient: {divide_primes(a, b)}")
