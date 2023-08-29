#include <iostream>
using namespace std;

struct ListNode
{
  int val;
  ListNode *next;
  ListNode () : val(0), next (NULL) {}
  ListNode (int x) : val(x), next (NULL) {}
  ListNode (int x, ListNode *next) : val (x), next (next) {}
};

ListNode* mergeTwoLists (ListNode* list1, ListNode* list2)
{
  ListNode* dummy = new ListNode ();
  ListNode* curr = dummy;
  while (list1 && list2)
  {
    if (list1->val > list2->val) {
      curr->next = new ListNode (list2->val);
      list2 = list2->next;
    } else {
      curr->next = new ListNode (list1->val);
      list1 = list1->next;
    }
    curr = curr->next;
  }

  if (list1)
    curr->next = list1;
  
  if (list2)
    curr->next = list2;
  
  return dummy->next;
}

int main ()
{
  ListNode* list1 = new ListNode (1, new ListNode (2, new ListNode (4)));
  ListNode* list2 = new ListNode (1, new ListNode (3, new ListNode (4)));
  ListNode* mergedList = mergeTwoLists (list1, list2);
  while (mergedList)
  {
    cout << mergedList->val << " ";
    mergedList = mergedList->next;
  }
  cout << endl;
  return 0;
}