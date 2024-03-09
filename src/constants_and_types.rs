pub const COMMAND_SIZE:usize = 32;
pub const DESTINATION_SIZE:usize = 8; // Single destination size
pub const COMMAND_NAME_SIZE:usize=  16;
pub const REGISTER_SIZE:usize = 16;

pub const STACK_DATA_SIZE:usize = 16;

pub type DestinationType = u16;
pub type RegisterDataType = u16;
pub type CommandType = u32;
pub type CommandNameType= u16;
pub type CommandBinary = [u8;COMMAND_SIZE];
pub type DestinationBinary = [u8;DESTINATION_SIZE];
pub type CombinedDestinationBinary = [u8;DESTINATION_SIZE*2];
pub type RegisterDataBinary = [u8;REGISTER_SIZE];


pub type InstructionParamType = u16;

pub const ZERO_FLAG:usize = 0;
pub const EQUAL_FLAG:usize = 1;
pub const LESS_THAN_FLAG:usize = 2;
pub const GREATER_THAN_FLAG:usize = 3;

pub const REGA:InstructionParamType = 0;
pub const REGB:InstructionParamType = 1;
pub const REGC:InstructionParamType = 2;
pub const REGD:InstructionParamType = 3;
pub const RESERVEREGISTER:InstructionParamType = 4;

