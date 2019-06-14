/*
crossbeam是Rust核⼼组的另外⼀位重要成员Aaron Turon牵头开发的。 它包含了并⾏开发的很多⽅⾯的功能，
⽐如⽆锁数据类型，以及重新设计 实现的管道。我们知道标准库给了⼀份mpsc（多⽣产者单消费者）管道的实现，
但是它有许多缺陷。crossbeam-channel这个库给我们提供了另外⼀套管道的 实现⽅式。不仅包括mpsc，
还包括mpmc（多⽣产者多消费者），⽽且使 ⽤便捷，执⾏效率也很⾼。
*/
//todo 编译不过去,找一个可以编译过去的例子
//extern crate crossbeam;
//#[macro_use]
//extern crate crossbeam_channel as channel;
//
//use channel::{Receiver, Sender};
//
//fn main() {
//    let people = vec!["Anna", "Bob", "Cody", "Dave", "Eva"];
//    let (tx, rx) = channel::bounded(1); // Make room for one unmatched send.
//    let (tx, rx) = (&tx, &rx);
//
//    crossbeam::scope(|s| {
//        for name in people {
//            s.spawn(move || seek(name, tx, rx));
//        }
//    });
//
//    if let Ok(name) = rx.try_recv() {
//        println!("No one received {}’s message.", name);
//    }
//}
//
//// Either send my name into the channel or receive someone else's, whatever happens first.
//
//fn seek<'a>(name: &'a str, tx: &Sender<&'a str>, rx: &Receiver<&'a str>) {
//    select_loop! {
//
//    recv(rx, peer) => println!("{} received a message from {}.", name, peer),
//
//    send(tx, name) => {}, // Wait for someone to receive my message.
//
//    }
//}
