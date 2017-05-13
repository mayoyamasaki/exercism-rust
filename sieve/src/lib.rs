pub fn primes_up_to(n: usize) -> Vec<u32> {
    let mut is_primes = vec![true; n];
    let n_sqrt = (n as f32).sqrt().floor() as usize;

    for i in 0..n_sqrt {
        if i == 0 || i == 1 {
            is_primes[i] = false;
        } else {
            let mut j = 0;
            while j < n {
                is_primes[j] = false;
                j += i;
            }
        }
    }

    let mut primes: Vec<u32> = Vec::new();
    for (i, is_prime) in is_primes.into_iter().enumerate() {
        if is_prime {
            primes.push(i as u32);
        }
    }
    primes
}
