use std::ops::Drop;
struct T {
    dropped: bool,
}
impl T {
    fn new() -> Self {
        T { dropped: false }
    }
}
impl Drop for T {
    fn drop(&mut self) {
        self.dropped = true;
    }
}

struct R<'a> {
    inner: Option<&'a T>,
}
impl<'a> R<'a> {
    fn new() -> Self {
        R { inner: None }
    }
    //'b生命周期大于等于'a
    fn set_ref<'b: 'a>(&mut self, ptr: &'a T) {
        self.inner = Some(ptr);
    }
}
impl<'a> Drop for R<'a> {
    fn drop(&mut self) {
        if let Some(ref inner) = self.inner {
            println!("droppen R when T is {}", inner.dropped);
        }
    }
}
////#![feature(generic_param_attrs, dropck_eyepatch)]
//unsafe impl<#[may_dangle] 'a> Drop for R<'a> {
//    fn drop(&mut self) {
//        if let Some(ref inner) = self.inner {
//            println!("droppen R when T is {}", inner.dropped);
//        }
//    }
//}

fn equallifetime() {
    {
        let (a, mut b): (T, R) = (T::new(), R::new());
        b.set_ref(&a);
    }
    {
        let (mut _a, _b) = (R::new(), T::new());
        //        a.set_ref(&b); //因为'a,'b的生命周期不匹配导致的,两者严格相等,在drop里是不允许的
    }
}

mod test {
    #[test]
    fn test_equal() {
        super::equallifetime();
    }
}
