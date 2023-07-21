impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim_start();

        if s.is_empty() {
            return 0;
        }
        
        let mut chars = s.chars().peekable();
        let mut sum : i32 = 0;

        let positive = match chars.peek() {
            Some(&'-') => {
                chars.next();
                false
            }
            Some(&'+') => {
                chars.next();
                true
            }
            _ => {
                true
            }
        };

        while chars.peek() == Some(&'0') {
            chars.next();
        }

        for c in chars {
            match c {
                '0' => {
                    sum = sum.saturating_mul(10);
                }
                '1'..='9' => {
                    let d = c.to_digit(10).unwrap() as i32;

                    if positive {
                        sum = sum.saturating_mul(10).saturating_add(d);
                    } else {
                        sum = sum.saturating_mul(10).saturating_sub(d);
                    }
                },
                _ => { break; }
            }
        }

        sum
    }
}
