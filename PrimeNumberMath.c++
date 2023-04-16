#include <iostream>
#include <vector>
#include <cmath>

class PrimeNumberMath {
public:
    static bool isPrime(int n) {
        if (n <= 1) {
            return false;
        }

        for (int i = 2; i <= std::sqrt(n); i++) {
            if (n % i == 0) {
                return false;
            }
        }

        return true;
    }

    static int nthPrime(int n) {
        std::vector<int> primes;

        int current = 2;
        while (primes.size() < n) {
            if (isPrime(current)) {
                primes.push_back(current);
            }
            current++;
        }

        return primes.back();
    }

    static int add(int n1, int n2) {
        int prime1 = nthPrime(n1);
        int prime2 = nthPrime(n2);

        return prime1 + prime2;
    }

    static int subtract(int n1, int n2) {
        int prime1 = nthPrime(n1);
        int prime2 = nthPrime(n2);

        return prime1 - prime2;
    }

    static int multiply(int n1, int n2) {
        int prime1 = nthPrime(n1);
        int prime2 = nthPrime(n2);

        return prime1 * prime2;
    }

    static int divide(int n1, int n2) {
        int prime1 = nthPrime(n1);
        int prime2 = nthPrime(n2);

        if (prime2 == 0) {
            throw std::invalid_argument("Cannot divide by zero");
        }

        return prime1 / prime2;
    }
};
