#![allow(dead_code)] // Crate level ignore
pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    fn empty() -> LinkedList<T> {
        LinkedList { head: None }
    }

    fn push(&mut self, element: T) {
        // let old_head = std::mem::replace(&mut self.head, None);
        let old_head = self.head.take(); // does the same as mem::replace
        let new_head = Box::new(Node {
            element,
            next: old_head,
        });
        self.head = Some(new_head);
        // match old_head {
        //     None => {
        //         self.head = Some(Box::new(Node{
        //             element,
        //             next: None,
        //         }));
        //     }
        //     Some(n) => {
        //         let new_head = Some(Box::new(Node {
        //             element,
        //             next: Some(n),
        //         }));
        //         self.head = Some(new_head);
        //     }
        // }
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                Some(node.element)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<&T> {
         match &self.head {
             None => None,
             Some(node) => Some(&node.element)
         }
    }
}

struct Node<T> {
    element: T,
    next: Link<T>,
}

// Similar to a Option type
// enum Link {
//     Empty,
//     NonEmpty(Box<Node>),
// }
type Link<T> = Option<Box<Node<T>>>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_with_one_element() {
        let mut list = LinkedList::empty();
        list.push(199);
    }
    #[test]
    fn it_works_with_multiple_elements() {
        let mut list = LinkedList::empty();
        list.push(199);
        list.push(333);
    }

    #[test]
    fn it_can_pop() {
        let mut list = LinkedList::empty();
        list.push(199);
        let output = list.pop();
        assert_eq!(output.unwrap(), 199);
    }

    #[test]
    fn it_can_peek() {
        let mut list = LinkedList::empty();
        list.push(199);
        let output = list.peek();
        assert_eq!(*output.unwrap(), 199);
    }

    #[test]
    fn it_can_push_multiple() {
        let mut list = LinkedList::empty();
        list.push(10);
        list.push(20);
        list.push(30);
        let output = list.pop();
        assert_eq!(output.unwrap(), 30);
        let output = list.pop();
        assert_eq!(output.unwrap(), 20);
        let output = list.pop();
        assert_eq!(output.unwrap(), 10);
    }
}
