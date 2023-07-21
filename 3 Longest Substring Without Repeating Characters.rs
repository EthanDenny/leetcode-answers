use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() <= 1 { return s.len() as i32; }

        let mut longest_length = 0;
        let mut symbols: HashSet<char> = HashSet::new();
        let mut sub_ptr = s.chars();
        
        for c in s.chars() {
            if !symbols.contains(&c) {
                symbols.insert(c);

                if symbols.len() > longest_length {
                    longest_length = symbols.len();
                }
            } else {
                while let Some(s) = sub_ptr.next() {
                    if s == c { break; }
                    symbols.remove(&s);
                }
            }
        }

        return longest_length as i32;
    }
}
