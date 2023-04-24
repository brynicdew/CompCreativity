import java.util.Scanner;

public class PrimeNumberMath {

    public static boolean isPrime(int num) {
        if (num < 2) {
            return false;
        }
        for (int i = 2; i <= Math.sqrt(num); i++) {
            if (num % i == 0) {
                return false;
            }
        }
        return true;
    }

    public static int nthPrime(int n) {
        int count = 0;
        int i = 2;
        while (count < n) {
            if (isPrime(i)) {
                count++;
            }
            i++;
        }
        return i - 1;
    }

    public static int addPrimes(int pos1, int pos2) {
        int prime1 = nthPrime(pos1);
        int prime2 = nthPrime(pos2);
        return prime1 + prime2;
    }

    public static int subtractPrimes(int pos1, int pos2) {
        int prime1 = nthPrime(pos1);
        int prime2 = nthPrime(pos2);
        return prime1 - prime2;
    }

    public static int multiplyPrimes(int pos1, int pos2) {
        int prime1 = nthPrime(pos1);
        int prime2 = nthPrime(pos2);
        return prime1 * prime2;
    }

    public static int dividePrimes(int pos1, int pos2) {
        int prime1 = nthPrime(pos1);
        int prime2 = nthPrime(pos2);
        return prime1 / prime2;
    }

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        System.out.print("Enter the position of the first prime number: ");
        int pos1 = scanner.nextInt();
        System.out.print("Enter the position of the second prime number: ");
        int pos2 = scanner.nextInt();

        int sum = addPrimes(pos1, pos2);
        int difference = subtractPrimes(pos1, pos2);
        int product = multiplyPrimes(pos1, pos2);
        int quotient = dividePrimes(pos1, pos2);

        System.out.println("The sum is: " + sum);
        System.out.println("The difference is: " + difference);
        System.out.println("The product is: " + product);
        System.out.println("The quotient is: " + quotient);
    }
}
