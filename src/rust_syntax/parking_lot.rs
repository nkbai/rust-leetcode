use parking_lot::Mutex;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
/*
Rust标准库帮我们封装了⼀些基本的操作系统的同步原语，⽐如Mutex Condvar等。⼀般情况下这些够我们使⽤了。
但是还有⼀些对性能有极致要 求的开发者对标准库的实现并不满意，于是社区⾥又有⼈开发出来了⼀套 替代品，
在性能和易⽤性⽅⾯，都⽐标准库更好，这就是parking_lot库。 下⾯的⽰例展⽰了这个库提供的Mutex，
它的⽤法与标准库的Mutex差别不 ⼤：
*/
fn main() {
    const N: usize = 10;

    let data = Arc::new(Mutex::new(0));

    let (tx, rx) = channel();
    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());
        thread::spawn(move || {
            let mut data = data.lock();
            *data += 1;
            if *data == N {
                tx.send(()).unwrap();
            }
        });
    }

    println!("{:?}", rx.recv().unwrap());
}

mod test {
    use super::*;
    #[test]
    fn test_parking_lot() {
        main();
    }
}
