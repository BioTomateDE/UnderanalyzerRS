use std::marker::PhantomData;

#[repr(C)]
pub struct RustString<'a> {
    ptr: *const u8,
    len: usize,
    _marker: PhantomData<&'a str>,
}

impl<'a> RustString<'a> {
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
