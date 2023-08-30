#include <iostream>
using namespace std;

struct ListNode
{
  int val;
  ListNode *next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode *next) : val(x), next(next) {}
};

ListNode *removeNthFromEnd(ListNode *head, int n)
{
  ListNode *dummy = new ListNode(-1, head);
  ListNode *fast = dummy;
  ListNode *slow = dummy;

  while (n--)
  {
    fast = fast->next;
  }

  while (fast->next != nullptr)
  {
    fast = fast->next;
    slow = slow->next;
  }

  slow->next = slow->next->next;

  return dummy->next;
}

int main()
{
  int list[] = {1, 2, 3, 4, 5};
  int n = 1;

  ListNode *head = new ListNode(list[0]);
  ListNode *curr = head;

  for (int i = 1; i < sizeof(list) / sizeof(list[0]); i++)
  {
    curr->next = new ListNode(list[i]);
    curr = curr->next;
  }

  ListNode *result = removeNthFromEnd(head, n);
  if (result->next->next->next->next == nullptr)
  {
    cout << "last node is nullptr" << endl;
  }
  return 0;
  while (result != nullptr)
  {
    cout << result->val << " ";
    result = result->next;
  }
  cout << endl;

  return 0;
}