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

        let chars: Vec<char> = s.chars().collect();
        let mut total = 0;
        let mut i = 0;

        while i < chars.len() {
            let current_val = Self::roman_value(chars[i]);

            if i + 1 < chars.len() {
                let next_val = Self::roman_value( chars[i + 1] );
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