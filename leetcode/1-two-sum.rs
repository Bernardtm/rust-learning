impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (index1, number1) in nums.iter().enumerate() {
            for (index2, number2) in nums.iter().enumerate().skip(index1 + 1) {
                if (number1 + number2) == target {
                    return vec![index1 as i32, index2 as i32];
                }
            }
        }
        vec![]
    }
}