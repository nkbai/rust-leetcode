use std::cell::RefCell;
fn main() {
    let shared_vec = RefCell::new(vec![1, 2, 3]);
    let shared1 = &shared_vec;
    let shared2 = &shared1;

    let p1 = shared1.borrow();
    let p2 = &p1[0];
    println!("{}", p2);
    drop(p1);

    shared2.borrow_mut().push(4);
}

struct CellV2<T> {
    value: T,
}

impl<T> CellV2<T> {
    fn new(v: T) -> Self
    where
        T: Copy,
    {
        CellV2 { value: v }
    }

    fn set(&self, v: T)
    where
        T: Copy,
    {
        unsafe {
            let p = &(self.value) as *const T as *mut T; //此处实际上引⼊了未定义⾏为
            *p = v;
        }
    }

    fn get(&self) -> T
    where
        T: Copy,
    {
        self.value
    }
}

struct Table<'arg> {
    cell: CellV2<&'arg isize>,
}

fn evil<'long, 'short>(t: &Table<'long>, s: &'short isize)
where
    'long: 'short,
{
    // The following assignment is not legal, but it escapes from lifetime checking
    let u: &Table<'short> = t; //内容错误是因为对这里的生命周期无法验证,普通只读共享情况下,'long 复制给'short是没有任何问题的,但是因为有内部可变性,导致了问题
    u.cell.set(s);
}
#[allow(dead_code)]
fn evil2<'long, 'short>(_t: &mut Table<'long>, _s: &'short isize)
where
    'long: 'short,
{
    // The following assignment is not legal, but it escapes from lifetime checking
    //    let u: &mut Table<'short> = t; //如果是mut借用,就存在lifetime mismatch的问题了
    //    u.cell.set(s);
}

fn innocent<'long>(t: &Table<'long>) {
    let foo: isize = 1;
    evil(t, &foo);
}

fn main_testcellv2() {
    let local = 100;
    let table = Table {
        cell: CellV2::new(&local),
    };
    innocent(&table); // reads `foo`, which has been destroyed
    let p = table.cell.get();
    println!("{}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        main();
    }
    #[test]
    fn testv2() {
        main_testcellv2();
    }
}
