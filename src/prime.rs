use std::collections::HashSet;

pub struct PrimeSieve {
    non_primes: HashSet<u64>,
}

impl PrimeSieve {
    fn new() -> PrimeSieve {
        PrimeSieve {
            non_primes: HashSet::new(),
        }
    }

    fn is_prime(&mut self, n: u64) -> bool {
        for i in 2..n {
            let mut j = 1;
            while i * j < n {
                self.non_primes.insert(i * j);
                j += 1;
            }
        }

        self.non_primes.contains(&n)
    }
}

pub fn is_prime(n: u64) -> bool {
    PrimeSieve::new().is_prime(n)
}

// calculate the prime factors of number `n`
pub fn factors(n: u64) -> Vec<u64> {
    let mut sieve = PrimeSieve::new();
    let mut primes = (2..).filter(|&n| sieve.is_prime(n));

    let mut factors = Vec::new();

    let mut dividend = n;
    let mut divisor = primes.next().unwrap();

    while dividend > 1 {
        if dividend % divisor == 0 {
            factors.push(divisor);
            dividend /= divisor;
        } else {
            divisor = primes.next().unwrap();
        }
    }

    factors
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_prime_works() {
        use super::is_prime;

        let primes = vec![2, 3, 5, 7, 9, 11, 127, 151, 211, 251];
        assert!(primes.iter().all(|&p| is_prime(p)));
    }
}
