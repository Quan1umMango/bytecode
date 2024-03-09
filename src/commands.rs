use crate::{
    constants_and_types::*,
    conversions::*,
};
use crate::{to_binary_slice,binary_slice_to_number};
use crate::{create_command_no_arg,create_command_two_arg,create_command_single_arg_full_dest,create_command_single_arg};

#[derive(Debug)]
pub enum Command {
    Halt,
    Mov(DestinationType),
    Add(DestinationType),
    Sub(DestinationType),
    Display(DestinationType),
    Push(DestinationType),
    PushRegister(DestinationType),
    Pop(DestinationType),
    Jump(DestinationType),
    JumpIfZero(DestinationType),
    JumpIfNotZero(DestinationType),
    JumpIfEqual(DestinationType),
    JumpIfNotEqual(DestinationType),
    JumpIfGreater(DestinationType),
    JumpIfLess(DestinationType),
    Compare(DestinationType),

    GetFromStack(DestinationType),
    GetFromStackPointer(DestinationType),

    Malloc(DestinationType),
    GetMemory(DestinationType),
    SetMemory(DestinationType),

    Return,

    Mul(DestinationType),
    Div(DestinationType),
    Mod(DestinationType),
    
    Or(DestinationType),
    And(DestinationType),
    Not(DestinationType),
    Xor(DestinationType),
    Nand(DestinationType),
    TruncateStack(DestinationType)

}


pub fn add_destination_to_command_binary(command:&mut CommandBinary,dest:(Option<DestinationBinary>,Option<DestinationBinary>)) {
    let start1= COMMAND_NAME_SIZE;
    let end1 = COMMAND_NAME_SIZE+DESTINATION_SIZE;
    let start2 = end1;
    let end2 = start2 + DESTINATION_SIZE;
    command[start1..end1].copy_from_slice(&dest.0.unwrap());
    if let Some(dest2) = dest.1 {
        command[start2..end2].copy_from_slice(&dest2);
    }
}


pub fn add_full_destination_to_command_binary(command:&mut CommandBinary,dest:CombinedDestinationBinary) {
    let start1= COMMAND_NAME_SIZE;
    let end1 = COMMAND_NAME_SIZE+DESTINATION_SIZE*2; 
    command[start1..end1].copy_from_slice(&dest);
    
}


impl Command {
    pub fn to_binary_code(&self) -> CommandBinary {
        use Command::*;
        let mut code:CommandBinary = [0;COMMAND_SIZE];
        match self {
            Halt => return code,
            Mov(dest) => create_command_two_arg!(*dest,code,1),
            Add(dest) => create_command_two_arg!(*dest,code,2),
            Sub(dest) => create_command_two_arg!(*dest,code,3),
            Display(dest) => create_command_two_arg!(*dest,code,4),
            Push(dest) => create_command_single_arg_full_dest!(*dest,code,5),

            PushRegister(dest) => create_command_single_arg!(*dest,code,6),
            Pop(dest) => create_command_single_arg!(*dest,code,7),

            Jump(dest) =>  create_command_single_arg!(*dest,code,8),
            JumpIfZero(dest) => create_command_single_arg!(*dest,code,9),
            JumpIfNotZero(dest) => create_command_single_arg!(*dest,code,10),
            JumpIfEqual(dest) => create_command_single_arg!(*dest,code,11),
            JumpIfNotEqual(dest) => create_command_single_arg!(*dest,code,12),
            JumpIfGreater(dest) => create_command_single_arg!(*dest,code,13),
            JumpIfLess(dest) => create_command_single_arg!(*dest,code,14),
            Compare(dest) => create_command_two_arg!(*dest,code,15),            
            
            GetFromStack(dest) => create_command_two_arg!(*dest,code,16),
            GetFromStackPointer(dest) => create_command_two_arg!(*dest,code,17),

            Malloc(dest) => create_command_single_arg_full_dest!(*dest,code,18),
            GetMemory(dest) => create_command_two_arg!(*dest,code,19),
            SetMemory(dest) => create_command_two_arg!(*dest,code,20),

            Return => create_command_no_arg!(code,21),
            Mul(dest) => create_command_two_arg!(*dest,code,22),
            Div(dest) => create_command_two_arg!(*dest,code,23),
            Or(dest) => create_command_two_arg!(*dest,code,24),
            And(dest) => create_command_two_arg!(*dest,code,25),
            Not(dest) => create_command_two_arg!(*dest,code,26),
            Xor(dest) => create_command_two_arg!(*dest,code,27),
            Nand(dest) => create_command_two_arg!(*dest,code,28),
            TruncateStack(dest) => create_command_single_arg_full_dest!(*dest,code,29),
            Mod(dest) => create_command_two_arg!(*dest,code,30)
        }
    }
}




pub fn get_destinations(bits:DestinationType) -> (DestinationType,DestinationType) {
    let slice = &combined_destination_binary_to_slice(bits);
    if slice.len() != 2*DESTINATION_SIZE { panic!("destination bits is not equal to {:?} bits",2*DESTINATION_SIZE)}
//    println!("{:?}",&slice[0..DESTINATION_SIZE]);
    let a = binary_slice_to_number!(DestinationType,&slice[0..DESTINATION_SIZE]);
    let b = binary_slice_to_number!(DestinationType,&slice[DESTINATION_SIZE..]);
    return (a as DestinationType,b as DestinationType)
}

pub fn get_destinations_slice(bits:DestinationType) -> (DestinationBinary,DestinationBinary) {
    let (a,b) = get_destinations(bits);
    //  let (start,end) =  (0,COMMAND_SIZE/DESTINATION_SIZE);
    return (destination_binary_to_slice(a)[0..].try_into().unwrap(),destination_binary_to_slice(b)[0..].try_into().unwrap());
}

pub fn command_slice_to_binary(sl: &[u8]) -> CommandType {
    return binary_slice_to_number!(CommandType,sl);
}


