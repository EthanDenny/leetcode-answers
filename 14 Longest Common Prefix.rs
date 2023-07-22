impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs[0]
            .chars()
            .enumerate()
            .take_while(|(i, c)| strs.iter().all(|s| s.chars().nth(*i) == Some(*c)))
            .map(|(_, c)| c)
            .collect::<String>()
    }
}
