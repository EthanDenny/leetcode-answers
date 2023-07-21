impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let mut i = 0;

        for n in nums.iter() {
            let diff = target - n;
            if map.contains_key(&diff) {
                return vec![*map.get(&diff).unwrap(), i];
            }
            else {
                map.insert(n, i);
            }
            i += 1;
        }

        return vec![-1, -1];
    }
}
