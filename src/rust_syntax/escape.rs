#[derive(Debug)]
struct A {
    a: u8,
    b: u16,
    c: u64,
}
impl A {
    pub fn new() -> A {
        A { a: 3, b: 2, c: 7 }
    }
}

#[test]
fn main() {
    let a = A::new();
    println!("a={:?}", a);
}
