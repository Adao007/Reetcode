from typing import Optional
class ListNode: 
    def __init__(self, new_data):
        self.data = new_data
        self.next = None 

class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None: 
        slow, fast = head, head.next 

        # Set slow and fast to end of halves 
        while fast and fast.next: 
            slow = slow.next
            fast = fast.next.next 

        # Prepare to reverse second half 
        second = slow.next 
        slow.next = None
        prev = None 

        while second: 
            tmp = second.next 
            second.next = prev 
            prev = second 
            second = tmp 

        # Finish the rest of the order 
        first = head 
        second = prev

        while second: 
            tmp1, tmp2 = first.next, second.next 
            first.next = second 
            second.next = tmp1 
            first = tmp1 
            second = tmp2 

def main(): 
    node1 = ListNode(1)
    node2 = ListNode(2)
    node3 = ListNode(3)
    node4 = ListNode(4)

    # Link 
    node1.next = node2
    node2.next = node3
    node3.next = node4

    head = node1 

    solver = Solution() 
    solver.reorderList(head) 

    while head:
        print(head.data, end=" -> ")
        head = head.next 
    print("NONE")

if __name__ == "__main__":
    main()
