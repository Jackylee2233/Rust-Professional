
fn sieve(limit: usize) -> Vec<u128> {
    let mut is_prime = vec![true; limit];
    for i in 2..is_prime.len() {
        if is_prime[i] {
            let mut j = i * i;
            while j < is_prime.len() {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    let mut primes = Vec::new();
    for i in 2..is_prime.len() {
        if is_prime[i] {
            primes.push(i as u128);
        }
    }

    primes
}

pub fn find_max_prime_factor(number: u128) -> u128 {
    let mut max_prime = 2;
    let mut n = number;
    let primes = sieve(1_000_000);

    // 去除所有因子 2
    while n % 2 == 0 {
        n /= 2;
    }

    // 检查素数表中的因子
    for &prime in primes.iter() {
        if prime * prime > n {
            break;
        }
        while n % prime == 0 {
            max_prime = prime;
            n /= prime;
        }
    }

    // 处理剩余的数
    if n > 2 {
        max_prime = n;
    }

    max_prime
}