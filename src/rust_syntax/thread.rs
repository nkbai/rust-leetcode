use std::thread;
fn main() {
    let mut v: Vec<i32> = vec![];

    thread::spawn(move || {
        //要求闭包自动生成的JoinHandle<T>中的T是支持Send的,因此从这个角度来说,
        //rust语言完全不知道线程,只是标准库的设计者知道.
        v.push(1);
    });

    //    println!("{:?}", v);
}

mod test {
    use super::*;
    #[test]
    fn test_thread() {
        main();
    }
}
