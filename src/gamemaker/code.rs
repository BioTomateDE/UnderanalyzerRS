use crate::gamemaker::UaInstruction;
use crate::primitives::RawArray;
use crate::primitives::RustStr;
use libgm::gml::Code;
use libgm::prelude::*;

#[repr(C)]
pub struct UaCode<'a> {
    name: RustStr<'a>,
    instructions: RawArray<UaInstruction<'a>>,
    children: RawArray<Self>,
    length: u32,
    start_offset: u32,
    argument_count: u16,
    local_count: u16,
}

impl<'a> UaCode<'a> {
    pub fn try_from_libgm(code_ref: GMRef<Code>, data: &'a GMData) -> Result<Self> {
        let code: &Code = data.codes.by_ref(code_ref)?;
        let name: &String = data.strings.by_ref(code.name)?;

        let modern = code.modern_data.clone().unwrap_or_default();
        let start_offset: u32 = modern.execution_offset;
        let argument_count = modern.arg_count;
        let local_count = modern.local_count;

        Ok(Self {
            name: RustStr::from_str(name),
            instructions: RawArray::from_boxed_slice(get_instructions(code, data)?),
            children: RawArray::from_boxed_slice(get_children(code_ref, data)?),
            length: code.length(),
            start_offset,
            argument_count,
            local_count,
        })
    }
}

fn get_instructions<'a>(code: &'a Code, data: &'a GMData) -> Result<Box<[UaInstruction<'a>]>> {
    code.instructions
        .iter()
        .map(|i| UaInstruction::try_from_libgm(i, data))
        .collect()
}

fn get_children(code_ref: GMRef<Code>, data: &GMData) -> Result<Box<[UaCode<'_>]>> {
    Code::find_children(code_ref, data)
        .into_iter()
        .map(|child_ref| UaCode::try_from_libgm(child_ref, data))
        .collect()
}
