/*
第20章　Vec源码分析

本节将通过⼀个⽐较完整的例⼦，把内存安全问题分析⼀遍。本节选 择的例⼦是标准库中的基本数据结构Vec<T>源代码分析。之所以选择这个 模块作为⽰例，其⼀是因为这个类型作为⾮常基础的数据结构，平时⽤得 很多，⼤家都很熟悉；第⼆个原因是，恰好它的内部实现又完全展现了 Rust内存安全的⽅⽅⾯⾯，深⼊剖析它的内部实现⾮常有利于加深我们对 Rust内存安全的认识。本章中⽤于分析的代码是1.23 nightly版本，Vec的内 部实现源码在此之前⼀直有所变化，以后也很可能还会有变化，请读者注 意这⼀点。
*/

use core::borrow::Borrow;

fn main() {
    let mut v1 = Vec::<i32>::new();
    println!("Start: length {} capacity {}", v1.len(), v1.capacity());
    for i in 1..10 {
        v1.push(i);
        println!(
            "[Pushed {}] length {} capacity {}",
            i,
            v1.len(),
            v1.capacity()
        );
    }

    let mut v2 = Vec::<i32>::with_capacity(1);
    println!("Start: length {} capacity {}", v2.len(), v2.capacity());

    v2.reserve(10);
    for i in 1..10 {
        v2.push(i);

        println!(
            "[Pushed {}] length {} capacity {}",
            i,
            v2.len(),
            v2.capacity()
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_vec() {
        main()
    }
}

fn calc_by<'a, F>(var: &'a i32, f: F) -> i32
where
    F: for<'f> Fn(&'f i32) -> i32,
{
    let local = *var;

    f(&local)
}
