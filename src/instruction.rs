use crate::constants_and_types::*;
use crate::{to_binary_slice,binary_slice_to_number,ones_complement,twos_complement,to_float_repr};

#[derive(Debug,Clone,PartialEq)]
pub enum StringNumberUnion {
    String(String),
    Num(u32),
}

impl Default for StringNumberUnion {
    fn default() -> Self { 
        return Self::Num(0);
    }
}

#[derive(Debug,Clone,PartialEq)]
pub enum Instruction {
    Halt,
    Mov(InstructionParamType,iInstructionParamType),
    Add(InstructionParamType,InstructionParamType),
    Sub(InstructionParamType,InstructionParamType),
    Mul(InstructionParamType,InstructionParamType),
    Div(InstructionParamType,InstructionParamType),
    Mod(InstructionParamType,InstructionParamType),
    Display(InstructionParamType),
    Push(iInstructionParamType),
    PushRegister(InstructionParamType),
    Pop(InstructionParamType),

    Jump(StringNumberUnion),
    JumpIfZero(StringNumberUnion),
    JumpIfNotZero(StringNumberUnion),
    JumpIfEqual(StringNumberUnion),
    JumpIfNotEqual(StringNumberUnion),
    JumpIfGreater(StringNumberUnion),
    JumpIfLess(StringNumberUnion),

    Compare(InstructionParamType,InstructionParamType),

    GetFromStack(InstructionParamType,InstructionParamType),
    GetFromStackPointer(InstructionParamType,InstructionParamType),

    SetFromStackPointer(InstructionParamType,InstructionParamType),
    Malloc(InstructionParamType),
    GetMemory(InstructionParamType,InstructionParamType),
    SetMemory(InstructionParamType,InstructionParamType),


    Or(InstructionParamType,InstructionParamType),
    And(InstructionParamType,InstructionParamType),
    Not(InstructionParamType),
    Xor(InstructionParamType,InstructionParamType),
    Nand(InstructionParamType,InstructionParamType),

    TruncateStack(InstructionParamType),


    Movf(InstructionParamType,FloatInstructionParamType),
    Addf(InstructionParamType, InstructionParamType),
    Subf(InstructionParamType, InstructionParamType),
    Displayf(InstructionParamType),
    Mulf(InstructionParamType, InstructionParamType),
    Divf(InstructionParamType, InstructionParamType),
    Modf(InstructionParamType, InstructionParamType),
    PushFloatRegister(InstructionParamType),
    PopFloat(InstructionParamType),
    Return,

    DisplayValue(InstructionParamType),
    DisplayChar(InstructionParamType),
}

impl Instruction {

    pub fn to_binary(&self) -> Vec<u8> {
        use Instruction::*;
        match self {
            Halt => { return to_binary_slice!(InstructionNameBinaryType,0).to_vec()},
            Mov(a,b) => {
                let mut a_binary = to_binary_slice!(InstructionParamType,*a).to_vec();
                let b = twos_complement!(InstructionParamType,*b);
                let mut b_binary = to_binary_slice!(InstructionParamType,b).to_vec();
                let mut instr_binary = to_binary_slice!(InstructionNameBinaryType,1).to_vec();

                instr_binary.append(&mut a_binary);
                instr_binary.append(&mut b_binary);
                return instr_binary
            }
            Add(a,b) | Sub(a,b) | Div(a,b) | Mul(a,b) | Mod(a,b) |
            Addf(a,b) | Subf(a,b) | Divf(a,b) | Mulf(a,b) | Modf(a,b) | 
            Compare(a,b) |
            GetFromStack(a,b) | GetFromStackPointer(a,b) | SetFromStackPointer(a,b) |
            GetMemory(a,b) |
            SetMemory(a,b) |
            Or(a,b) | And(a,b) | Xor(a,b) | Nand(a,b)
                => {
                
                let mut a_binary = to_binary_slice!(InstructionParamType,*a).to_vec();
                let mut b_binary = to_binary_slice!(InstructionParamType,*b).to_vec();

                let mut instr_binary = to_binary_slice!(InstructionNameBinaryType,self.get_instruction_number()).to_vec();

                instr_binary.append(&mut a_binary);
                instr_binary.append(&mut b_binary);
                    return instr_binary
                }

            Display(a) |
                Displayf(a) | 
                DisplayChar(a) |
                PushRegister(a)|Pop(a)|
                PushFloatRegister(a) |PopFloat(a)|
                TruncateStack(a)|
                Not(a) 
                => {
                    let mut a_binary = to_binary_slice!(InstructionParamType,*a).to_vec();
                    let mut instr_binary = to_binary_slice!(InstructionNameBinaryType,self.get_instruction_number()).to_vec();
                    instr_binary.append(&mut a_binary);
                    return instr_binary;
                }

            Movf(a,b) => {
                let mut a_binary = to_binary_slice!(InstructionParamType,*a).to_vec();
                let mut b_binary = to_binary_slice!(InstructionParamType,to_float_repr!(FloatInstructionParamType,InstructionParamType,*b)).to_vec();
 
                let mut instr_binary = to_binary_slice!(InstructionNameBinaryType,self.get_instruction_number()).to_vec();

                instr_binary.append(&mut a_binary);
                instr_binary.append(&mut b_binary);
                return instr_binary               
            }

            Return => return to_binary_slice!(InstructionNameBinaryType,self.get_instruction_number()).to_vec(),
            Jump(s) |  
                JumpIfZero(s)|
                JumpIfNotZero(s)|
                JumpIfEqual(s)|
                JumpIfNotEqual(s)|
                JumpIfGreater(s)|
                JumpIfLess(s)
                => {
                    use StringNumberUnion::*;
                match s {
                    Num(a) => {
                        let mut a_binary = to_binary_slice!(InstructionParamType,*a).to_vec();
                         let mut instr_binary = to_binary_slice!(InstructionNameBinaryType,self.get_instruction_number()).to_vec();

                instr_binary.append(&mut a_binary);
                return instr_binary     
                    }
                    String(_) => todo!()
                }
            }

            Push(a) => {
                let a = twos_complement!(InstructionParamType,*a);
                let mut a_binary = to_binary_slice!(InstructionParamType,a).to_vec();
                let mut instr_binary = to_binary_slice!(InstructionNameBinaryType,self.get_instruction_number()).to_vec();

                instr_binary.append(&mut a_binary);
                return instr_binary
            } 
            _ => {
                panic!("Unimplemented: {:?}",self);
            }
        }
    }


    pub fn get_instruction_number(&self) -> InstructionNameBinaryType {
        use Instruction::*;
        match self {
            Halt => 0,
            Mov(_, _) => 1,
            Add(_, _) => 2,
            Sub(_, _) => 3,
            Mul(_, _) => 4,
            Div(_, _) => 5,
            Mod(_, _) => 6,
            Display(_) => 7,
            Push(_) => 8,
            PushRegister(_) => 9,
            Pop(_) => 10,
            Jump(_) => 11,
            JumpIfZero(_) => 12,
            JumpIfNotZero(_) => 13,
            JumpIfEqual(_) => 14,
            JumpIfNotEqual(_) => 15,
            JumpIfGreater(_) => 16,
            JumpIfLess(_) => 17,
            Compare(_, _) => 18,
            GetFromStack(_, _) => 19,
            GetFromStackPointer(_, _) => 20,
            SetFromStackPointer(_, _) => 21,
            Malloc(_) => 22,
            GetMemory(_, _) => 23,
            SetMemory(_, _) => 24,
            Or(_, _) => 25,
            And(_, _) => 26,
            Not(_) => 27,
            Xor(_, _) => 28,
            Nand(_, _) => 29,
            TruncateStack(_) => 30,
            Movf(_, _) => 31,
            Addf(_, _) => 32,
            Subf(_, _) => 33,
            Displayf(_) => 34,
            Mulf(_, _) => 35,
            Divf(_, _) => 36,
            Modf(_, _) => 37,
  Return => 38,
            DisplayValue(_) => 39,
 PushFloatRegister(_) => 40,
            PopFloat(_) => 41,
            DisplayChar(_) => 42,
           
        }
    }


    pub fn get_default_from_number(i: usize) -> Option<Instruction> {
        use Instruction::*;

        match i {
            0 => Some(Halt),
            1 => Some(Mov(InstructionParamType::default(), iInstructionParamType::default())),
            2 => Some(Add(InstructionParamType::default(), InstructionParamType::default())),
            3 => Some(Sub(InstructionParamType::default(), InstructionParamType::default())),
            4 => Some(Mul(InstructionParamType::default(), InstructionParamType::default())),
            5 => Some(Div(InstructionParamType::default(), InstructionParamType::default())),
            6 => Some(Mod(InstructionParamType::default(), InstructionParamType::default())),
            7 => Some(Display(InstructionParamType::default())),
            8 => Some(Push(iInstructionParamType::default())),
            9 => Some(PushRegister(InstructionParamType::default())),
            10 => Some(Pop(InstructionParamType::default())),
            11 => Some(Jump(StringNumberUnion::default())),
            12 => Some(JumpIfZero(StringNumberUnion::default())),
            13 => Some(JumpIfNotZero(StringNumberUnion::default())),
            14 => Some(JumpIfEqual(StringNumberUnion::default())),
            15 => Some(JumpIfNotEqual(StringNumberUnion::default())),
            16 => Some(JumpIfGreater(StringNumberUnion::default())),
            17 => Some(JumpIfLess(StringNumberUnion::default())),
            18 => Some(Compare(InstructionParamType::default(), InstructionParamType::default())),
            19 => Some(GetFromStack(InstructionParamType::default(), InstructionParamType::default())),
            20 => Some(GetFromStackPointer(InstructionParamType::default(), InstructionParamType::default())),
            21 => Some(SetFromStackPointer(InstructionParamType::default(), InstructionParamType::default())),
            22 => Some(Malloc(InstructionParamType::default())),
            23 => Some(GetMemory(InstructionParamType::default(), InstructionParamType::default())),
            24 => Some(SetMemory(InstructionParamType::default(), InstructionParamType::default())),
            25 => Some(Or(InstructionParamType::default(), InstructionParamType::default())),
            26 => Some(And(InstructionParamType::default(), InstructionParamType::default())),
            27 => Some(Not(InstructionParamType::default())),
            28 => Some(Xor(InstructionParamType::default(), InstructionParamType::default())),
            29 => Some(Nand(InstructionParamType::default(), InstructionParamType::default())),
            30 => Some(TruncateStack(InstructionParamType::default())),
            31 => Some(Movf(InstructionParamType::default(), FloatInstructionParamType::default())),
            32 => Some(Addf(InstructionParamType::default(), InstructionParamType::default())),
            33 => Some(Subf(InstructionParamType::default(), InstructionParamType::default())),
            34 => Some(Displayf(InstructionParamType::default())),
            35 => Some(Mulf(InstructionParamType::default(), InstructionParamType::default())),
            36 => Some(Divf(InstructionParamType::default(), InstructionParamType::default())),
            37 => Some(Modf(InstructionParamType::default(), InstructionParamType::default())),
            38 => Some(Return),
            39 => Some(DisplayValue(InstructionParamType::default())),
            40 => Some(PushFloatRegister(InstructionParamType::default())),
            41 => Some(PopFloat(InstructionParamType::default())),
            42 => Some(DisplayChar(InstructionParamType::default())),
            _ => None,
        }
    }

    pub fn get_param_binary_size(&self) -> (Option<usize>,Option<usize>) {
        use Instruction::*;
        match self {
            Halt => { (None,None) },
            Mov(_,_) => { (Some(REGISTER_PARAM_SIZE),Some(INT_PARAM_SIZE))  }
            Add(_,_) | Sub(_,_) | Div(_,_) | Mul(_,_) | Mod(_,_) |
                Addf(_,_) | Subf(_,_) | Divf(_,_) | Mulf(_,_) | Modf(_,_) | 
                Compare(_,_) |
                GetFromStack(_,_) | GetFromStackPointer(_,_) | SetFromStackPointer(_,_) |
                GetMemory(_,_) |
                SetMemory(_,_) |
                Or(_,_) | And(_,_) | Xor(_,_) | Nand(_,_)
                => {
                    (Some(REGISTER_PARAM_SIZE),Some(REGISTER_PARAM_SIZE))
                }

            Display(_) |
                Displayf(_) |
                DisplayChar(_) | 
                PushRegister(_)| Pop(_) |
                Push(_) | 
                PushFloatRegister(_)| PopFloat(_) |
                TruncateStack(_)|
                Not(_) 
                => {
                    (Some(REGISTER_PARAM_SIZE),None)
                }

            Movf(_,_) => {
                (Some(REGISTER_PARAM_SIZE),Some(FLOAT_PARAM_SIZE))
            }

            Return => { (None,None) } 


            Jump(_) |
                JumpIfZero(_) |
                JumpIfNotZero(_) |
                JumpIfEqual(_) |
                JumpIfNotEqual(_) |
                JumpIfGreater(_) |
                JumpIfLess(_) => {
                    (Some(JUMP_DESTINATION_PARAM_SIZE),None)
                }

                
            _ => {
                (None,None)
            }
        }
    }

}

