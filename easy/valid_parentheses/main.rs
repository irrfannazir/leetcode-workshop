struct Solution;

/*
Runtime:   100% 
Memory:     34.04%
*/

impl Solution{
    fn ismatch(a: char, b: char) -> bool{
        if (
            a == '[' && b == ']'
        ) || (
            a == '{' && b == '}'
        ) || (
            a == '(' && b == ')'
        ){
            return true;
        }
        false
    }

    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for i in s.chars(){
            if i == '[' || i == '{' || i == '('{
                stack.push(i);
            }else if i == ']' || i == '}' || i == ')'{
                let last = stack.pop();
                if last == None{
                    return false
                }
                if !Self::ismatch(last.unwrap(), i){
                    return false
                }
            }
        }
        if stack.is_empty(){
            return true;
        }else{
            return false;
        }
    }
}

fn main(){
    let result = Solution::is_valid("]".to_string());
    println!("Result: {}", result);
}