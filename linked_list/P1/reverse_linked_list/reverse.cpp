#include <iostream>
using namespace std;

struct ListNode {
  int val;
  ListNode *next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode *next) : val(x), next(next) {}
};

ListNode* reverseList(ListNode* head) {
  ListNode *prev = NULL;
  ListNode *curr = head;
  ListNode *frwd = NULL;

  while (curr != NULL)
  {
    frwd = curr->next;
    curr->next = prev;
    prev = curr;
    curr = frwd;
  }
  return prev;
}

int main() {
  ListNode *head = new ListNode(1);
  ListNode *second = new ListNode(2);
  ListNode *third = new ListNode(3);
  ListNode *fourth = new ListNode(4);
  ListNode *fifth = new ListNode(5);

  head->next = second;
  second->next = third;
  third->next = fourth;
  fourth->next = fifth;

  ListNode *temp = head;
  while (temp != NULL)
  {
    cout << temp->val << " ";
    temp = temp->next;
  }
  cout << endl;

  ListNode *newHead = reverseList(head);
  temp = newHead;
  while (temp != NULL)
  {
    cout << temp->val << " ";
    temp = temp->next;
  }
  cout << endl;
  return 0;
}