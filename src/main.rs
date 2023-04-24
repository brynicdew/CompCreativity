use std::io;

fn is_prime(n: u32) -> bool {
    /* Return true if n is prime, else false. */
    if n <= 1 {
        return false;
    }
    let sqrt_n = (n as f64).sqrt() as u32;
    for i in 2..=sqrt_n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn nth_prime(n: u32) -> u32 {
    /* Return the nth prime number. */
    let mut count = 0;
    let mut i = 2;
    loop {
        if is_prime(i) {
            count += 1;
            if count == n {
                return i;
            }
        }
        i += 1;
    }
}

fn add_primes(a: u32, b: u32) -> u32 {
    /* Return the sum of the a-th and b-th prime numbers. */
    nth_prime(a) + nth_prime(b)
}

fn subtract_primes(a: u32, b: u32) -> u32 {
    /* Return the difference of the a-th and b-th prime numbers. */
    nth_prime(a) - nth_prime(b)
}

fn multiply_primes(a: u32, b: u32) -> u32 {
    /* Return the product of the a-th and b-th prime numbers. */
    nth_prime(a) * nth_prime(b)
}

fn divide_primes(a: u32, b: u32) -> f32 {
    /* Return the quotient of the a-th and b-th prime numbers. */
    nth_prime(a) as f32 / nth_prime(b) as f32
}

fn main() {
    let mut a = String::new();
    let mut b = String::new();

    println!("Enter the position of the first prime number: ");
    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read input.");

    println!("Enter the position of the second prime number: ");
    io::stdin()
        .read_line(&mut b)
        .expect("Failed to read input.");

    let a: u32 = a.trim().parse().expect("Invalid input.");
    let b: u32 = b.trim().parse().expect("Invalid input.");

    println!("{}-th prime number: {}", a, nth_prime(a));
    println!("{}-th prime number: {}", b, nth_prime(b));
    println!("Sum: {}", add_primes(a, b));
    println!("Difference: {}", subtract_primes(a, b));
    println!("Product: {}", multiply_primes(a, b));
    println!("Quotient: {}", divide_primes(a, b));
}
