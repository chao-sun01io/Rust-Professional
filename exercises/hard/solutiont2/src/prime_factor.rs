pub fn find_max_prime_factor(mut number: u128) -> u128 {
    let mut max_prime_factor = 1;

    // Divide by 2 until number is odd
    while number % 2 == 0 {
        max_prime_factor = 2;
        number /= 2;
    }

    while number % 3 == 0 {
        max_prime_factor = 3;
        number /= 3;
    }

    // Iterate from 3 to sqrt(number) with step 2
    let mut i = 5;
    while i * i <= number {
        while number % i == 0 {
            max_prime_factor = i;
            number /= i;
        }
        while number % (i + 2) == 0 {
            max_prime_factor = (i + 2);
            number /= (i + 2);
        }
        i += 6;
    }

    // If number is greater than 2, it is a prime number
    if number > 2 {
        max_prime_factor = number;
    }

    max_prime_factor
}
