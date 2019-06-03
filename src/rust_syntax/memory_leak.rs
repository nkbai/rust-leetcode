/*
想在rust中制造内存泄漏也不是件容易的事
*/

//-------------------------循环引用1

//struct Node {
//    next: Option<Box<Node>>,
//}

//fn main() {
//    let mut node1 = Box::new(Node { next: None });
//    let mut node2 = Box::new(Node { next: None });
//
//    node1.next = Some(node2);
//    node2.next = Some(node1);
//}

//错误:
//15 |     node1.next = Some(node2);
//|                       ----- value moved here
//16 |     node2.next = Some(node1);
//|     ^^^^^^^^^^ value partially assigned here after move
//

//-------------------------循环引用2

//fn main() {
//    let mut node1 = Box::new(Node { next: None });
//    let mut node2 = Box::new(Node { next: None });
//
//    node1.next = Some(node2);
//    match node1.next { //match的过程也会转移所有权
//        Some(mut n) => n.next = Some(node1),
//
//        None => {}
//    }
//}

//错误:
//|
//31 |         Some(mut n) => n.next = Some(node1),
//|              ----- value moved here  ^^^^^ value used here after partial move

//-------------------------循环引用3
//use std::rc::Rc;
//
//struct Node {
//    next: Option<Rc<Node>>,
//}
//
//impl Drop for Node {
//    fn drop(&mut self) {
//        println!("drop");
//    }
//}

//fn main() {
//    let mut node1 = Node { next: None };
//    let mut node2 = Node { next: None };
//    let mut node3 = Node { next: None };
//
//    node1.next = Some(Rc::new(node2)); //node2所有权在这里转移了
//    node2.next = Some(Rc::new(node3)); //node2又被使用了
//    node3.next = Some(Rc::new(node1));
//}

/*
...
63 |     node1.next = Some(Rc::new(node2));
   |                               ----- value moved here
64 |     node2.next = Some(Rc::new(node3));
   |     ^^^^^^^^^^ value assigned here after move
*/

//-------------------------循环引用4

//fn main() {
//    let mut node1 = Rc::new(Node { next: None });
//    let mut node2 = Rc::new(Node { next: None });
//    let mut node3 = Rc::new(Node { next: None });
//
//    node1.next = Some(node2); //Rc 引用是不可修改的
//    node2.next = Some(node3); //node2所有权在上一行已经被移走了
//    node3.next = Some(node1);
//}

/*
error[E0594]: cannot assign to data in a `&` reference
  --> src/rust_syntax/memory_leak.rs:83:5
   |
83 |     node1.next = Some(node2);
   |     ^^^^^^^^^^ cannot assign
....
...
83 |     node1.next = Some(node2);
   |                       ----- value moved here
84 |     node2.next = Some(node3);
   |     ^^^^^ value borrowed here after move
*/

//-------------------------循环引用5

use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new() -> Node {
        Node { next: None }
    }
}
impl Drop for Node {
    fn drop(&mut self) {
        println!("drop");
    }
}

fn alloc_objects() {
    let node1 = Rc::new(RefCell::new(Node::new()));
    let node2 = Rc::new(RefCell::new(Node::new()));
    let node3 = Rc::new(RefCell::new(Node::new()));

    //    Rc::downgrade(node1);
    node1.borrow_mut().next = Some(node2.clone());
    node2.borrow_mut().next = Some(node3.clone());
    node3.borrow_mut().next = Some(node1.clone()); //成功制造了内存泄漏,但是没有导致内存安全(访问了不该访问的地址,多次释放等问题)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_alloc_object() {
        alloc_objects();
        //        assert_eq!(3, 5);
    }
}
