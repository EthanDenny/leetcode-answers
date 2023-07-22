impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut move_amount = 0;

        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] { move_amount += 1 };
            nums[i - move_amount] = nums[i];
        }

        (nums.len() - move_amount) as i32
    }
}
