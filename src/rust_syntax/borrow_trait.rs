use std::borrow::Borrow;
use std::fmt::Display;
/*
Which should I use?
We can see how they’re kind of the same: they both deal with owned and borrowed versions of some type. However, they’re a bit different.

Choose Borrow when you want to abstract over different kinds of borrowing, or when you’re building a data structure that treats owned and borrowed values in equivalent ways, such as hashing and comparison.

Choose AsRef when you want to convert something to a reference directly, and you’re writing generic code.
*/

fn foo<T: Borrow<i32> + Display>(a: T) {
    println!("a is borrowed: {}", a);
}
#[test]
fn test_foo() {
    let mut i = 5;

    foo(&i);
    foo(&mut i);
}

fn foo2<T: AsRef<str>>(s: T) {
    let _slice = s.as_ref();
}
#[test]
fn test_foo2() {
    let s = "Hello".to_string();
    foo2(&s);
    use std::collections::HashMap;
    let mut m = HashMap::new();
    m.insert("a".to_string(), 32);
    let _t = m.get("a");
}
