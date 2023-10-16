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

ListNode *rotateRight (ListNode *head, int k)
{
  if (head == nullptr)
    return nullptr;

  int size = 1;
  ListNode *temp = head;
  while (temp->next != nullptr)
  {
    size++;
    temp = temp->next;
  }
  
  k = k % size;
  if (k == 0)
    return head;

  temp->next = head;
  int stepsToNewHead = size - k;
  while (stepsToNewHead--)
  {
    temp = temp->next;
  }
  head = temp->next;
  temp->next = nullptr;
  
  return head;
}

int main()
{
  int nums[] = {1, 2, 3, 4, 5};
  ListNode *head = new ListNode(nums[0]);
  ListNode *temp = head;
  for (int i = 1; i < sizeof(nums)/sizeof(nums[i]); i++)
  {
    temp->next = new ListNode(nums[i]);
    temp = temp->next;
  }
  
  head = rotateRight(head, 3);
  
  while (head != nullptr)
  {
    cout << head->val << " ";
    head = head->next;
  }
  cout << endl;

  return 0;
}