use bit_vec::BitVec;

/// Test for primes using sieve of eratosthenes combined with brute force search
pub struct PrimeSieve {
    bitvec: BitVec,
}

impl PrimeSieve {
    pub fn new(max_prime : usize) -> PrimeSieve {
        let mut bitvec = BitVec::from_elem(max_prime, true);

        bitvec.set(0, false);
        bitvec.set(1, false);

        for i in 2 .. 1 + (max_prime as f64).sqrt() as usize {
            if bitvec[i] {
                for j in i.. {
                    if i * j >= max_prime {
                        break;
                    }
                    bitvec.set(i * j, false);
                }
            }
        }

        PrimeSieve {
            bitvec,
        }
    }

    pub fn is_prime(&self, n: usize) -> bool {
        self.bitvec.get(n).unwrap_or(false)
    }
}

// calculate the prime factors of number `n`
pub fn factors(n: usize) -> Vec<usize> {
    let mut tester = PrimeSieve::new(1 + (n as f32).sqrt() as usize);
    let mut primes = (2..).filter(|&n| tester.is_prime(n));

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
    fn prime_sieve_works() {
        use super::PrimeSieve;

        let mut sieve = PrimeSieve::new(252);

        let primes = vec![2, 3, 5, 7, 11, 13, 127, 151, 211, 251];

        assert!(primes.iter().all(|&p| sieve.is_prime(p)))
    }

    #[test]
    fn generate_primes() {
        use super::PrimeSieve;

        let sieve = PrimeSieve::new(20);

        let primes: Vec<usize> = (0..=20).filter(|&n| sieve.is_prime(n)).collect(); 

        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }
}
