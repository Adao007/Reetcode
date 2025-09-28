// Definition for singly-linked list. 
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}
impl ListNode{
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

// Set current -> 
// Set next -> prev
// Set prev -> 

fn main() {
    // Given a singly linked list, reverse the list so that the 
    // elements in the back are now in the front... 
    // head = [0, 1, 2, 3] -> output = [3, 2, 1, 0]
    
}

// [0] -> [1] -> [2] -> [3]
// head
// 