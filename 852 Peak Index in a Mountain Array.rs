impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut upper = arr.len();
        let mut lower = 0;
        let mut index = arr.len() / 2;

        loop {
            if arr[index + 1] > arr[index] {
                lower = index;
                index = (index + upper) / 2;
            } else if arr[index - 1] > arr[index] {
                upper = index;
                index = (index + lower) / 2;
            } else {
                break;
            }
        }

        index as i32
    }
}
