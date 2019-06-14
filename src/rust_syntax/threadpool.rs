use std::sync::mpsc::channel;
use threadpool::ThreadPool;
/*
threadpool是⼀个基本的线程池实现
*/
fn main() {
    let n_workers = 4;
    let n_jobs = 8;
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = channel();
    for _ in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(1)
                .expect("channel will be there waiting for the pool");
        });
    }

    assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), 8);
}

mod test {
    use super::*;
    #[test]
    fn test_threadpool() {
        main();
    }
}
