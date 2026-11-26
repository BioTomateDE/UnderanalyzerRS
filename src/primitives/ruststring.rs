use libgm::prelude::*;
use libgm::wad::elem::string::Strings;
use std::marker::PhantomData;

#[repr(C)]
pub struct RustStr<'a> {
    ptr: *const u8,
    len: u32,
    _marker: PhantomData<&'a str>,
}

impl<'a> RustStr<'a> {
    pub const EMPTY: Self = Self::from_str("");

    #[must_use]
    pub const fn from_str(string: &'a str) -> Self {
        assert!(string.len() <= u32::MAX as usize, "String length exceeds 4 GiB");
        let ptr = string.as_ptr();
        let len = string.len() as u32;
        Self {
            ptr,
            len,
            _marker: PhantomData,
        }
    }

    pub fn try_from_libgm(string_ref: GMRef<String>, strings: &'a Strings) -> Result<Self> {
        let string: &String = strings.by_ref(string_ref)?;
        Ok(Self::from_str(string))
    }
}

unsafe impl Send for RustStr<'_> {}
unsafe impl Sync for RustStr<'_> {}
