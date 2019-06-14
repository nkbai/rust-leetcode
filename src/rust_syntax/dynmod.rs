//DST类型
//#![feature(dyn_trait)]

/// 什么样的trait不能用trait object:
///
/// ``` rust
/// let p: &dyn Clone = if from_input() { &obj1 as &dyn Clone } else { &obj2 as &dyn Clone };
/// let o = p.clone();
/// ```
/// 这种情况下,rust无法处理返回值
use std::mem;

trait Bird {
    fn fly(&self);
}

struct Duck;
struct Swan;

impl Bird for Duck {
    fn fly(&self) {
        println!("duck duck")
    }
}

impl Bird for Swan {
    fn fly(&self) {
        println!("swan swan")
    }
}

//DST类型作为参数

fn print_traitobject(p: &dyn Bird) {
    // 使⽤transmute执⾏强制类型转换,把变量p的内部数据取出来
    let (data, vtable): (usize, *const usize) = unsafe { mem::transmute(p) };
    println!("TraitObject [data:{}, vtable:{:p}]", data, vtable);
    unsafe {
        // 打印出指针 v 指向的内存区间的值

        println!(
            "data in vtable [{}, {}, {}, {}]",
            *vtable,
            *vtable.offset(1),
            *vtable.offset(2),
            *vtable.offset(3)
        );
    }
}

mod test {
    use super::*;
    #[test]
    fn test_traitobj() {
        let duck = Duck;
        let p_duck = &duck;
        let p_bird = p_duck as &dyn Bird;
        println!(
            "Size of p_duck {}, Size of p_bird {}",
            mem::size_of_val(&p_duck),
            mem::size_of_val(&p_bird)
        );

        let duck_fly: usize = Duck::fly as usize;
        let swan_fly: usize = Swan::fly as usize;
        println!("Duck::fly {}", duck_fly);
        println!("Swan::fly {}", swan_fly);

        print_traitobject(p_bird);
        let swan = Swan;
        print_traitobject(&swan as &dyn Bird);
    }
}
