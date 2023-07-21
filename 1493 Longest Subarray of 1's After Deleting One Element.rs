impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut max_length = 0;
        let mut zero_count = 0;

        for (end, num) in nums.iter().enumerate() {
            if *num == 0 {
                zero_count += 1;
            }

            while zero_count > 1 {
                if nums[start] == 0 {
                    zero_count -= 1;
                }
                start += 1;
            }

            max_length = max_length.max(end - start);
        }

        max_length as i32
    }
}
