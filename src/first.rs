use std::mem;

//  list -> head -> link -> box<node> -> node -> elem+list -> head ...
pub struct List {
    head: Link,
}
struct Node {
    elem: i32,
    next: Link,
}
enum Link {
    Empty,
    More(Box<Node>),
}

////////////

impl List {
    fn new() -> Self {
        List { head: Link::Empty }
    }
    // before: LIST^1 -> head^1 -> link^1 -> more<box<node^1>> -> node^1 -> elem^1 + next^1 -> list^2 -> head^2 ...
    // after:  LIST^1 -> head^1 -> link^1 -> more<box<node^3>> -> node^3 -> elem^3 + next^3 -> list^1 -> head^1 -> link^1 -> more<box<node^1>> ...
    fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
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
