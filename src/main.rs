use std::convert::TryInto;

pub struct PrimeNumberMath;

impl PrimeNumberMath {
    pub fn is_prime(n: i32) -> bool {
        if n <= 1 {
            return false;
        }

        let max_divisor = (n as f64).sqrt().floor() as i32;

        for i in 2..=max_divisor {
            if n % i == 0 {
                return false;
            }
        }

        true
    }

    pub fn nth_prime(n: i32) -> i32 {
        let mut primes = Vec::new();
        let mut current = 2;

        while primes.len() < n as usize {
            if PrimeNumberMath::is_prime(current) {
                primes.push(current);
            }

            current += 1;
        }

        *primes.last().unwrap()
    }

    pub fn add(n1: i32, n2: i32) -> i32 {
        let prime1 = PrimeNumberMath::nth_prime(n1);
        let prime2 = PrimeNumberMath::nth_prime(n2);

        prime1 + prime2
    }

    pub fn subtract(n1: i32, n2: i32) -> i32 {
        let prime1 = PrimeNumberMath::nth_prime(n1);
        let prime2 = PrimeNumberMath::nth_prime(n2);

        prime1 - prime2
    }

    pub fn multiply(n1: i32, n2: i32) -> i32 {
        let prime1 = PrimeNumberMath::nth_prime(n1);
        let prime2 = PrimeNumberMath::nth_prime(n2);

        prime1 * prime2
    }

    pub fn divide(n1: i32, n2: i32) -> i32 {
        let prime1 = PrimeNumberMath::nth_prime(n1);
        let prime2 = PrimeNumberMath::nth_prime(n2);

        if prime2 == 0 {
            panic!("Cannot divide by zero");
        }

        prime1 / prime2
    }
}

fn main() {
    let n1 = 3;
    let n2 = 4;

    let sum = PrimeNumberMath::add(n1, n2);
    let difference = PrimeNumberMath::subtract(n1, n2);
    let product = PrimeNumberMath::multiply(n1, n2);
    let quotient = PrimeNumberMath::divide(n1, n2);

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
}
