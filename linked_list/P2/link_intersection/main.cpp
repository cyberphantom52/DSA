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

ListNode *getIntersectionNode(ListNode *headA, ListNode *headB)
{
  ListNode *tmp = headA;
  ListNode *result = nullptr;
  while (tmp->next != NULL)
    tmp = tmp->next;
  tmp->next = headA;

  result = detectCycle(headB);
  tmp->next = NULL;

  return result;
}

int main()
{
  ListNode *headA = new ListNode(4);
  ListNode *headB = new ListNode(5);
  ListNode *headC = new ListNode(8);

  headA->next = new ListNode(1);
  headA->next->next = headC;

  headB->next = new ListNode(6);
  headB->next->next = new ListNode(1);
  headB->next->next->next = headC;

  headC->next = new ListNode(4);
  headC->next->next = new ListNode(5);

  ListNode *result = getIntersectionNode(headA, headB);
  if (result)
    cout << result->val << endl;
  else
    cout << "No intersection" << endl;

  return 0;
}