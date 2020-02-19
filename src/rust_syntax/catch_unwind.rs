#[allow(dead_code)]
use std::panic;
use std::panic::AssertUnwindSafe;
#[allow(dead_code)]
fn catch_unwind() {
    let mut x: Vec<i32> = vec![1];
    let mut y: Vec<i32> = vec![2];
    //   不加move 错误 `&mut std::vec::Vec<i32>` may not be safely transferred across an unwind boundary
    panic::catch_unwind(move || {
        x.push(10);

        y.push(100);
    })
    .ok();

    //    println!("Observe corruptted data. {:?} {:?}", x, y);
}

fn catch_unwind_unsafe() {
    let mut x: Vec<i32> = vec![1];
    let mut y: Vec<i32> = vec![2];

    panic::catch_unwind(AssertUnwindSafe(|| {
        x.push(10);
        if x.len() > 5 {
            panic!("user panic");
        }
        y.push(100);
    }))
    .ok();

    println!("Observe corruptted data. {:?} {:?}", x, y);
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_main() {
        catch_unwind_unsafe();
    }
}
