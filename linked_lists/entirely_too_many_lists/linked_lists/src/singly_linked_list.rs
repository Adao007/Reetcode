type Link<T> = Option<Box<Node<T>>>;

pub struct SinglyLinkedList<T> {
    pub head: Link<T>,
}

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        SinglyLinkedList { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn reverse_list(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();

        while let Some(mut node) = current {
            current = node.next;
            node.next = prev.take();
            prev = Some(node);
        }

        self.head = prev;
    }
}

impl<T: PartialOrd> SinglyLinkedList<T> {
    pub fn merge_list(&mut self, mut other: Self) {
        let mut current = &mut self.head;

        // Process all nodes from the other list
        while other.head.is_some() {
            // If we've reached the end of self, just append the rest of other
            if current.is_none() {
                *current = other.head.take();
                return;
            }

            if let Some(other_node) = other.head.as_ref() {
                if let Some(current_node) = current.as_ref() {
                    if other_node.elem <= current_node.elem {
                        if let Some(mut node) = other.head.take() {
                            other.head = node.next.take();
                            node.next = current.take();
                            *current = Some(node)
                        }
                    }

                    // Move to next node in self
                    if let Some(node) = current {
                        current = &mut node.next;
                    }
                }
            }
        }
    }
}

impl<T> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

// IntoIter Trait Implementation
// Tuple structs are an alternative form of struct,
// useful for trivial wrappers around other types.
pub struct IntoIter<T>(SinglyLinkedList<T>);
impl<T> SinglyLinkedList<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

// Iter Trait Implementation
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> SinglyLinkedList<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

// IterMut Trait Implementation
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> SinglyLinkedList<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::SinglyLinkedList;
    #[test]
    fn basics() {
        let mut list = SinglyLinkedList::new();
        // Check empty list behaves correctly
        assert_eq!(list.pop(), None);

        // Populate list and check for normal removal
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));

        // Check for corrupted data by pushing more ref
        list.push(4);
        list.push(5);
        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
    }
}

#[cfg(test)]
mod peek {
    use super::SinglyLinkedList;
    #[test]
    fn peek() {
        let mut list = SinglyLinkedList::new();
        list.push(1);
        list.push(2);
        assert_eq!(list.peek(), Some(&2));

        list.peek_mut().map(|elem| {
            *elem += 8;
        });

        assert_eq!(list.peek(), Some(&10));
    }
}

#[cfg(test)]
mod iterator {
    use super::SinglyLinkedList;
    #[test]
    fn into_iter() {
        let mut list = SinglyLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // Test turning list into iterator
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = SinglyLinkedList::new();
        let mut j = 3;

        list.push(1);
        list.push(2);
        list.push(3);
        for i in list.iter() {
            assert_eq!(*i, j);
            j -= 1;
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = SinglyLinkedList::new();
        list.push(1);
        list.push(42);
        list.push(100);

        let mut iter_mut = list.iter_mut();
        assert_eq!(iter_mut.next(), Some(&mut 100));
        assert_eq!(iter_mut.next(), Some(&mut 42));
        assert_eq!(iter_mut.next(), Some(&mut 1));
    }
}

#[cfg(test)]
mod reverse {
    use super::SinglyLinkedList;
    #[test]
    fn reverse() {
        let mut list = SinglyLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        list.reverse_list();

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
    }
}

#[cfg(test)]
mod merge {
    use super::SinglyLinkedList;
    #[test]
    fn merge() {
        let mut list = SinglyLinkedList::new();
        list.push(5);
        list.push(1);

        let mut other_list = SinglyLinkedList::new();
        other_list.push(4);
        other_list.push(2);
        other_list.push(1);

        list.merge_list(other_list);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));
    }
}
