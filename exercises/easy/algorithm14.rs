/*
    Find Duplicates in Array
    Given an array, find all the duplicate elements and return them. 
    You need to solve the problem with O(1) space complexity (i.e., without using extra arrays or hash tables).

    Implement the function `find_duplicates(nums: Vec<i32>) -> Vec<i32>`.
    The function should return a vector containing all the duplicate elements in the array.
    
    Hint: You can modify the input array in place to track duplicates.
*/

use std::fmt::{self, Display, Formatter};

pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut result = Vec::new();
    let n = nums.len();
    
    for i in 0..n {
        // Get the actual value at current position, taking abs in case it was marked negative
        let curr_val = nums[i].abs();
        // Map the value to a valid index (1-based to 0-based indexing)
        let index = ((curr_val - 1) % n as i32) as usize;
        
        if nums[index] > 0 {
            nums[index] = -nums[index];
        } else {
            // If we find a negative number, this value has been seen before
            // Add the original number (not the index)
            result.push(curr_val);
        }
    }
    
    // Sort and remove duplicates
    result.sort_unstable();
    result.dedup();
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicates_1() {
        let nums = vec![1, 2, 3, 4, 5, 6, 2, 3];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_find_duplicates_2() {
        let nums = vec![4, 5, 6, 7, 5, 4];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![4, 5]);
    }

    #[test]
    fn test_find_duplicates_3() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_find_duplicates_4() {
        let nums = vec![1, 1, 1, 1, 1];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_find_duplicates_5() {
        let nums = vec![10, 9, 8, 7, 6, 7, 8];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![7, 8]);
    }
}
