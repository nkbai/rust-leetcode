#[macro_use]
use foreign_types::ForeignTypeRef;

mod foo_sys {
    pub enum FOO {}
    pub enum BAR {}

    extern "C" {
        pub fn FOO_free(foo: *mut FOO);
        pub fn BAR_free(bar: *mut BAR);
        pub fn BAR_get_foo(bar: *mut BAR) -> *mut FOO;
    }
}

foreign_type! {
    /// A Foo.
    pub type Foo: Sync + Send {
        type CType = foo_sys::FOO;
        fn drop = foo_sys::FOO_free;
    }

    /// A Bar.
    pub type Bar: Sync + Send {
        type CType = foo_sys::BAR;
        fn drop = foo_sys::BAR_free;
    }
}

impl BarRef {
    fn foo(&self) -> &FooRef {
        unsafe { FooRef::from_ptr(foo_sys::BAR_get_foo(self.as_ptr())) }
    }

    fn foo_mut(&mut self) -> &mut FooRef {
        unsafe { FooRef::from_ptr_mut(foo_sys::BAR_get_foo(self.as_ptr())) }
    }
}
