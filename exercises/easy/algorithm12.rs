/*
    Palindrome Check
    Given a string, check if it is a palindrome (i.e., it reads the same forward and backward).
    The solution should ignore case differences and non-alphabetical characters.

    You need to implement the function `is_palindrome(s: String) -> bool`.
    The function should return `true` if the string is a palindrome, and `false` otherwise.
    
    Hint: Consider normalizing the string by converting it to lowercase and removing non-alphabetical characters before checking.
*/

use std::fmt::{self, Display, Formatter};

pub fn is_palindrome(s: String) -> bool {
    // 将字符串转换为小写并只保留字母字符
    let cleaned: String = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    // 获取处理后的字符向量
    let chars: Vec<char> = cleaned.chars().collect();
    
    // 使用双指针方法检查是否为回文
    let len = chars.len();
    let mut left = 0;
    let mut right = if len > 0 { len - 1 } else { 0 };

    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_1() {
        let s = "A man, a plan, a canal, Panama".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_2() {
        let s = "Racecar".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_3() {
        let s = "Hello, World!".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_palindrome_4() {
        let s = "No 'x' in Nixon".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_5() {
        let s = "Was it a car or a cat I saw?".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }
}
