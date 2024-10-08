use rand::Rng;
use rug::{Integer, rand::RandState};

fn first_100_primes() -> Vec<Integer> {
    vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
        401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
        509, 521, 523, 541
    ].into_iter().map(Integer::from).collect()
}

fn is_divisible_by_small_primes(candidate: &Integer, primes: &[Integer]) -> bool {
    for prime in primes {
        if candidate.is_divisible(prime) {
            return true;
        }
    }
    false
}


pub fn random_small_prime() -> Integer {
    let primes = first_100_primes();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..primes.len());
    primes[index].clone()
}

pub fn generate_large_prime(bits: u32, state:&mut RandState<'_>) -> Integer {

    let small_primes = first_100_primes();

    loop {
        let candidate = Integer::from(Integer::random_bits(bits, state));

        if is_divisible_by_small_primes(&candidate, &small_primes) {
            continue;
        }
        
        if candidate.is_probably_prime(40) != rug::integer::IsPrime::No {
            return candidate;
        }
    }
}