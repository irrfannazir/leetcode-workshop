struct Solution;

/*
Runtime:    0ms     |   beats 100%
Memory:     2.12mb  |   beats 65.54%
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

fn create_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    if values.is_empty() {
        return None;
    }
    
    let mut head = Box::new(ListNode::new(values[0]));
    let mut current = &mut head;
    
    for &val in values.iter().skip(1) {
        current.next = Some(Box::new(ListNode::new(val)));
        current = current.next.as_mut().unwrap();
    }
    
    Some(head)
}

// Helper function to print a linked list
fn print_list(head: &Option<Box<ListNode>>) {
    let mut current = head;
    let mut result = String::new();
    
    while let Some(node) = current {
        result.push_str(&node.val.to_string());
        result.push_str("->");
        current = &node.next;
    }
    
    if result.is_empty() {
        println!("None");
    } else {
        result.pop(); // Remove last '>'
        result.pop(); // Remove last '-'
        println!("{}", result);
    }
}


//I started programming in leetcode from here

use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();

        for list in lists {
            if let Some(node) = list {
                heap.push(node);
            }
        }

        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while let Some(mut node) = heap.pop() {
            if let Some(next) = node.next.take() {
                heap.push(next);
            }

            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
}

fn main() {
    println!("Test case 1:");
    let list1 = create_list(vec![1, 4, 5]);
    let list2 = create_list(vec![1, 3, 4]);
    let list3 = create_list(vec![2, 6]);
    let lists = vec![list1, list2, list3];
    
    let merged = Solution::merge_k_lists(lists);
    print_list(&merged);
    
    println!();
}
