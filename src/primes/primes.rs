use rand::Rng;

pub struct Primes {}

impl Primes {
    pub fn is_prime(n: usize) -> bool {
        if n < 2 {
            return false;
        }
        let mut i = 2;
        while i*i <= n {
            if n%i == 0 {
                return false;
            }
            i+=1;
        }
        true
    }
    pub fn next_prime(number: usize) -> usize {
        let mut n = number;
        loop {
            if Primes::is_prime(n) {
                return n
            }
            n+=1
        }
    }
    pub fn get_prime_divisors(n: usize) -> Vec<usize> {
        let mut m = n;
        let mut i = 2;
        let mut prime_divisors = Vec::new();
        while i*i < n {
            if m%i == 0 {
                prime_divisors.push(i);
                while m%i == 0 {
                    m/=i;
                }
            }
            i+=1
        }
        if m > 1 {
            prime_divisors.push(m);
        }
        prime_divisors
    }
    pub fn fpow(base: usize, exponent: usize, modulus: usize) -> usize {
        let mut res = 1;
        let mut k = exponent;
        let mut a = base;
        while k > 0 {
            if k%2 == 1 {
                res = (res*a)%modulus;
            }
            a = (a*a)%modulus;
            k/=2;
        }
        res
    }
    pub fn group_generator(p: usize) -> usize {
        let fi = p-1;
        let divisors = Primes::get_prime_divisors(fi);
        let mut rng = rand::thread_rng();
        loop {
            let g = rng.gen_range(1..fi);
            let mut flag = true;
            for divisor in &divisors {
                if Primes::fpow(g, fi/divisor, p) == 1 {
                    flag = false;
                    break;
                }
            }
            if flag {
                return g
            }
        }
    }
    pub fn group_generator_and_size(n: usize) -> (usize, usize) {
        let p = Primes::next_prime(n+1);
        (Primes::group_generator(p), p)
    }
}