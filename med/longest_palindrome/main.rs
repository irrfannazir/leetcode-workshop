struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();

        if n < 2 {
            return s;
        }

        let mut start = 0;
        let mut max_len = 1;

        for i in 0..n {

            let mut l = i as i32;
            let mut r = i as i32;

            while l >= 0 && r < n as i32 && chars[l as usize] == chars[r as usize] {
                if (r - l + 1) as usize > max_len {
                    start = l as usize;
                    max_len = (r - l + 1) as usize;
                }
                l -= 1;
                r += 1;
            }

            let mut l = i as i32;
            let mut r = i as i32 + 1;

            while l >= 0 && r < n as i32 && chars[l as usize] == chars[r as usize] {
                if (r - l + 1) as usize > max_len {
                    start = l as usize;
                    max_len = (r - l + 1) as usize;
                }
                l -= 1;
                r += 1;
            }
        }

        chars[start..start + max_len].iter().collect()
    }
}

fn main() {
    let s = String::from("babad");
    let result = Solution::longest_palindrome(s);
    println!("{}", result);
}