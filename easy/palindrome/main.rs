struct Solution;

/*
Runtime:    0ms     |   beats 100%
Memory:     2.08mb  |   beats 95.19%
*/

//I started programming in leetcode from here

impl Solution {
    fn num_digits(mut x: u32) -> u8{
        if x == 0{
            return 1;
        }
        let mut length: u8 = 0;
        while x != 0 {
            length += 1;
            x /= 10;
        }
        length
    }
    
    fn digit_at(mut x: u32, mut digit: u8) -> u8{
        while digit > 0{
            x /= 10;
            digit -= 1;
        }
        (x % 10) as u8
    }
    

    pub fn is_palindrome(x: i32) -> bool {
        if x < 0{
            return false;
        }
        if x < 10{
            return true;
        }
        let x = x as u32;
        let length = Self::num_digits(x); 
        for i in 0..(length/2) {
            if Self::digit_at(x, i) != Self::digit_at(x, length - i - 1) {
                return false;
            }
        }
        true
    }
}

//To here

fn main() {
    let result = Solution::is_palindrome(121);
    println!("Result: {}", result);
    let result = Solution::is_palindrome(123);
    println!("Result: {}", result);
}