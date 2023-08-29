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

int size(ListNode *head)
{
  int size = 0;
  ListNode *temp = head;
  while (temp != NULL)
  {
    size += 1;
    temp = temp->next;
  }
  return size;
}

ListNode *reverseBetween(ListNode *head, int start, int end)
{
  if (!head || start == end || end > size(head))
    return head;

  ListNode *dummy = new ListNode(0, head);
  ListNode *p = dummy;
  ListNode *tail = NULL;
  int i = 0;

  for (i = 0; i < start - 1; i++)
  {
    p = p->next;
  }
  tail = p->next;

  for (i = start; i < end; i++)
  {
    ListNode *temp = p->next;
    p->next = tail->next;
    tail->next = tail->next->next;
    p->next->next = temp;
  }

  return dummy->next;
}

ListNode *reverseKGroup(ListNode *head, int k)
{
  int numGroup = size(head) / k;
  for (int i = 0; i <= numGroup; i++)
  {
    head = reverseBetween(head, i * k + 1, (i + 1) * k);
  }
  return head;
}

int main()
{
  ListNode *head = new ListNode(1);
  ListNode *temp = head;
  for (int i = 2; i <= 5; i++)
  {
    temp->next = new ListNode(i);
    temp = temp->next;
  }

  head = reverseKGroup(head, 2);
  while (head != NULL)
  {
    cout << head->val << " ";
    head = head->next;
  }
  cout << endl;
  return 0;
}