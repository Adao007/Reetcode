"""
# Definition for a Node 
class Node: 
    def __init__(self, x: int, next: 'Node' = None, random: 'Node' = Node):
        self.val = int(x)
        self.next = next
        self.random = random 
"""

class Solution: 
    def copyList(self, head: Optional[Node]) -> Optional[Node]:
        lookUp = {None: None}

        curr = head 
        while curr: 
            copy = Node(curr.val)
            lookUp[curr] = copy 
            curr = curr.next
        
        curr = head
        while curr: 
            copy = lookUp[curr]
            copy.next = lookUp[curr.next]
            copy.random = lookUp[curr.random]
            curr = curr.next 

        return lookUp[head]


