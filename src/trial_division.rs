// trial_division.rs
// Prime Number Toolkit

/// Returns `true` if `n` is a prime number, `false` otherwise.
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let boundary = isqrt(n);
    let mut divisor = 3;

    while divisor <= boundary {
        if n % divisor == 0 {
            return false;
        }
        divisor += 2;
    }
    true
}

/// Integer square root using Newton's method.
/// This is the Rust equivalent of Python's `math.isqrt()`.
fn isqrt(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;

    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

/// Returns a vector of all prime numbers from 2 up to `limit` (inclusive).
pub fn primes_up_to(limit: u64) -> Vec<u64> {
    if limit < 2 {
        return Vec::new();
    }
    (2..=limit).filter(|&n| is_prime(n)).collect()
}

/// Returns the nth prime number (1-indexed).
/// Example: nth_prime(1) = 2, nth_prime(10) = 29
pub fn nth_prime(n: u64) -> u64 {
    assert!(n >= 1, "n must be 1 or greater, got {n}");

    let mut count = 0;
    let mut candidate = 1;

    while count < n {
        candidate += 1;
        if is_prime(candidate) {
            count += 1;
        }
    }
    candidate
}

/// Returns the count of prime numbers less than or equal to `limit`.
pub fn prime_count(limit: u64) -> usize {
    primes_up_to(limit).len()
}

// ====================== Tests ======================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_individual_numbers() {
        let cases = [
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (9, false),
            (13, true),
            (25, false),
            (97, true),
        ];
        for (number, expected) in cases {
            assert_eq!(is_prime(number), expected, "is_prime({})", number);
        }
    }

    #[test]
    fn list_primes_up_to_a_limit() {
        assert_eq!(primes_up_to(1), Vec::<u64>::new());
        assert_eq!(primes_up_to(10), vec![2, 3, 5, 7]);
        assert_eq!(primes_up_to(50).len(), 15);
    }

    #[test]
    fn counts_primes() {
        assert_eq!(prime_count(10), 4);
        assert_eq!(prime_count(100), 25);
        assert_eq!(prime_count(1000), 168);
    }

    #[test]
    fn finds_the_nth_prime() {
        assert_eq!(nth_prime(1), 2);
        assert_eq!(nth_prime(5), 11);
        assert_eq!(nth_prime(10), 29);
    }

    #[test]
    #[should_panic(expected = "n must be 1 or greater")]
    fn nth_prime_rejects_zero() {
        nth_prime(0);
    }

    #[test]
    fn isqrt_matches_known_values() {
        assert_eq!(isqrt(0), 0);
        assert_eq!(isqrt(1), 1);
        assert_eq!(isqrt(35), 5);
        assert_eq!(isqrt(36), 6);
        assert_eq!(isqrt(37), 6);
    }
}
