struct Solution;

//I started programming in leetcode from here

impl Solution {
    fn num_digits(mut x: i32) -> i32{
        if x == 0{
            return 1;
        }
        let mut length: i32 = 0;
        while x != 0 {
            length += 1;
            x /= 10;
        }
        length
    }

    fn digit_at(mut x: i32, mut digit: i32) -> i32{
        while digit > 0{
            x /= 10;
            digit -= 1;
        }
        x % 10
    }

    pub fn is_palindrome(x: i32) -> bool {
        if x < 0{
            return false;
        }
        let length: i32 = Self::num_digits(x); 
        for i in 0..(length/2) {
            if Self::digit_at(x, i) != Self::digit_at(x, length - i - 1) {
                return false;
            }
        }
        return true;
    }
}

//To here

fn main() {
    let result = Solution::is_palindrome(121);
    println!("Result: {}", result);
    let result = Solution::is_palindrome(123);
    println!("Result: {}", result);
}