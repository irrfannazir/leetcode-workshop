struct Solution;

/*

*/

impl Solution{

    fn roman_value(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }

    pub fn roman_to_int(s: String) -> i32 {

        let len = s.len();
        let mut total = 0;
        let mut i = 0;

        while i < len {
            let current_val = Self::roman_value(s.as_bytes()[i] as char);

            if i + 1 < len {
                let next_val = Self::roman_value( s.as_bytes()[i + 1] as char);
                if current_val < next_val {
                    total += next_val - current_val;
                    i += 2;
                    continue;
                }
            }

            total += current_val;
            i += 1;
        }

        total
    }
}

fn main(){
    let result = Solution::roman_to_int("IV".to_string());
    println!("Result: {}", result);
}