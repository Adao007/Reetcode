from typing import Optional 
class ListNode: 
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next 

class Solution: 
    def remove(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        dummy = ListNode(0, head)
        fast = head
        slow = dummy 

        for i in range(n):
            fast = fast.next 

        while fast: 
            slow = slow.next 
            fast = fast.next 

        slow.next = slow.next.next 
        return dummy.next 
    
def main():
    node1 = ListNode(1)
    node2 = ListNode(2)
    node3 = ListNode(3)
    node4 = ListNode(4)
    node1.next = node2
    node2.next = node3
    node3.next = node4

    solver = Solution() 
    head = solver.remove(node1, 2)

    while head: 
        print(head.val, end=" -> ")
        head = head.next 
    print("NONE")

if __name__ == "__main__": 
    main() 