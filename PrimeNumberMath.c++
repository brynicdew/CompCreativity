#include <iostream>
#include <cmath>

using namespace std;

bool isPrime(int num) {
    if (num <= 1) {
        return false;
    }
    int limit = sqrt(num);
    for (int i = 2; i <= limit; ++i) {
        if (num % i == 0) {
            return false;
        }
    }
    return true;
}

int nthPrime(int n) {
    int primeCount = 0;
    int i = 2;
    while (primeCount < n) {
        if (isPrime(i)) {
            primeCount++;
        }
        i++;
    }
    return i - 1;
}

int add(int pos1, int pos2) {
    int prime1 = nthPrime(pos1);
    int prime2 = nthPrime(pos2);
    return prime1 + prime2;
}

int subtract(int pos1, int pos2) {
    int prime1 = nthPrime(pos1);
    int prime2 = nthPrime(pos2);
    return prime1 - prime2;
}

int multiply(int pos1, int pos2) {
    int prime1 = nthPrime(pos1);
    int prime2 = nthPrime(pos2);
    return prime1 * prime2;
}

int divide(int pos1, int pos2) {
    int prime1 = nthPrime(pos1);
    int prime2 = nthPrime(pos2);
    return prime1 / prime2;
}

int main() {
    int pos1, pos2;
    cout << "Enter the position of the first prime number: ";
    cin >> pos1;
    cout << "Enter the position of the second prime number: ";
    cin >> pos2;

    cout << "The sum of the prime numbers is: " << add(pos1, pos2) << endl;
    cout << "The difference of the prime numbers is: " << subtract(pos1, pos2) << endl;
    cout << "The product of the prime numbers is: " << multiply(pos1, pos2) << endl;
    cout << "The quotient of the prime numbers is: " << divide(pos1, pos2) << endl;

    return 0;
}
