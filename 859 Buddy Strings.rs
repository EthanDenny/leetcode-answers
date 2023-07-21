impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false
        };

        let mut pairs = std::collections::HashMap::new();
        let mut swaps = 0;
        let mut has_identical_pair = false;

        for (c1, c2) in s.chars().zip(goal.chars()) {
            let entry = pairs.entry((c1, c2)).or_insert(0);
            if *entry > 0 {
                *entry -= 1;
                if c1 == c2 {
                    has_identical_pair = true;
                } else {
                    swaps += 1;
                }
            } else {
                *pairs.entry((c2, c1)).or_insert(0) += 1;
            }
        }
        
        pairs.retain(|p, n| p.0 != p.1 && *n > 0);

        pairs.is_empty() && (swaps == 1 || (has_identical_pair && swaps == 0))
    }
}
