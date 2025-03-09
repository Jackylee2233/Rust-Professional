/*
    Nth Fibonacci Number
    Implement a function to calculate the `n`th Fibonacci number. 
    The Fibonacci sequence is defined as follows:
    F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n > 1.

    You need to implement the function `fib(n: i32) -> i32` to return the `n`th Fibonacci number.
    
    Hint: Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
*/

use std::fmt::{self, Display, Formatter};

// 定义2x2矩阵结构体
#[derive(Clone, Copy)]
struct Matrix {
    data: [[i32; 2]; 2]
}

impl Matrix {
    // 矩阵乘法
    fn multiply(&self, other: &Matrix) -> Matrix {
        let mut result = Matrix { data: [[0; 2]; 2] };
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }

    // 矩阵快速幂
    fn power(self, mut n: i32) -> Matrix {
        let mut result = Matrix { data: [[1, 0], [0, 1]] }; // 单位矩阵
        let mut base = self;
        
        while n > 0 {
            if n & 1 == 1 {
                result = result.multiply(&base);
            }
            base = base.multiply(&base);
            n >>= 1;
        }
        result
    }
}

pub fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    
    // 构建基础矩阵 [[1, 1], [1, 0]]
    let base = Matrix { data: [[1, 1], [1, 0]] };
    
    // 计算矩阵的n-1次幂
    let result = base.power(n - 1);
    
    // 返回矩阵中的[0][0]元素，即为第n个斐波那契数
    result.data[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let result = fib(0);
        println!("Fibonacci of 0: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fib_2() {
        let result = fib(1);
        println!("Fibonacci of 1: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_3() {
        let result = fib(2);
        println!("Fibonacci of 2: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_4() {
        let result = fib(3);
        println!("Fibonacci of 3: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fib_5() {
        let result = fib(10);
        println!("Fibonacci of 10: {}", result);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fib_6() {
        let result = fib(20);
        println!("Fibonacci of 20: {}", result);
        assert_eq!(result, 6765);
    }
}
