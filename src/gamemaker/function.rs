use libgm::gamemaker::elements::function::GMFunction;

use crate::primitives::RustStr;

#[repr(C)]
pub struct Function<'a> {
    name: RustStr<'a>,
}

impl<'a> Function<'a> {
    pub const NULL: Self = Self::new(RustStr::EMPTY);

    #[must_use]
    const fn new(name: RustStr<'a>) -> Self {
        Self { name }
    }

    #[must_use]
    pub fn from_libgm(function: &'a GMFunction) -> Self {
        Self::new(RustStr::from_str(&function.name))
    }
}
