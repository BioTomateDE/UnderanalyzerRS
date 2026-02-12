use crate::{
    gamemaker::{function::Function, variable::Variable},
    primitives::RustString,
};

use libgm::{
    gml::instruction::{Instruction as LibGMInstruction, PushValue},
    prelude::*,
};

#[repr(C)]
pub struct Instruction<'a> {
    variable: Variable<'a>,
    function: Function<'a>,
    value_string: RustString<'a>,
    value_double: f64,
    value_long: i64,
    value_int: i32,
    branch_offset: i32,
    argument_count: i32,
    asset_reference: i32,
    value_short: i16,
    extended_kind: i16,
    instance_type: i16,
    opcode: u8,
    type1: u8,
    type2: u8,
    comparison_kind: u8,
    duplication_size: u8,
    duplication_size2: u8,
    variable_type: u8,
    pop_swap_size: u8,
    pop_with_context_exit: u8,
}

impl<'a> Instruction<'a> {
    pub fn try_from_libgm(instr: &'a LibGMInstruction, data: &'a GMData) -> Result<Self> {
        Ok(Self {
            variable: extract_variable(instr, data)?,
            function: extract_function(instr, data)?,
            value_string: extract_string(instr),
            value_double: extract_double(instr),
            value_long: extract_long(instr),
            value_int: extract_int(instr),
            branch_offset: 4 * instr.jump_offset().unwrap_or(0),
            argument_count: extract_argument_count(instr),
            asset_reference: extract_asset_reference(instr),
            value_short: extract_short(instr),
            extended_kind: instr.extended_kind().unwrap_or(0),
            instance_type: instr.variable().map_or(0, |v| v.instance_type.build()),
            opcode: instr.opcode(),
            type1: instr.type1().map_or(0, u8::from),
            type2: instr.type2().map_or(0, u8::from),
            comparison_kind: extract_comparison_kind(instr),
            duplication_size: extract_dupsize(instr),
            duplication_size2: extract_dupsize2(instr),
            variable_type: instr.variable().map_or(0, |v| u8::from(v.variable_type)),
            pop_swap_size: extract_popswap_size(instr),
            pop_with_context_exit: u8::from(*instr == LibGMInstruction::PopWithContextExit),
        })
    }
}

fn extract_variable<'a>(instr: &LibGMInstruction, data: &'a GMData) -> Result<Variable<'a>> {
    if let Some(code_var) = instr.variable() {
        let variable = data.variables.by_ref(code_var.variable)?;
        Ok(Variable::from_libgm(variable))
    } else {
        Ok(Variable::NULL)
    }
}

fn extract_function<'a>(instr: &LibGMInstruction, data: &'a GMData) -> Result<Function<'a>> {
    if let Some(func_ref) = instr.function() {
        let function = data.functions.by_ref(func_ref)?;
        Ok(Function::from_libgm(function))
    } else {
        Ok(Function::NULL)
    }
}

fn extract_string(instr: &LibGMInstruction) -> RustString<'_> {
    if let LibGMInstruction::Push {
        value: PushValue::String(string),
    } = instr
    {
        RustString::from_str(string)
    } else {
        RustString::EMPTY
    }
}

const fn extract_double(instr: &LibGMInstruction) -> f64 {
    if let LibGMInstruction::Push {
        value: PushValue::Double(double),
    } = instr
    {
        *double
    } else {
        0.0
    }
}

const fn extract_long(instr: &LibGMInstruction) -> i64 {
    if let LibGMInstruction::Push {
        value: PushValue::Int64(long),
    } = instr
    {
        *long
    } else {
        0
    }
}

const fn extract_int(instr: &LibGMInstruction) -> i32 {
    if let LibGMInstruction::Push {
        value: PushValue::Int32(int),
    } = instr
    {
        *int
    } else {
        0
    }
}

const fn extract_short(instr: &LibGMInstruction) -> i16 {
    match instr {
        LibGMInstruction::Push {
            value: PushValue::Int16(integer),
        }
        | LibGMInstruction::PushImmediate { integer } => *integer,
        _ => 0,
    }
}

fn extract_argument_count(instr: &LibGMInstruction) -> i32 {
    match *instr {
        LibGMInstruction::Call { argument_count, .. }
        | LibGMInstruction::CallVariable { argument_count } => i32::from(argument_count),
        _ => 0,
    }
}

const fn extract_asset_reference(instr: &LibGMInstruction) -> i32 {
    match instr {
        LibGMInstruction::PushReference { asset_reference } => asset_reference.build() as i32,
        _ => 0,
    }
}

fn extract_comparison_kind(instr: &LibGMInstruction) -> u8 {
    match instr {
        LibGMInstruction::Compare {
            comparison_type, ..
        } => u8::from(*comparison_type),
        _ => 0,
    }
}

const fn extract_dupsize(instr: &LibGMInstruction) -> u8 {
    match *instr {
        LibGMInstruction::Duplicate { size, .. }
        | LibGMInstruction::DuplicateSwap { size1: size, .. } => size,
        _ => 0,
    }
}

const fn extract_dupsize2(instr: &LibGMInstruction) -> u8 {
    match *instr {
        LibGMInstruction::DuplicateSwap { size2, .. } => size2,
        _ => 0,
    }
}

const fn extract_popswap_size(instr: &LibGMInstruction) -> u8 {
    match *instr {
        LibGMInstruction::PopSwap { is_array: false } => 5,
        LibGMInstruction::PopSwap { is_array: true } => 6,
        _ => 0,
    }
}
