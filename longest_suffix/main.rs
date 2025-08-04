struct Solution;

/*
Runtime:   100% 
Memory:     98.01%
*/

impl Solution{

    fn str_sub(a: String, b: String) -> String{
        let mut res = String::from("");
        for (ca, cb) in a.chars().zip(b.chars()){
            if ca != cb{
                break;
            }
            res.push(ca);
        }
        res
    }

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut res = strs[0].clone();
        for strt in &strs{
            if &res == strt{
                continue;
            }
            res = Self::str_sub(res, strt.clone());
        }
        res
    }
}

fn main(){
    let result = Solution::longest_common_prefix(vec!["Hello".to_string(), "Heavy".to_string()]);
    println!("Result: {}", result);
}