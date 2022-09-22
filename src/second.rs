use std::mem;

// DIY singly list list https://rust-unofficial.github.io/too-many-lists/
// Test at end of file.

// Contents of a list looks like this, which can go on forever:
//  list -> head -> link -> option<box<node>> -> node -> elem+list -> head ...

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

pub struct List {
    head: Link,
}

impl List {
    fn new() -> Self {
        List { head: None }
    }

    // Add element to the front of the list
    // before: LIST^1 -> head^1 -> link^1 -> some<box<node^1>> -> node^1 -> elem^1 + next^1 -> list^2 -> head^2 ...
    // after:  LIST^1 -> head^1 -> link^1 -> some<box<node^3>> -> node^3 -> elem^3 + next^3 -> list^1 -> head^1 -> link^1 -> some<box<node^1>> ...
    fn push(&mut self, elem: i32) {
        // Make node from supplied element value.
        let new_node = Box::new(Node {
            elem,
            // "next" for the new node has to become the same Link as the one currently in the list head, but they can't both own it, so we have to use mem::replace to steal it away from list head temporarily making that None so that it's not an uninitialized value (dangerous and not allowed by rust compiler).
            // mem::replace puts the second param into dest (1st param) and returns the replaced value
            next: mem::replace(&mut self.head, None),
        });
        self.head = Some(new_node);
    }

    // Take front element from front of list
    pub fn pop(&mut self) -> Option<i32> {
        // Same switch around of values as in push() above, have to get head moved out so we can own it before we can set it to the next link in line.
        match mem::replace(&mut self.head, None) {
            None => None, // If list head was None (empty) then we just return None.
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

// This is how destructors are done in rust, by implementing the Drop trait.
// https://rust-unofficial.github.io/too-many-lists/first-drop.html
// We've implemented drop because the default de-allocation of a list would be recursive which could stackoverflow.
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
