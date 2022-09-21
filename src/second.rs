use std::mem;

//  list -> head -> link -> box<node> -> node -> elem+list -> head ...
pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

// diy Option, replaced with type alias Link above
// enum Link {
//     None,
//     Some(Box<Node>),
// }

////////////

impl List {
    fn new() -> Self {
        List { head: None }
    }
    // before: LIST^1 -> head^1 -> link^1 -> more<box<node^1>> -> node^1 -> elem^1 + next^1 -> list^2 -> head^2 ...
    // after:  LIST^1 -> head^1 -> link^1 -> more<box<node^3>> -> node^3 -> elem^3 + next^3 -> list^1 -> head^1 -> link^1 -> more<box<node^1>> ...
    fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, None),
        });
        self.head = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
