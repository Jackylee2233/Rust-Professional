pub fn dp_rec_mc(amount: u32) -> u32 {
    if amount == 0 {
        return 0;
    }

    // 可用的面值
    const BILLS: [u32; 8] = [1, 2, 5, 10, 20, 30, 50, 100];
    
    // 创建DP数组，初始化为最大值
    let mut dp = vec![u32::MAX; (amount + 1) as usize];
    dp[0] = 0;  // 基础情况：金额为0时需要0张钞票

    // 对每个金额进行计算
    for i in 1..=amount {
        // 尝试每种面值的钞票
        for &bill in BILLS.iter() {
            if bill <= i {
                // 如果可以使用当前面值
                if let Some(prev_amount) = dp[(i - bill) as usize].checked_add(1) {
                    dp[i as usize] = dp[i as usize].min(prev_amount);
                }
            }
        }
    }

    // 如果无法找零，返回0（根据题目要求）
    if dp[amount as usize] == u32::MAX {
        0
    } else {
        dp[amount as usize]
    }
}
