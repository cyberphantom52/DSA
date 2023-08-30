#include <iostream>
using namespace std;

class Node
{
public:
  int data;
  Node *next;
  Node *child;
  Node() : data(0), next(nullptr), child(nullptr){};
  Node(int x) : data(x), next(nullptr), child(nullptr) {}
  Node(int x, Node *next, Node *child) : data(x), next(next), child(child) {}
};

Node *mergeTwoLists(Node *list1, Node *list2)
{
  Node *dummy = new Node();
  Node *curr = dummy;
  while (list1 && list2)
  {
    if (list1->data > list2->data)
    {
      curr->child = list2;
      list2 = list2->child;
    }
    else
    {
      curr->child = list1;
      list1 = list1->child;
    }
    curr = curr->child;
  }

  if (list1)
    curr->child = list1;

  if (list2)
    curr->child = list2;

  return dummy->child;
}

Node *flattenLinkedList(Node *head)
{
  Node *temp = nullptr;
  while (head && head->next) {
    temp = head->next->next;
    head = mergeTwoLists(head, head->next);
    head->next = temp;
  }

  return head;
}

int main()
{
  Node *head = new Node(39);
  head->child = new Node(40);

  head->next = new Node(44);

  head->next->next = new Node(2);
  head->next->next->child = new Node(5);
  head->next->next->child->child = new Node(33);
  head->next->next->child->child->child = new Node(42);

  head->next->next->next = new Node(29);
  head->next->next->next->child = new Node(46);
  head->next->next->next->child->child = new Node(49);

  head->next->next->next->next = new Node(13);
  head->next->next->next->next->child = new Node(20);

  Node *flattened = flattenLinkedList(head);
  while (flattened != nullptr)
  {
    cout << flattened->data << " ";
    flattened = flattened->child;
  }
  cout << endl;

  return 0;
}