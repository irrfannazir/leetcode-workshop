struct Solution;

/*
    Runtime:   100% 
    Memory:     98.36%
*/

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut left_max = 0;
        let mut right_max = 0;
        let mut water = 0;
        while left < right {
            if height[left] < height[right] {
                if height[left] >= left_max {
                    left_max = height[left];
                }else{
                    water += left_max - height[left];
                }
                left += 1;
            }else{
                if height[right] >= right_max {
                    right_max = height[right];
                }else{
                    water += right_max - height[right];
                }
                right -= 1;
            }
        }
        water
    }
}


pub fun main(){
    let result = imp::trap(vec![4,2,0,3,2,5]);
    println!("Result: {}", result);
}