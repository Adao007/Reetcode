class Solution:
    def findCycle(self, head: Optional[ListNode]) -> bool:
        slow = head
        fast = head 

        while fast and fast.next:
            slow.next
            fast.next.next

            if slow == fast:
                return True 
            
        return False