//extern crate scoped_threadpool;
use scoped_threadpool::Pool;
/*
如果要在多线程之间共享变量，必 须使⽤Arc这样的保证线程安全的智能指针。
然⽽，Arc是有运⾏期开销的 （虽然很⼩）。假如我们有时候需要⼦线程访问当前调⽤栈中的局部变量，
⽽且能保证当前函数的⽣命周期⼀定⼤于⼦线程的⽣命周期，⼦线程 ⼀定先于当前函数退出，
那我们能不能直接在⼦线程中使⽤最简单的借⽤ 指针&来访问⽗线程栈上的局部对象呢？
*/
fn main() {
    let mut pool = Pool::new(4);

    let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7];
    //虽然vec是局部变量,但是我确保了闭包存在的生命周期不会超过vec的生命周期
    //因此可以放到另一个thread中去执行
    pool.scoped(|scope| {
        for e in &mut vec {
            scope.execute(move || {
                *e += 1;
            });
        }
    });
    //fn scoped<'pool, 'scope, F, R>(&'pool mut self, f: F) -> R where F: FnOnce(&Scope<'pool, 'scope>) -> R
    println!("{:?}", vec);
}

mod test {
    use super::*;
    #[test]
    fn test_scoped() {
        main();
    }
}
