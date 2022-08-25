pub struct List {
    head: Link,
}
struct Node {
    elem: i32,
    next: List,
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
}
