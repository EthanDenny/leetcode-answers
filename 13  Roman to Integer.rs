use std::iter::Peekable;

impl Solution {
    fn add_or_subtract<I: Iterator<Item = char>>(iter: &mut Peekable<I>, n: i32, a: char, b: char) -> i32 {
        if let Some(c) = iter.peek() {
            if *c == a || *c == b {
                return -n;
            }
        }

        n
    }

    pub fn roman_to_int(s: String) -> i32 {
        use std::iter::Peekable;

        let mut n = 0;
        let mut iter = s.chars().peekable();

        while let Some(c) = iter.next() {
            n += match c {
                'I' => Solution::add_or_subtract(&mut iter, 1, 'V', 'X'),
                'V' => 5,
                'X' => Solution::add_or_subtract(&mut iter, 10, 'L', 'C'),
                'L' => 50,
                'C' => Solution::add_or_subtract(&mut iter, 100, 'D', 'M'),
                'D' => 500,
                'M' => 1000,
                _ => 0
            }
        }

        n
    }
}
