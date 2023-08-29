#include <iostream>
using namespace std;

struct ListNode
{
  int val;
  ListNode *next;
  ListNode(int x) : val (x), next(NULL) {}
};

bool hasCycle (ListNode *head)
{
  if (head == NULL)
    return false;

  ListNode *fast = head;
  ListNode *slow = head;
  while (fast->next != NULL && fast->next->next != NULL)
  {
    fast = fast->next->next;
    slow = slow->next;
    if (fast == slow)
      return true;
  }
  return false;
}

int main ()
{
  ListNode *head = new ListNode(3);
  ListNode *node1 = new ListNode(2);
  ListNode *node2 = new ListNode(0);
  ListNode *node3 = new ListNode(-4);
  head->next = node1;
  node1->next = node2;
  node2->next = node3;
  node3->next = node1;
  cout << hasCycle(head) << endl;
}