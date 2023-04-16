import math

class PrimeNumberMath:
    @staticmethod
    def is_prime(n: int) -> bool:
        if n <= 1:
            return False

        for i in range(2, int(math.sqrt(n)) + 1):
            if n % i == 0:
                return False

        return True

    @staticmethod
    def nth_prime(n: int) -> int:
        primes = []

        current = 2
        while len(primes) < n:
            if PrimeNumberMath.is_prime(current):
                primes.append(current)
            current += 1

        return primes[-1]

    @staticmethod
    def add(n1: int, n2: int) -> int:
        prime1 = PrimeNumberMath.nth_prime(n1)
        prime2 = PrimeNumberMath.nth_prime(n2)

        return prime1 + prime2

    @staticmethod
    def subtract(n1: int, n2: int) -> int:
        prime1 = PrimeNumberMath.nth_prime(n1)
        prime2 = PrimeNumberMath.nth_prime(n2)

        return prime1 - prime2

    @staticmethod
    def multiply(n1: int, n2: int) -> int:
        prime1 = PrimeNumberMath.nth_prime(n1)
        prime2 = PrimeNumberMath.nth_prime(n2)

        return prime1 * prime2

    @staticmethod
    def divide(n1: int, n2: int) -> int:
        prime1 = PrimeNumberMath.nth_prime(n1)
        prime2 = PrimeNumberMath.nth_prime(n2)

        if prime2 == 0:
            raise ValueError("Cannot divide by zero")

        return prime1 // prime2
