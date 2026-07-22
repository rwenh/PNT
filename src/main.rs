// src/main.rs
use prime_toolkit::trial_division::{is_prime, nth_prime, prime_count, primes_up_to};

// Helper to print clean section headers
fn print_header(title: &str) {
    println!();
    println!("{}", "=".repeat(50));
    println!("      {title}");
    println!("{}", "=".repeat(50));
}

fn main() {
    // Test is_prime()
    print_header("DAY 1 - Checking individual numbers");

    let test_cases = [
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (9, false),
        (13, true),
        (25, false),
        (97, true),
    ];
    let mut all_passed = true;

    for (number, expected) in test_cases {
        let result = is_prime(number);
        let status = if result == expected { " PASS" } else { " FAIL" };
        if result != expected {
            all_passed = false;
        }
        println!("  {status} | is_prime({number:>3}) = {result:<5} (expected {expected})");
    }

    println!();
    if all_passed {
        println!("   All tests passed! Your is_prime() is working correctly.");
    } else {
        println!("   Some tests failed. Check your is_prime() logic.");
    }

    // Primes up to a limit
    print_header("Primes up to a limit");
    let limit = 50;
    let result = primes_up_to(limit);
    println!("  Primes up to {limit}:");
    println!("  {result:?}");
    println!("  Count: {} primes found", result.len());

    // Prime count for bigger ranges
    println!();
    for ceiling in [100, 500, 1000] {
        let count = prime_count(ceiling);
        println!("  Primes up to {ceiling:>5}: {count:>4} found");
    }

    // Nth prime
    print_header("Find the Nth prime");
    for position in [1, 2, 3, 4, 5, 10, 20, 50, 100] {
        let prime = nth_prime(position);
        let suffix = match position {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };
        println!("      The {position:>3}{suffix} prime is: {prime}");
    }

    print_header("Prime gaps (first 30 primes)");
    let first_30_primes = primes_up_to(115);
    println!("  Prime Gap from previous");
    println!("  -----     ---------------");
    for (i, &p) in first_30_primes.iter().enumerate() {
        if i == 0 {
            println!("   {p:<6} (first prime)");
        } else {
            let gap = p - first_30_primes[i - 1];
            println!("  {p:<6}  +{gap}");
        }
    }
}
