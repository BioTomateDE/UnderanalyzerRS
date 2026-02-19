use libgm::{gml::GMCode, prelude::*};

use crate::{
    gamemaker::Instruction,
    primitives::{RawArray, RustStr},
};

#[repr(C)]
pub struct Code<'a> {
    name: RustStr<'a>,
    instructions: RawArray<Instruction<'a>>,
    children: RawArray<Self>,
    length: u32,
    start_offset: u32,
    argument_count: u16,
    local_count: u16,
}

impl<'a> Code<'a> {
    pub fn try_from_libgm(code_ref: GMRef<GMCode>, data: &'a GMData) -> Result<Self> {
        let code: &GMCode = data.codes.by_ref(code_ref)?;

        let modern = code.modern_data.clone().unwrap_or_default();
        let start_offset: u32 = modern.offset;
        let argument_count = modern.arguments_count;
        let local_count = modern.locals_count;

        Ok(Self {
            name: RustStr::from_str(&code.name),
            instructions: RawArray::from_vec(get_instructions(code, data)?),
            children: RawArray::from_vec(get_children(code_ref, data)?),
            length: code.length(),
            start_offset,
            argument_count,
            local_count,
        })
    }
}

fn get_instructions<'a>(code: &'a GMCode, data: &'a GMData) -> Result<Vec<Instruction<'a>>> {
    code.instructions
        .iter()
        .map(|i| Instruction::try_from_libgm(i, data))
        .collect()
}

fn get_children<'a>(code_ref: GMRef<GMCode>, data: &'a GMData) -> Result<Vec<Code<'a>>> {
    GMCode::find_children(code_ref, data)
        .into_iter()
        .map(|child_ref| Code::try_from_libgm(child_ref, data))
        .collect()
}
