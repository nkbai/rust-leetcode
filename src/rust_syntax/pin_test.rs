#![allow(unused_imports, dead_code)]
use std::marker::PhantomPinned;
use std::marker::Unpin;
use std::path::Path;
use std::pin::Pin;
use std::ptr::NonNull;

struct Unmovable {
    data: String,
    slice: NonNull<String>,
    _pin: PhantomPinned,
}
impl Unpin for Unmovable {}
impl Unmovable {
    fn new(data: String) -> Pin<Box<Self>> {
        let res = Unmovable {
            data,
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);
        let slice = NonNull::from(&boxed.data);
        unsafe {
            let mut_ref = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).slice = slice;
        }
        boxed
    }
}

#[test]
fn test_pin() {
    let unmoved = Unmovable::new("hello".into());
    let mut still_unmoved = unmoved;
    assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data));
    let new_unmoved = Unmovable::new("ww".into());
    unsafe {
        let still_unmoved_mut_ref = still_unmoved.as_mut();
        //        still_unmoved_mut_ref.data="he".to_string();
        Pin::get_unchecked_mut(still_unmoved_mut_ref).data = "new".to_string();
    }

    //    let mut a = *still_unmoved; 不能取出来在去swap，因为Pin::Deref是&self借用？
    //    let mut b = *new_unmoved;
    //如果没有Unpin无法编译通过，为什么呢？ 是因为as_mut 只有在实现了Unpin的T中才有？
    //    std::mem::swap(&mut *still_unmoved, &mut *new_unmoved);
    println!(
        "still_unmoved={},data={}",
        unsafe { still_unmoved.slice.as_ref() },
        still_unmoved.data
    );
    println!(
        "new_unmoved={},data={}",
        unsafe { new_unmoved.slice.as_ref() },
        new_unmoved.data
    );
}
