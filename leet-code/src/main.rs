struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i_index, i_num) in nums.iter().enumerate() {
            for (j_index, j_num) in nums.iter().enumerate() {
                if i_index == j_index {
                    continue;
                }
                if i_num + j_num == target {
                    return vec![i_index as i32, j_index as i32];
                }
            }
        }
        vec![]
    }
}

fn main() {
    let ret = Solution::two_sum(vec![2, 7, 11, 15], 9);
    println!("{:?}", ret);
}
