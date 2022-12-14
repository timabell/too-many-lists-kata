// DIY singly list list https://rust-unofficial.github.io/too-many-lists/
// Test at end of file.

// Contents of a list looks like this, which can go on forever:
//  list -> head -> link -> option<box<node>> -> node -> elem+list -> head ...

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    // Add element to the front of the list
    // before: LIST^1 -> head^1 -> link^1 -> some<box<node^1>> -> node^1 -> elem^1 + next^1 -> list^2 -> head^2 ...
    // after:  LIST^1 -> head^1 -> link^1 -> some<box<node^3>> -> node^3 -> elem^3 + next^3 -> list^1 -> head^1 -> link^1 -> some<box<node^1>> ...
    fn push(&mut self, elem: T) {
        // Make node from supplied element value.
        let new_node = Box::new(Node {
            elem,
            // "next" for the new node has to become the same Link as the one currently in the list head, but they can't both own it, so we have to use take() to steal it away from list head temporarily making that None so that it's not an uninitialized value (dangerous and not allowed by rust compiler).
            // take() puts None into the option its called on and returns the replaced value
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // Take front element from front of list
    pub fn pop(&mut self) -> Option<T> {
        // take() is the same switch around of values as in push() above, have to get head moved out so we can own it before we can set it to the next link in line.
        // map() allows us to apply something to the contents of an Option<x> and get an option with the result https://doc.rust-lang.org/std/option/enum.Option.html#method.map
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    // give me a read-only reference to head without removing it
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
    // give me a mutable reference to head without removing it
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

// This is how destructors are done in rust, by implementing the Drop trait.
// https://rust-unofficial.github.io/too-many-lists/first-drop.html
// We've implemented drop because the default de-allocation of a list would be recursive which could stackoverflow.
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // Steal value of head and replace it with None, giving us an owned reference to it in cur_link (which we'll then repeatedly overwrite as we loop).
        let mut cur_link = self.head.take();
        // `while let` is clever pattern matching voodoo,
        // it's attempting to assign cur_link to pattern Some(boxed_node), which will set boxed_node if it works, otherwise the while will exit
        while let Some(mut _boxed_node) = cur_link {
            // As above, steal the `next` link without leaving next uninitialized.
            cur_link = self.head.take();
            // no need to actually drop inner things (boxed_nde) as they just go out of scope now as nothing else owns them so rust compiler can free them automatically

            // panic!("oops!") // uncomment this to prove that this is called in the deallocate test
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
    #[test]
    fn deallocate() {
        let mut list = List::new();
        list.push(42);
        list.push(33);
        // run free, list!
    }
    #[test]
    fn strings(){
        let mut list = List::new();
        list.push("hi");
        list.push("there");
        let _ = list.pop();
    }
    #[test]
    fn peek(){
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        list.push("hi");
        let peek = list.peek();
        assert_eq!(peek, Some(&"hi"));
    }
    #[test]
    fn peek_mut(){
        let mut list = List::new();
        assert_eq!(list.peek_mut(), None);
        list.push("before");
        // peek_mut().map() gets us contents as a `&mut &str` that we can actually de-reference with * and assign to
        list.peek_mut().map(|contents|{
            *contents = "after"
        });
        assert_eq!(list.peek(), Some(&"after"));
        assert_eq!(list.pop(), Some("after"));
    }
}
