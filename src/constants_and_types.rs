pub const STACK_DATA_SIZE:usize = 32;

pub type RegisterDataType = u32;
pub type iRegisterDataType = i32;

pub type InstructionParamType = u32;
pub type iInstructionParamType = i32;
pub type FloatInstructionParamType = f32;

pub type InstructionNameBinaryType = u32;
pub const INSTRUCTION_NAME_SIZE:usize = 32; // bits


pub const ZERO_FLAG:usize = 0;
pub const EQUAL_FLAG:usize = 1;
pub const LESS_THAN_FLAG:usize = 2;
pub const GREATER_THAN_FLAG:usize = 3;

pub const REGA:InstructionParamType = 0;
pub const REGB:InstructionParamType = 1;
pub const REGC:InstructionParamType = 2;
pub const REGD:InstructionParamType = 3;
pub const RESERVEREGISTER:InstructionParamType = 4;



// These are floating point register 
// They store floating point only data
pub type FloatRegisterDataType = f32;

pub const INT_PARAM_SIZE:usize = 32; // bits 
pub const FLOAT_PARAM_SIZE:usize = 32;
pub const REGISTER_PARAM_SIZE:usize = 32;
pub const JUMP_DESTINATION_PARAM_SIZE:usize = 32;
