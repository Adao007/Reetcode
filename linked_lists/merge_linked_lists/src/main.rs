use num_traits::int::PrimInt;
use linked_lists::singly_linked_list::*;

fn main() {
    let mut list1 = SinglyLinkedList::new();
    let mut list2 = SinglyLinkedList::new();

    // list1 = 1 -> 2 -> 4
    list1.push(4); list1.push(2); list1.push(1);
    // list2 = 1 -> 3 -> 5
   list2.push(5); list2.push(3); list2.push(1);
    let mut list = merge_out_of_place(&mut list1, &mut list2);
    list.reverse_list();

    // list should equal: 1, 1, 2, 3, 4, 5
    for i in list.iter() {
        println!("{:?}", i);
    }
}

// Function is O(n + m) Time Complexity and O(n + m) Space Complexity
fn merge_out_of_place<T: PrimInt>(
    list1:&mut SinglyLinkedList<T>,
    list2:&mut SinglyLinkedList<T>
) -> SinglyLinkedList<T> {
    let mut value1 = list1.pop();
    let mut value2 = list2.pop();
    let mut list = SinglyLinkedList::new();

    loop {
        match(value1, value2) {
            (Some(v1), Some(v2)) => {
                if v1 <= v2 {
                    list.push(v1);
                    value1 = list1.pop();
                }
                else {
                    list.push(v2);
                    value2 = list2.pop();
                }
            },
            (Some(v1), None) => {
                list.push(v1);
                value1 = list1.pop();
            },
            (None, Some(v2)) => {
                list.push(v2);
                value2 = list2.pop();
            }
            (None, None) => { break; },
        }
    }
    list
}