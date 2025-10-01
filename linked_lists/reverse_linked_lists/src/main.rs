use linked_lists::singly_linked_list::*; 

fn main() {
    let mut list = SinglyLinkedList::new(); 
    list.push(1); 
    list.push(2);
    list.push(3); 

    // Before reverse
    for i in list.iter() {
        println!("{:?}", i); 
    }

    list.reverse_list(); 
    // After reverse
    println!("The List is now reversed!");
    for i in list.iter() {
        println!("{:?}", i);
    }
}
