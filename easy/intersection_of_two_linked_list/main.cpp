#include <iostream>

/*
Runtime: beats 6.19%
Memory: beats 99.80%
*/

//   Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};


class Solution {
public:
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
        ListNode *ptrA = headA;
        ListNode *ptrB = headB;
        while(ptrA != ptrB){
            if(ptrA == nullptr){
                ptrA = headA;
            }else{
                ptrA = ptrA -> next;
            }

            if(ptrB == nullptr){
                ptrB = headB;
            }else{
                ptrB = ptrB -> next;
            }
        }
        if(ptrA == ptrB){
            return ptrA;
        }
        return nullptr;
    }
};

int main(){
    ListNode* common = new ListNode(8);
    common->next = new ListNode(10);

    ListNode* headA = new ListNode(1);
    headA->next = new ListNode(2);
    headA->next->next = common;

    ListNode* headB = new ListNode(3);
    headB->next = common;

    Solution sol;
    ListNode* intersection = sol.getIntersectionNode(headA, headB);

    if (intersection)
        std::cout << "Intersection at node with value: " << intersection->val << std::endl;
    else
        std::cout << "No intersection" << std::endl;

    return 0;
}
