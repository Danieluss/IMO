#[cfg(test)]
mod test {
    use imo::primes::primes::Primes;
    #[test]
    fn test_is_prime() {
        assert!(Primes::is_prime(2));
        assert!(Primes::is_prime(13));
        assert!(Primes::is_prime(37));
        assert!(Primes::is_prime(2137));
        assert!(Primes::is_prime(1_000_000_007));
        assert!(Primes::is_prime(1_000_000_009));
        assert!(!Primes::is_prime(1));
        assert!(!Primes::is_prime(4));
        assert!(!Primes::is_prime(7*13));
        assert!(!Primes::is_prime(37*41));
        assert!(!Primes::is_prime(37*41));
    }

    #[test]
    fn test_next_prime() {
        assert_eq!(Primes::next_prime(1), 2);
        assert_eq!(Primes::next_prime(1250), 1259);
        assert_eq!(Primes::next_prime(104130), 104147);

    }

    #[test]
    fn test_get_prime_divisors() {
        assert_eq!(Primes::get_prime_divisors(78), vec![2, 3, 13]);
    }

    #[test]
    fn test_fpow() {
        assert_eq!(Primes::fpow(2, 0, 100), 1);
        assert_eq!(Primes::fpow(6, 13, usize::MAX), 13060694016);
        assert_eq!(Primes::fpow(3, 5, 1000), 243);
        assert_eq!(Primes::fpow(3, 5, 100), 43);
    }

    fn check_group_generating(g: usize, n: usize) {
        let mut v = Vec::new();
        let mut a = 1;
        for _ in 0..n-1 {
            v.push(a);
            a = (a*g)%n;
        }
        v.sort();
        println!("{} {}", g, v.len());
        for i in 0..n-1 {
            assert_eq!(v[i], i+1);
        }
    }

    #[test]
    fn test_group_generator() {
        check_group_generating(Primes::group_generator(13), 13);
        check_group_generating(Primes::group_generator(2137), 2137);
    }

    #[test]
    fn test_group_generator_and_size() {
        let (g, n) = Primes::group_generator_and_size(2135);
        assert_eq!(n, 2137);
        check_group_generating(g, n);
    }
}