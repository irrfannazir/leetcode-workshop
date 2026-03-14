struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {

        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let v1 = l1.as_ref().map_or(0, |node| node.val);
            let v2 = l2.as_ref().map_or(0, |node| node.val);

            let sum = v1 + v2 + carry;
            carry = sum / 10;

            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();

            l1 = match l1 {
                Some(node) => node.next,
                None => None,
            };

            l2 = match l2 {
                Some(node) => node.next,
                None => None,
            };
        }

        dummy.next
    }
}

fn main() {

    // l1 = [2,4,3]
    let l1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 3,
                next: None,
            })),
        })),
    }));

    // l2 = [5,6,4]
    let l2 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode {
                val: 4,
                next: None,
            })),
        })),
    }));

    let result = Solution::add_two_numbers(l1, l2);

    let mut current = &result;
    while let Some(node) = current {
        print!("{} ", node.val);
        current = &node.next;
    }
}