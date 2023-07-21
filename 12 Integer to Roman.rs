impl Solution {
    fn digits(strvec: &mut Vec<String>, d: i32, d1: &str, d2: &str, d3: &str) {
        match d {
            1..=3 => {
                for _ in 0..d {
                    strvec.push(d1.to_string());
                }
            }
            4 => {
                strvec.push(d1.to_string());
                strvec.push(d2.to_string());
            }
            5..=8 => {
                strvec.push(d2.to_string());
                for _ in 0..(d-5) {
                    strvec.push(d1.to_string());
                }
            }
            9 => {
                strvec.push(d1.to_string());
                strvec.push(d3.to_string());
            }
            _ => {}
        }
    }

    pub fn int_to_roman(num: i32) -> String {
        let mut strvec : Vec<String> = Vec::new();
        
        Solution::digits(&mut strvec, (num % 10000) / 1000, "M", "", "");
        Solution::digits(&mut strvec, (num % 1000) / 100, "C", "D", "M");
        Solution::digits(&mut strvec, (num % 100) / 10, "X", "L", "C");
        Solution::digits(&mut strvec, num % 10, "I", "V", "X");

        strvec.join("")
    }
}
