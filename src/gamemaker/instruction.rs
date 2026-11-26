use crate::gamemaker::object::UaObject;
use crate::primitives::RustStr;
use libgm::gml::instruction::CodeVariable;
use libgm::gml::instruction::DataType;
use libgm::gml::instruction::Instruction;
use libgm::gml::instruction::PushValue;
use libgm::prelude::*;

#[repr(u8)]
enum ObjectKind {
    None = 0,
    String = 1,
    Variable = 2,
    Function = 3,
}

#[repr(C)]
pub struct UaInstruction<'a> {
    /// Push value (int/long/double)
    primitive: u64,
    /// Reference to variable OR function OR string
    object: UaObject<'a>,
    /// Some extra value(s)
    immediate: u32,
    /// Data types
    types: u8,
    opcode: u8,
    object_kind: ObjectKind,
}

impl<'a> UaInstruction<'a> {
    #[allow(clippy::too_many_lines)]
    pub fn try_from_libgm(instr: &'a Instruction, data: &'a GMData) -> Result<Self> {
        let opcode: u8 = instr.opcode();
        let mut immediate: u32 = 0;
        let mut primitive = 0;
        let mut object = UaObject::NULL;
        let mut object_kind = ObjectKind::None;
        let mut types: u8 = 0xFF;

        match *instr {
            Instruction::Binary { op: _, lhs, rhs } => {
                types = encode_double_type(rhs, lhs);
            }
            Instruction::Convert { from, to } => {
                types = encode_double_type(from, to);
            }
            Instruction::Negate { data_type }
            | Instruction::Not { data_type }
            | Instruction::PopDiscard { data_type } => {
                types = data_type.encode();
            }
            Instruction::Compare {
                lhs,
                rhs,
                comparison_type,
            } => {
                types = encode_double_type(rhs, lhs);
                immediate = comparison_type.as_u8() as u32;
            }
            Instruction::Pop {
                variable: code_variable,
                type1,
                type2,
            } => {
                types = encode_double_type(type1, type2);
                immediate = encode_variable_ref(code_variable);
                let var = data.variables.by_ref(code_variable.variable)?;
                object = UaObject::try_from_libgm(var, &data.strings)?;
                object_kind = ObjectKind::Variable;
            }
            Instruction::PopSwap { is_array } => {
                types = encode_double_type(DataType::Short, DataType::Variable);
                immediate = if is_array { 6 } else { 5 };
            }
            Instruction::Duplicate { data_type, count } => {
                immediate = count as u32;
                types = data_type.encode();
            }
            Instruction::DuplicateSwap {
                data_type,
                size1,
                size2,
            } => {
                immediate = size1 as u32 | (size2 as u32) << 8;
                types = data_type.encode();
            }
            Instruction::Exit => {
                types = DataType::Int.encode();
            }
            Instruction::Return => {
                types = DataType::Variable.encode();
            }
            Instruction::Branch { jump_offset }
            | Instruction::BranchIf { jump_offset }
            | Instruction::BranchUnless { jump_offset }
            | Instruction::PushWithContext { jump_offset }
            | Instruction::PopWithContext { jump_offset } => {
                immediate = jump_offset as u32;
            }
            Instruction::PopWithContextExit => {
                immediate = 0xF << 20;
            }
            Instruction::Push { value } => {
                types = value.data_type().encode();
                match value {
                    PushValue::Short(short) => immediate = short as u32,
                    PushValue::Int(int) => primitive = int as u64,
                    PushValue::Long(long) => primitive = long as u64,
                    PushValue::Double(double) => primitive = double.to_bits(),
                    PushValue::Bool(bool) => primitive = bool as u64,
                    PushValue::String(string_ref) => {
                        object.name = RustStr::try_from_libgm(string_ref, &data.strings)?;
                        object_kind = ObjectKind::String;
                    }
                    PushValue::Variable(code_variable) => {
                        immediate = encode_variable_ref(code_variable);
                        let var = data.variables.by_ref(code_variable.variable)?;
                        object = UaObject::try_from_libgm(var, &data.strings)?;
                        object_kind = ObjectKind::Variable;
                    }
                    PushValue::Function(func_ref) => {
                        let func = data.functions.by_ref(func_ref)?;
                        object.name = RustStr::try_from_libgm(func.name, &data.strings)?;
                        object_kind = ObjectKind::Function;
                    }
                }
            }
            Instruction::PushLocal { variable: v }
            | Instruction::PushGlobal { variable: v }
            | Instruction::PushBuiltin { variable: v } => {
                immediate = encode_variable_ref(v);
                let var = data.variables.by_ref(v.variable)?;
                object = UaObject::try_from_libgm(var, &data.strings)?;
                object_kind = ObjectKind::Variable;
            }
            Instruction::PushImmediate { integer } => immediate = integer as u32,
            Instruction::Call { function, arg_count } => {
                types = DataType::Int.encode();
                immediate = arg_count as u32;
                let func = data.functions.by_ref(function)?;
                object.name = RustStr::try_from_libgm(func.name, &data.strings)?;
                object_kind = ObjectKind::Function;
            }
            Instruction::CallVariable { arg_count } => {
                types = DataType::Variable.encode();
                immediate = arg_count as u32;
            }
            Instruction::CheckArrayIndex
            | Instruction::PushArrayFinal
            | Instruction::PopArrayFinal
            | Instruction::PushArrayContainer
            | Instruction::SetArrayOwner
            | Instruction::HasStaticInitialized
            | Instruction::SetStaticInitialized
            | Instruction::SaveArrayReference
            | Instruction::RestoreArrayReference
            | Instruction::IsNullishValue => {
                immediate = instr.extended_kind().unwrap() as u32;
            }
            Instruction::PushReference { asset_reference } => {
                primitive = asset_reference.encode_old() as u64;
            }
        }

        Ok(Self {
            primitive,
            object,
            immediate,
            types,
            opcode,
            object_kind,
        })
    }
}

const fn encode_double_type(type1: DataType, type2: DataType) -> u8 {
    type1.encode() | type2.encode() << 4
}

const fn encode_variable_ref(code_variable: CodeVariable) -> u32 {
    let inst = code_variable.instance_type.encode(code_variable.ref_type);
    let reftype = code_variable.ref_type.encode();
    inst as u32 | (reftype as u32) << 16
}
