//// 如何使用生成器
//
//#![feature(genrators, generator_trait)]
//
//use std::ops::{Generator, GeneratorState};
//
//fn main() {
//    let mut g = || {
//        let mut curr: u64 = 1;
//        let mut next: u64 = 1;
//        loop {
//            let new_next = curr.checked_add(next);
//            if let Some(new_next) = new_next {
//                curr = next;
//                next = new_next;
//                yield curr;
//            } else {
//                return;
//            }
//        }
//    };
//    loop {
//        match g.resume() {
//            GeneratorState::Yielded(v) => println!("{}", v),
//            GeneratorState::Complete(_) => return,
//        }
//    }
//}
//
//mod test {
//    use super::*;
//    #[test]
//    fn test_generator() {
//        main();
//    }
//}
