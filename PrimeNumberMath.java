public class PrimeNumberMath {

    public static boolean isPrime(int n) {
        if (n <= 1) {
            return false;
        }

        for (int i = 2; i <= Math.sqrt(n); i++) {
            if (n % i == 0) {
                return false;
            }
        }

        return true;
    }

    public static int nthPrime(int n) {
        int count = 0;
        int num = 2;

        while (count < n) {
            if (isPrime(num)) {
                count++;
            }
            num++;
        }

        return num - 1;
    }

    public static int add(int n1, int n2) {
        int prime1 = nthPrime(n1);
        int prime2 = nthPrime(n2);

        return prime1 + prime2;
    }

    public static int subtract(int n1, int n2) {
        int prime1 = nthPrime(n1);
        int prime2 = nthPrime(n2);

        return prime1 - prime2;
    }

    public static int multiply(int n1, int n2) {
        int prime1 = nthPrime(n1);
        int prime2 = nthPrime(n2);

        return prime1 * prime2;
    }

    public static int divide(int n1, int n2) {
        int prime1 = nthPrime(n1);
        int prime2 = nthPrime(n2);

        if (prime2 == 0) {
            throw new ArithmeticException("Cannot divide by zero");
        }

        return prime1 / prime2;
    }
}
