use libgm::gamemaker::elements::function::GMFunction;

use crate::primitives::RustString;

#[repr(C)]
pub struct Function<'a> {
    name: RustString<'a>,
}

impl<'a> Function<'a> {
    pub const NULL: Self = Self::new(RustString::EMPTY);

    #[must_use]
    const fn new(name: RustString<'a>) -> Self {
        Self { name }
    }

    #[must_use]
    pub fn from_libgm(function: &'a GMFunction) -> Self {
        Self::new(RustString::from_str(&function.name))
    }
}
