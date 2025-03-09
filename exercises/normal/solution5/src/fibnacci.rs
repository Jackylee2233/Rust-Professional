pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    if threshold < 1 {
        return 0;
    }

    // 初始化：前两个数都是1，都要计入和
    let mut sum = 2;  // 初始和为2（1+1）
    let mut prev = 1;
    let mut curr = 2; // 从第三个数开始

    while curr <= threshold {
        if curr % 2 == 1 {
            sum += curr;
        }
        
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    sum
}
