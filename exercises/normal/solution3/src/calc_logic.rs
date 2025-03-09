pub fn new_birthday_probability(n: u32) -> f64 {
    if n < 2 {
        return 0.0;
    }
    
    // 计算所有人生日都不相同的概率
    let mut not_same_prob = 1.0;
    for i in 0..n {
        // 使用 365.0 确保浮点数运算
        not_same_prob *= (365.0 - i as f64) / 365.0;
    }
    
    // 至少两个人生日相同的概率 = 1 - 所有人生日都不相同的概率
    let result = 1.0 - not_same_prob;
    
    // 四舍五入到4位小数
    (result * 10000.0).round() / 10000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_birthday_probability() {
        assert_eq!(new_birthday_probability(2), 0.0027); // 2人
        assert_eq!(new_birthday_probability(10), 0.1169); // 10人
        assert_eq!(new_birthday_probability(23), 0.5073); // 23人
        assert_eq!(new_birthday_probability(50), 0.9704); // 50人
    }
}
