use std::mem;
use std::ptr;
fn swap<T>(x: &mut T, y: &mut T) {
    unsafe {
        let mut t: T = mem::uninitialized();

        ptr::copy_nonoverlapping(&*x, &mut t, 1);
        ptr::copy_nonoverlapping(&*y, x, 1);
        ptr::copy_nonoverlapping(&t, y, 1);

        mem::forget(t); //因为t为正常初始化,必须阻止其析构
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_swap() {
        let mut x = 3;
        let mut y = 4;
        swap(&mut x, &mut y);
        assert_eq!(x, 4);
        assert_eq!(y, 3);
    }
}
