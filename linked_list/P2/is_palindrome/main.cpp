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

ListNode *reverse(ListNode *head)
{
  ListNode *prev = NULL;
  ListNode *frwd = NULL;

  while (head != NULL)
  {
    frwd = head->next;
    head->next = prev;
    prev = head;
    head = frwd;
  }
  return prev;
}

bool isPalindrome(ListNode *head)
{
  if (head == NULL)
    return true;
  
  ListNode *slow = head;
  ListNode *fast = head;
  ListNode *temp = head;

  // find the middle of the list
  while (fast->next != nullptr && fast->next->next != nullptr)
  {
    slow = slow->next;
    fast = fast->next->next;
  }
  
  // reverse the second half of the list
  slow->next = reverse(slow->next);
  slow = slow->next;

  // compare the first half and second half
  while (slow != NULL)
  {
    if (temp->val != slow->val)
      return false;
    slow = slow->next;
    temp = temp->next;
  }

  return true;
}

int main()
{
  ListNode *head = new ListNode(1);
  ListNode *node1 = new ListNode(2);
  ListNode *node2 = new ListNode(2);
  ListNode *node3 = new ListNode(1);
  head->next = node1;
  node1->next = node2;
  node2->next = node3;
  cout << isPalindrome(head) << endl;
  return 0;
}