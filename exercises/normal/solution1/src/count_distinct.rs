use std::collections::HashSet;

/// 统计字符串中不重复元素的个数
/// 
/// # Arguments
/// * `input` - 逗号分隔的字符串
/// 
/// # Returns
/// * `usize` - 不重复元素的个数
/// 
pub fn new_count_distinct(input_str: &str) -> usize {
    if input_str.is_empty() {
        return 0;
    }

    // 将输入字符串按逗号分割，去除空白，收集到 HashSet 中
    let unique_elements: HashSet<&str> = input_str
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    unique_elements.len()
}
