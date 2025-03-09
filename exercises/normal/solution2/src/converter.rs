use std::collections::HashMap;

pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let from_base: u32;
    let num_part: &str;

    // 解析输入格式 "number(base)"
    if let Some((n, b)) = num_str.split_once('(') {
        num_part = n;
        from_base = b.trim_end_matches(')').parse().unwrap();
    } else {
        return String::from("Invalid input format");
    }

    // 创建数字到字符的映射
    let digits: Vec<char> = "0123456789abcdef".chars().collect();
    let mut char_to_value: HashMap<char, u32> = HashMap::new();
    for (i, &c) in digits.iter().enumerate() {
        char_to_value.insert(c, i as u32);
    }
    
    // 转换为10进制
    let mut decimal = 0;
    for c in num_part.to_lowercase().chars() {
        if let Some(&value) = char_to_value.get(&c) {
            if value >= from_base {
                return String::from("Invalid digit for given base");
            }
            decimal = decimal * from_base + value;
        } else {
            return String::from("Invalid character in input");
        }
    }
    
    // 转换为目标进制
    if decimal == 0 {
        return String::from("0");
    }
    
    let mut result = String::new();
    let mut n = decimal;
    while n > 0 {
        let remainder = (n % to_base) as usize;
        result.insert(0, digits[remainder]);
        n /= to_base;
    }
    
    result
}
