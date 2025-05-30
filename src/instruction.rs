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
    Call(StringNumberUnion), // Same as Jump but stores the return address and returns to the
                             // address when ret is called
    JumpIfZero(StringNumberUnion),
    JumpIfNotZero(StringNumberUnion),
    JumpIfEqual(StringNumberUnion),
    JumpIfNotEqual(StringNumberUnion),
    JumpIfGreater(StringNumberUnion),
    JumpIfLess(StringNumberUnion),

    Compare(InstructionParamType,InstructionParamType),

    GetFromStack(InstructionParamType,InstructionParamType),
    GetFromStackPointer(InstructionParamType,InstructionParamType),

    SetStack(InstructionParamType,InstructionParamType),
    SetFromStackPointer(InstructionParamType,InstructionParamType),
    ExtendStack(InstructionParamType, InstructionParamType),
    /// Creates a new memory unit and pushs the unit id of the created unit on to the stack
    Malloc(InstructionParamType),
    Free(InstructionParamType),
    //       Memory unit id        dest                 offset 
    GetMemory(InstructionParamType,InstructionParamType,InstructionParamType),
    //       Memory unit id        src                 offset 
    SetMemory(InstructionParamType,InstructionParamType,InstructionParamType),


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

    DisplayChar(InstructionParamType),
    
    GetFlag(InstructionParamType,InstructionParamType),
    GetStackPointer(InstructionParamType),
    TruncateStackRange(InstructionParamType,InstructionParamType),
    
    // prints string from the stack 
    // arg 1: len of the string.
    // arg 2: end location of string.
    // prints until it reaches a null character or the string len
    Write(InstructionParamType,InstructionParamType),
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
            GetFromStack(a,b) | GetFromStackPointer(a,b) | SetFromStackPointer(a,b) | SetStack(a,b) |
            
            Or(a,b) | And(a,b) | Xor(a,b) | Nand(a,b)|
            GetFlag(a,b) |

            TruncateStackRange(a,b) | 
            ExtendStack(a,b) |
            Write(a,b)
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
                Not(a)| 
                GetStackPointer(a)|
                Malloc(a) | 
                Free(a)
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
                JumpIfLess(s)|
                Call(s) => {
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

            GetMemory(a,b,c) |
            SetMemory(a,b,c) => {
                let mut a_binary = to_binary_slice!(InstructionParamType,*a).to_vec();
                let mut b_binary = to_binary_slice!(InstructionParamType, *b).to_vec();
                let mut c_binary = to_binary_slice!(InstructionParamType, *c).to_vec();

                let mut instr_binary = to_binary_slice!(InstructionNameBinaryType,self.get_instruction_number()).to_vec();
                instr_binary.append(&mut a_binary);
                instr_binary.append(&mut b_binary);
                instr_binary.append(&mut c_binary);
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
            Mov(..) => 1,
            Add(..) => 2,
            Sub(..) => 3,
            Mul(..) => 4,
            Div(..) => 5,
            Mod(..) => 6,
            Display(..) => 7,
            Push(..) => 8,
            PushRegister(..) => 9,
            Pop(..) => 10,
            Jump(..) => 11,
            JumpIfZero(..) => 12,
            JumpIfNotZero(..) => 13,
            JumpIfEqual(..) => 14,
            JumpIfNotEqual(..) => 15,
            JumpIfGreater(..) => 16,
            JumpIfLess(..) => 17,
            Compare(..) => 18,
            GetFromStack(..) => 19,
            GetFromStackPointer(..) => 20,
            SetFromStackPointer(..) => 21,
            Malloc(..) => 22,
            GetMemory(..) => 23,
            SetMemory(..) => 24,
            Or(..) => 25,
            And(..) => 26,
            Not(..) => 27,
            Xor(..) => 28,
            Nand(..) => 29,
            TruncateStack(..) => 30,
            Movf(..) => 31,
            Addf(..) => 32,
            Subf(..) => 33,
            Displayf(..) => 34,
            Mulf(..) => 35,
            Divf(..) => 36,
            Modf(..) => 37,
            Return => 38,
            ExtendStack(..) => 39,
            PushFloatRegister(..) => 40,
            PopFloat(..) => 41,
            DisplayChar(..) => 42,
            GetFlag(..) => 43,
            SetStack(..) => 44,
            GetStackPointer(..) => 45,
            TruncateStackRange(..) => 46,
            Call(..) => 47,
            Write(..) => 48,
            Free(..) => 49
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
            23 => Some(GetMemory(InstructionParamType::default(), InstructionParamType::default(), InstructionParamType::default())),
            24 => Some(SetMemory(InstructionParamType::default(), InstructionParamType::default(), InstructionParamType::default())),
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
            39 => Some(ExtendStack(InstructionParamType::default(),InstructionParamType::default())),
            40 => Some(PushFloatRegister(InstructionParamType::default())),
            41 => Some(PopFloat(InstructionParamType::default())),
            42 => Some(DisplayChar(InstructionParamType::default())),
            43 => Some(GetFlag(InstructionParamType::default(),InstructionParamType::default())),
            44 => Some(SetStack(InstructionParamType::default(),InstructionParamType::default())),
            45 => Some(GetStackPointer(InstructionParamType::default())),
            46 => Some(TruncateStackRange(InstructionParamType::default(),InstructionParamType::default())),
            47 => Some(Call(StringNumberUnion::default())),
            48 => Some(Write(InstructionParamType::default(),InstructionParamType::default())),
            49 => Some(Free(InstructionParamType::default())),
            _ => None,
        }
    }

    pub fn get_param_binary_size(&self) -> (Option<usize>,Option<usize>,Option<usize>) {
        use Instruction::*;
        match self {
            Halt => { (None,None,None) },
            Mov(_,_) => { (Some(REGISTER_PARAM_SIZE),Some(INT_PARAM_SIZE),None)  }
            Add(_,_) | Sub(_,_) | Div(_,_) | Mul(_,_) | Mod(_,_) |
                Addf(_,_) | Subf(_,_) | Divf(_,_) | Mulf(_,_) | Modf(_,_) | 
                Compare(_,_) |
                GetFromStack(_,_) | GetFromStackPointer(_,_) | SetFromStackPointer(_,_) | SetStack(_,_) |
              
                Or(_,_) | And(_,_) | Xor(_,_) | Nand(_,_) |
                GetFlag(_,_) |
                TruncateStackRange(_,_) | 
                ExtendStack(..) |
                Write(_,_)
                => {
                    (Some(REGISTER_PARAM_SIZE),Some(REGISTER_PARAM_SIZE),None)
                }

            Display(_) |
                Displayf(_) |
                DisplayChar(_) | 
                PushRegister(_)| Pop(_) |
                Push(_) | 
                PushFloatRegister(_)| PopFloat(_) |
                TruncateStack(_)|
                Not(_) |
                GetStackPointer(_) |
                Malloc(..) |
                Free(..)
                => {
                    (Some(REGISTER_PARAM_SIZE),None,None)
                }

            Movf(_,_) => {
                (Some(REGISTER_PARAM_SIZE),Some(FLOAT_PARAM_SIZE),None)
            }

            Return => { (None,None,None) } 


            Jump(_) |
                JumpIfZero(_) |
                JumpIfNotZero(_) |
                JumpIfEqual(_) |
                JumpIfNotEqual(_) |
                JumpIfGreater(_) |
                JumpIfLess(_) |
                Call(_) => {
                    (Some(JUMP_DESTINATION_PARAM_SIZE),None,None)
                }

              GetMemory(..) |
                SetMemory(..) => {
                    (Some(REGISTER_PARAM_SIZE),Some(REGISTER_PARAM_SIZE),Some(REGISTER_PARAM_SIZE))
                }               
            _ => {
                (None,None,None)
            }
        }
    }

}

