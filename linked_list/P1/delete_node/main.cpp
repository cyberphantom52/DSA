#include<iostream>
using namespace std;

struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
};

void deleteNode(ListNode* node) {
  ListNode* tmp = node->next;
  node->val = node->next->val;
  node->next = node->next->next;
  delete tmp;
}

int main ()
{
  ListNode *head = new ListNode(4);
  ListNode *node1 = new ListNode(5);
  ListNode *node2 = new ListNode(1);
  ListNode *node3 = new ListNode(9);
  head->next = node1;
  node1->next = node2;
  node2->next = node3;
  deleteNode(node1);
  ListNode *temp = head;
  while (temp != NULL) {
    cout << temp->val << " ";
    temp = temp->next;
  }
  cout << endl;
  return 0;
}