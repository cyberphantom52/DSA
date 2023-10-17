#include <iostream>
#include <unordered_map>
using namespace std;

class Node
{
public:
  int val;
  Node *next;
  Node *random;

  Node(int _val)
  {
    val = _val;
    next = NULL;
    random = NULL;
  }
};

/* Old Solution
Node *copyRandomList(Node *head)
{
  unordered_map<Node *, Node *> map;
  Node *curr = head;
  while (curr != NULL)
  {
    map[curr] = new Node(curr->val);
    curr = curr->next;
  }

  curr = head;
  while (curr != NULL)
  {
    map[curr]->next = map[curr->next];
    map[curr]->random = map[curr->random];
    curr = curr->next;
  }

  return map[head];
}
*/
Node *copyRandomList(Node *head)
{
  Node *curr = head;
  Node *temp = head;
  while (curr != nullptr)
  {
    temp = curr->next;
    curr->next = new Node(curr->val);
    curr->next->next = temp;
    curr = temp;
  }

  curr = head;
  while (curr  != nullptr)
  {
    if (curr->random != nullptr)
      curr->next->random = curr->random->next;
    curr = curr->next->next;
  }

  Node* dummy = new Node(0);
  curr = head;
  temp = dummy;
  Node* fast;
  while (curr != nullptr)
  {
    fast = curr->next->next;
    temp->next = curr->next;
    curr->next = fast;
    temp = temp->next;
    curr = fast;
  }

  return dummy->next;
}

int main()
{
  Node *A = new Node(7);
  Node *B = new Node(13);
  Node *C = new Node(11);
  Node *D = new Node(10);
  Node *E = new Node(1);

  A->next = B;
  B->next = C;
  C->next = D;
  D->next = E;

  B->random = A;
  C->random = E;
  D->random = C;
  E->random = A;

  Node *copy = copyRandomList(A);
  while (copy != NULL)
  {
    cout << "val:" << copy->val << endl;
    if (copy->random != NULL)
      cout << "random:" << copy->random->val << endl;
    copy = copy->next;
  }
  return 0;
}