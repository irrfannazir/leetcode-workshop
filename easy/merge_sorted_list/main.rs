struct Solution;

/*
Runtime:   100% 
Memory:     34.04%
*/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution{
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        ;
    }
}

fn main(){
    let result = Solution::is_valid("]".to_string());
    println!("Result: {}", result);
}