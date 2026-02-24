#include <iostream>

//   Definition for singly-linked list.
  struct ListNode {
      int val;
      ListNode *next;
      ListNode() : val(0), next(nullptr) {}
      ListNode(int x) : val(x), next(nullptr) {}
      ListNode(int x, ListNode *next) : val(x), next(next) {}
  };

/*
  Runtim: beats 100%
  Memory: beats 61.7%
*/


class Solution {
public:
    ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
        ListNode* head = new ListNode(0);
        ListNode* tail = head;

        while (list1 && list2) {
            if (list1 -> val < list2 -> val) {
                tail -> next = list1;
                list1 = list1 -> next;
            } else {
                tail -> next = list2;
                list2 = list2 -> next;
            }
            tail = tail -> next;
        }

        if (list1) tail->next = list1;
        if (list2) tail->next = list2;

        return head -> next;
    }
};

int main(){
    ListNode* l1 = new ListNode(1);
    l1->next = new ListNode(3);
    l1->next->next = new ListNode(5);

    ListNode* l2 = new ListNode(2);
    l2->next = new ListNode(4);
    l2->next->next = new ListNode(6);

    Solution solution;
    ListNode* merged = solution.mergeTwoLists(l1, l2);

    while (merged) {
        std::cout << merged->val << " ";
        merged = merged->next;
    }
    std::cout << std::endl;
    return 0;
}