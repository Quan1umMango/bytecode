 /// Instructions are absrtactions over commands. Instead of writing Mov(0b0001000) you can write
 /// Mov(0,1). Much easier!
 
use crate::{commands::*,constants_and_types::*};
use crate::{to_binary_slice,binary_slice_to_number};
use std::collections::HashMap;


 #[derive(Debug)]
 pub enum Instruction {
     Halt,
     Mov(InstructionParamType,InstructionParamType),
     Add(InstructionParamType,InstructionParamType),
     Sub(InstructionParamType,InstructionParamType),
   Mul(InstructionParamType,InstructionParamType),
    Div(InstructionParamType,InstructionParamType),
     Mod(InstructionParamType,InstructionParamType),
     Display(InstructionParamType),
     Push(InstructionParamType),
     PushRegister(InstructionParamType),
     Pop(InstructionParamType),
     
     Jump(String),
     JumpIfZero(String),
     JumpIfNotZero(String),
     JumpIfEqual(String),
     JumpIfNotEqual(String),
    JumpIfGreater(String),
     JumpIfLess(String),
 
     Compare(InstructionParamType,InstructionParamType),

     GetFromStack(InstructionParamType,InstructionParamType),
     GetFromStackPointer(InstructionParamType,InstructionParamType),

    Malloc(InstructionParamType),
     GetMemory(InstructionParamType,InstructionParamType),
     SetMemory(InstructionParamType,InstructionParamType),

    
    Or(InstructionParamType,InstructionParamType),
    And(InstructionParamType,InstructionParamType),
    Not(InstructionParamType),
    Xor(InstructionParamType,InstructionParamType),
    Nand(InstructionParamType,InstructionParamType),
    
     TruncateStack(InstructionParamType),
 }

 impl Instruction {
     pub fn to_command(&self,labels:&HashMap<String,(usize,Option<usize>)>) -> Command {
         use Instruction::*;
         
         match self {
             Halt => Command::Halt,
             Mov(a,v) =>Command::Mov(join_params_to_command_params(*a,*v)),
             Add(a,v) => Command::Add(join_params_to_command_params(*a,*v)),
             Sub(a,v) => Command::Sub(join_params_to_command_params(*a,*v)),
             Mod(a,v) => Command::Mod(join_params_to_command_params(*a,*v)),
             Display(a) => Command::Display(join_params_to_command_params(*a,0)),
             Push(a) => Command::Push(*a),
             PushRegister(a) => Command::PushRegister(join_params_to_command_params(*a,0)),
             Pop(a) => Command::Pop(join_params_to_command_params(*a,0)),
            Jump(a) => {
                 if let Some((start,_end))=  labels.get(a) {
                     Command::Jump(join_params_to_command_params((*start).try_into().unwrap(),0))
                 }else {
                     panic!("Cannot find label as it is not defined: {:?}",a);
                 }
                 
             }
             JumpIfZero(a) => {
                 if let Some((start,_end))=  labels.get(a) {
                     Command::JumpIfZero(join_params_to_command_params((*start).try_into().unwrap(),0))
                 }else {
                     panic!("Cannot find label as it is not defined: {:?}",a);
                 }
                 
             }
             JumpIfNotZero(a) => {
                 if let Some((start,_end))=  labels.get(a) {
                     Command::JumpIfNotZero(join_params_to_command_params((*start).try_into().unwrap(),0))
                 }else {
                     panic!("Cannot find label as it is not defined: {:?}",a);
                 }
                 
             }
           JumpIfEqual(a) => {
                 if let Some((start,_end))=  labels.get(a) {
                     Command::JumpIfEqual(join_params_to_command_params((*start).try_into().unwrap(),0))
                 }else {
                     panic!("Cannot find label as it is not defined: {:?}",a);
                 }

             }
             JumpIfNotEqual(a) => {
                 if let Some((start,_end))=  labels.get(a) {
                     Command::JumpIfNotEqual(join_params_to_command_params((*start).try_into().unwrap(),0))
                 }else {
                     panic!("Cannot find label as it is not defined: {:?}",a);
                 }

             }
             JumpIfGreater(a) => {
                 if let Some((start,_end))=  labels.get(a) {
                     Command::JumpIfGreater(join_params_to_command_params((*start).try_into().unwrap(),0))
                 }else {
                     panic!("Cannot find label as it is not defined: {:?}",a);
                 }

             }
             JumpIfLess(a) => {
                 if let Some((start,_end))=  labels.get(a) {
                     Command::JumpIfLess(join_params_to_command_params((*start).try_into().unwrap(),0))
                 }else {
                     panic!("Cannot find label as it is not defined: {:?}",a);
                 }

             }

             Compare(a,v) => Command::Compare(join_params_to_command_params(*a,*v)),
            
             GetFromStack(a,v) => Command::GetFromStack(join_params_to_command_params(*a,*v)),
             GetFromStackPointer(a,v) => Command::GetFromStackPointer(join_params_to_command_params(*a,*v)),

             Malloc(a) => Command::Malloc(*a),
             GetMemory(a,v) => Command::GetMemory(join_params_to_command_params(*a,*v)),
             SetMemory(a,v) =>Command::SetMemory(join_params_to_command_params(*a,*v)),

             Mul(a,v)=> Command::Mul(join_params_to_command_params(*a,*v)),
             Div(a,v)=> Command::Div(join_params_to_command_params(*a,*v)),

             Or(a,v)=> Command::Or(join_params_to_command_params(*a,*v)),
             And(a,v)=> Command::And(join_params_to_command_params(*a,*v)),
             Not(a)=> Command::Not(join_params_to_command_params(*a,0)),
             Xor(a,v)=> Command::Xor(join_params_to_command_params(*a,*v)),
             Nand(a,v)=> Command::Nand(join_params_to_command_params(*a,*v)),
            TruncateStack(a) => Command::TruncateStack(join_params_to_command_params(*a,0)),
         }
     }
 }

 fn join_params_to_command_params(a:InstructionParamType,v:InstructionParamType) -> DestinationType {
        let len = to_binary_slice!(InstructionParamType,0).len(); 
        let mut v = to_binary_slice!(InstructionParamType,v)[len-DESTINATION_SIZE..].to_vec();      
        let mut a = to_binary_slice!(InstructionParamType,a)[len-DESTINATION_SIZE..].to_vec();
     a.append(&mut v);
        
        binary_slice_to_number!(DestinationType,a)
  
 }

 fn slice_to_binary_u16(sl:&[u8]) -> u16 {
 let mut num: u32 = 0;
    for i in 0..sl.len() {
        if sl[i] == 0 {
            continue;
        }
        num += 1 << (sl.len() - 1 - i);
    }
    num as u16
 }
