use libgm::{gml::GMCode, prelude::*};

use crate::{
    gamemaker::Instruction,
    primitives::{RawArray, RustString},
};

#[repr(C)]
pub struct Code<'a> {
    name: RustString<'a>,
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

        let children: Vec<GMRef<GMCode>> = find_children(code_ref, data)?;
        let children: Vec<Self> = children
            .into_iter()
            .map(|c| Self::try_from_libgm(c, data))
            .collect::<Result<_>>()?;

        Ok(Self {
            name: RustString::from_str(&code.name),
            instructions: RawArray::from_vec(get_instructions(code, data)?),
            children: RawArray::from_vec(children),
            length: get_length(code),
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

fn find_children(code_ref: GMRef<GMCode>, data: &GMData) -> Result<Vec<GMRef<GMCode>>> {
    // No child code entries before WAD 15
    if data.general_info.wad_version < 15 {
        return Ok(Vec::new());
    }

    let mut children: Vec<GMRef<GMCode>> = Vec::new();

    for (idx, code_entry) in data.codes.iter().enumerate() {
        let Some(parent) = code_entry.parent() else {
            continue;
        };
        if code_ref == parent {
            children.push(GMRef::from(idx));
        }
    }

    Ok(children)
}

fn get_length(code: &GMCode) -> u32 {
    code.instructions
        .iter()
        .map(|instr| u32::from(instr.size()))
        .sum()
}
