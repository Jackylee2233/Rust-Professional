fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn check_goldbach(n: u64) -> bool {
    // 对于给定的n，检查是否存在一个质数p和一个自然数k
    // 使得 n = p + 2k²
    let sqrt_n = ((n as f64) / 2.0).sqrt() as u64;
    
    for k in 0..=sqrt_n {
        let twice_square = 2 * k * k;
        if twice_square >= n {
            break;
        }
        let p = n - twice_square;
        if is_prime(p) {
            return true;
        }
    }
    false
}

pub fn goldbach_conjecture() -> String {
    let mut results = Vec::new();
    let mut n = 1;
    
    // 使用缓存来存储已找到的质数
    while results.len() < 2 {
        n += 2; // 只检查奇数
        if !check_goldbach(n) {
            results.push(n);
        }
    }
    
    format!("{},{}", results[0], results[1])
}
