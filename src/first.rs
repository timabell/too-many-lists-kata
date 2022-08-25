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
    fn push(&mut self, elem: i32){
        let new_node = Node{
            elem,
            next: self.head,
        };
        self.head = Link::More(new_node);
    }
}
