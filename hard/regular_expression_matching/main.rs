struct Solution;

/*
Runtime:    0ms     |   beats 100%
Memory:     2.12mb  |   beats 65.91%
*/

//I started programming in leetcode from here

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let chars_s: Vec<char> = s.chars().collect();
        let chars_p: Vec<char> = p.chars().collect();
        let m = chars_s.len();
        let n = chars_p.len();
        
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;
        
        for j in 1..=n {
            if j >= 2 && chars_p[j - 1] == '*' {
                dp[0][j] = dp[0][j - 2];
            }
        }
        
        for i in 1..=m {
            for j in 1..=n {
                let pch = chars_p[j - 1];
                if pch == '*' {
                    if j >= 2 {
                        dp[i][j] = dp[i][j - 2];
                        let prev_p = chars_p[j - 2];
                        let sch = chars_s[i - 1];
                        if prev_p == '.' || prev_p == sch {
                            dp[i][j] = dp[i][j] || dp[i - 1][j];
                        }
                    }
                } else {
                    let sch = chars_s[i - 1];
                    if pch == '.' || pch == sch {
                        dp[i][j] = dp[i - 1][j - 1];
                    }
                }
            }
        }
        
        dp[m][n]
    }
}

fn main() {
    println!("{}", Solution::is_match("aa".to_string(), "a".to_string())); 
    println!("{}", Solution::is_match("aa".to_string(), "a*".to_string()));
    println!("{}", Solution::is_match("ab".to_string(), ".*".to_string()));
    println!("{}", Solution::is_match("aab".to_string(), "c*a*b".to_string()));
    println!("{}", Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()));
}
