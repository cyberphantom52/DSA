#include <iostream>
using namespace std;

struct ListNode
{
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
};

ListNode *detectCycle(ListNode *head)
{
  ListNode *fast = head;
  ListNode *slow = head;
  while (fast != NULL && fast->next != NULL)
  {
    slow = slow->next;
    fast = fast->next->next;
    if (slow == fast)
    {
      slow = head;
      while (slow != fast)
      {
        slow = slow->next;
        fast = fast->next;
      }
      return slow;
    }
  }

  return NULL;
}

int main()
{
  ListNode *head = new ListNode(3);
  ListNode *node1 = new ListNode(2);
  ListNode *node2 = new ListNode(0);
  ListNode *tail = new ListNode(-4);
  head->next = node1;
  node1->next = node2;
  node2->next = tail;
  tail->next = node1;
  ListNode *result = detectCycle(head);
  if (result == NULL)
    cout << "No cycle" << endl;
  else
    cout << "Cycle detected at " << result->val << endl;
}