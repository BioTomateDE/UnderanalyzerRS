use crate::primitives::RustStr;
use libgm::prelude::*;
use libgm::wad::elem::string::Strings;
use libgm::wad::elem::variable::Variable;

/// A variable, function or string.
#[repr(C)]
pub struct UaObject<'a> {
    pub name: RustStr<'a>,
    // Only used for variables:
    pub variable_id: i32,
    pub instance_type: i16,
}

impl<'a> UaObject<'a> {
    pub const NULL: Self = Self {
        name: RustStr::EMPTY,
        variable_id: 0,
        instance_type: 0,
    };

    pub fn try_from_libgm(variable: &Variable, strings: &'a Strings) -> Result<Self> {
        let name = RustStr::try_from_libgm(variable.name, strings)?;
        let mut this = Self { name, ..Self::NULL };

        if let Some(ref modern) = variable.modern_data {
            this.variable_id = modern.variable_id;
            this.instance_type = modern.variable_type.encode() as i16;
        }
        Ok(this)
    }
}
