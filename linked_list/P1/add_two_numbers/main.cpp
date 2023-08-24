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

ListNode *addTwoNumbers(ListNode *l1, ListNode *l2)
{
  ListNode *result = new ListNode();
  ListNode *p = result;
  int val = 0;
  do
  {
    if (l1)
    {
      val += l1->val;
      l1 = l1->next;
    }

    if (l2)
    {
      val += l2->val;
      l2 = l2->next;
    }

    p->val = val % 10;
    val /= 10;

    if (l1 || l2 || val)
      p = p->next = new ListNode();

  } while (l1 || l2 || val);

  return result;
}

int main()
{
  // int l1[] = {2, 4, 3}, l2[] = {5, 6, 4};
  int l1[] = {9, 9, 9, 9, 9, 9, 9}, l2[] = {9, 9, 9, 9};
  ListNode *head1 = new ListNode(l1[0]);
  ListNode *head2 = new ListNode(l2[0]);

  ListNode *p1 = head1, *p2 = head2;
  for (int i = 1; i < sizeof(l1) / sizeof(l1[0]); i++)
  {
    p1->next = new ListNode(l1[i]);
    p1 = p1->next;
  }

  for (int i = 1; i < sizeof(l2) / sizeof(l2[0]); i++)
  {
    p2->next = new ListNode(l2[i]);
    p2 = p2->next;
  }

  ListNode *res = addTwoNumbers(head1, head2);

  while (res)
  {
    cout << res->val << " ";
    res = res->next;
  }
  cout << endl;

  return 0;
}