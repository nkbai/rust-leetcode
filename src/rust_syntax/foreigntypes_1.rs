//use foreign_types::{ForeignType, ForeignTypeRef, Opaque};
//use std::ops::{Deref, DerefMut};
//use std::ptr::NonNull;
//
//mod foo_sys {
//    pub enum FOO {}
//
//    extern "C" {
//        pub fn FOO_free(foo: *mut FOO);
//    }
//}
//
//// The borrowed type is a newtype wrapper around an `Opaque` value.
////
//// `FooRef` values never exist; we instead create references to `FooRef`s
//// from raw C pointers.
//pub struct FooRef(Opaque);
//
//impl ForeignTypeRef for FooRef {
//    type CType = foo_sys::FOO;
//}
//
//// The owned type is simply a newtype wrapper around the raw C type.
////
//// It dereferences to `FooRef`, so methods that do not require ownership
//// should be defined there.
//pub struct Foo(NonNull<foo_sys::FOO>);
//
//unsafe impl Sync for FooRef {}
//unsafe impl Send for FooRef {}
//
//unsafe impl Sync for Foo {}
//unsafe impl Send for Foo {}
//
//impl Drop for Foo {
//    fn drop(&mut self) {
//        unsafe { foo_sys::FOO_free(self.as_ptr()) }
//    }
//}
//
//impl ForeignType for Foo {
//    type CType = foo_sys::FOO;
//    type Ref = FooRef;
//
//    unsafe fn from_ptr(ptr: *mut foo_sys::FOO) -> Foo {
//        Foo(NonNull::new_unchecked(ptr))
//    }
//
//    fn as_ptr(&self) -> *mut foo_sys::FOO {
//        self.0.as_ptr()
//    }
//}
//
//impl Deref for Foo {
//    type Target = FooRef;
//
//    fn deref(&self) -> &FooRef {
//        unsafe { FooRef::from_ptr(self.as_ptr()) }
//    }
//}
//
//impl DerefMut for Foo {
//    fn deref_mut(&mut self) -> &mut FooRef {
//        unsafe { FooRef::from_ptr_mut(self.as_ptr()) }
//    }
//}
//
//// add in Borrow, BorrowMut, AsRef, AsRefMut, Clone, ToOwned...
//
//foreign_type! {
//    /// A Foo.
//    pub type Foo
//        : Sync + Send // optional
//    {
//        type CType = foo_sys::FOO;
//        fn drop = foo_sys::FOO_free;
//        fn clone = foo_sys::FOO_duplicate; // optional
//    }
//
//    /// A Foo with generic parameters.
//    pub type GenericFoo<T> {
//        type CType = foo_sys::FOO;
//        // This type is added as a `PhantomData` field to handle variance
//        // of the parameters. However, it has no impact on trait impls:
//        // `GenericFoo<T>` is always `Clone`, even if `T` is not.
//        type PhantomData = T;
//        fn drop = foo_sys::FOO_free;
//        fn clone = foo_sys::FOO_duplicate;
//    }
//}
