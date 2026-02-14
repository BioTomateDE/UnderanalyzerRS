use libgm::gamemaker::elements::variable::GMVariable;

use crate::primitives::RustStr;

#[repr(C)]
pub struct Variable<'a> {
    name: RustStr<'a>,
    variable_id: i32,
    instance_type: i16,
}

impl<'a> Variable<'a> {
    pub const NULL: Self = Self::new(RustStr::EMPTY, 0, 0);

    #[must_use]
    const fn new(name: RustStr<'a>, variable_id: i32, instance_type: i16) -> Self {
        Self {
            name,
            variable_id,
            instance_type,
        }
    }

    #[must_use]
    pub fn from_libgm(variable: &'a GMVariable) -> Self {
        let name = RustStr::from_str(&variable.name);

        let modern = variable.modern_data.clone().unwrap_or_default();
        let variable_id: i32 = modern.variable_id;
        let instance_type: i16 = modern.instance_type.build();

        Self::new(name, variable_id, instance_type)
    }
}
