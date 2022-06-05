enum L1 {
    Node(i32, Box<L1>),
    NIL,
}
use L1::*;

pub fn exp() {
    let list = Node(
        1,
        Box::new(Node(3, Box::new(Node(2, Box::new(Node(3, Box::new(NIL))))))),
    );
}
