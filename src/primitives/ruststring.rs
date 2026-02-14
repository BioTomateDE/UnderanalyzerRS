use std::marker::PhantomData;

#[repr(C)]
pub struct RustStr<'a> {
    ptr: *const u8,
    len: usize,
    _marker: PhantomData<&'a str>,
}

impl<'a> RustStr<'a> {
    pub const EMPTY: Self = Self::from_str("");

    #[must_use]
    pub const fn from_str(string: &'a str) -> Self {
        let ptr = string.as_ptr();
        let len = string.len();
        Self {
            ptr,
            len,
            _marker: PhantomData,
        }
    }
}

unsafe impl Send for RustStr<'_> {}
unsafe impl Sync for RustStr<'_> {}
