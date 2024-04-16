use crate::{
    instruction::Instruction,
    constants_and_types::*,
};

use crate::{
    to_binary_slice,binary_slice_to_number,
    ones_complement,twos_complement,integer_from_twos_complement,
    jump,
    binary_to_float,to_float_repr
};


use std::collections::HashMap;
pub struct VM {
    registers: [RegisterDataType;5],
    floating_point_registers: [FloatRegisterDataType;5],
    stack:Vec<[u8;STACK_DATA_SIZE]>,
    sp:usize,
   pub instructions:[Instruction;1000],
    command_pointer: usize, 
    last_command:usize,
    flags:[u8;16],
    return_addresses:Vec<usize>,
    labels:HashMap<String,(usize,Option<usize>)>,
    memory: Vec<Option<RegisterDataType>>,
}

impl VM {
    pub fn new() -> Self {
        const ARRAY_REPEAT_VALUE:Instruction = Instruction::Halt;
        Self {
            registers: [0; 5],
            floating_point_registers: [0.0;5],
            stack: Vec::new(),
            sp: 0,
            instructions:[ARRAY_REPEAT_VALUE;1000],
            command_pointer: 0,
            last_command:1, // 0th element will be a Jump to main label
            flags: [0;16],
            labels:HashMap::new(),
            return_addresses: Vec::new(),
            memory:Vec::new(),
        }
    }

    pub fn from_raw_instructions(s:String) -> Self {
        let mut instructions = {
            let mut v:Vec<Instruction> = Vec::new();
            let mut i = INSTRUCTION_NAME_SIZE; 
            let step_by = INSTRUCTION_NAME_SIZE;
            while i < s.len(){
                let ch = s.chars().nth(i);
                match ch {
                    Some('0') | Some('1') => {}
                    Some(a) => {
                        println!("Bytecode Corruption Error: Unexpected character {:?}",a);
                        std::process::exit(1);
                    }
                    None => {
                        println!("Character at index: {:?} was not found.",i);
                        std::process::exit(1);

                    }
                }
                let instruction_binary:Vec<u8> = s[i-INSTRUCTION_NAME_SIZE..i].chars().map(|x| x.to_digit(2).unwrap() as u8).collect();
                let instruction_num = binary_slice_to_number!(u32,instruction_binary);
                let mut instruction = match Instruction::get_default_from_number(instruction_num as usize) {
                    Some(i) => i,
                    None => {
                        println!("Bytecode Error: Instruction with code: {:?} not found. Instruction code in decimal: {:?}",instruction_binary,instruction_num);
                        std::process::exit(1);
                    }
                };
                if instruction == Instruction::Halt || instruction == Instruction::Return {
                    v.push(instruction);
                    i += step_by; continue;
                }
                
                let param_size = instruction.get_param_binary_size();
                match param_size {
                    (None,None) => {
                        println!("Bytecode Corruption Error: expected parameters, found nothing for instruction: {:?}",instruction);
                        std::process::exit(1);
                    }
                    (None,Some(_)) => unreachable!(),
                    _ => {}
                }
                use Instruction::*;
                match instruction {
                    Add(ref mut a, ref mut b) | Sub(ref mut a, ref mut b) | Div(ref mut a, ref mut b) | Mul(ref mut a, ref mut b) | Mod(ref mut a, ref mut b) |
                        Addf(ref mut a, ref mut b) | Subf(ref mut a, ref mut b) | Divf(ref mut a, ref mut b) | Mulf(ref mut a, ref mut b) | Modf(ref mut a, ref mut b) | 
                        Compare(ref mut a, ref mut b) |
                        GetFromStack(ref mut a, ref mut b) | GetFromStackPointer(ref mut a, ref mut b) | SetFromStackPointer(ref mut a, ref mut b) |
                        GetMemory(ref mut a, ref mut b) |
                        SetMemory(ref mut a, ref mut b) |
                        Or(ref mut a, ref mut b) | And(ref mut a, ref mut b) | Xor(ref mut a, ref mut b) | Nand(ref mut a, ref mut b) |
                        GetFlag(ref mut a, ref mut b)=> {

                            let (size_a,size_b) = (param_size.0.unwrap(),param_size.1.unwrap());
                            let param_a = binary_slice_to_number!(InstructionParamType,s[i..i+size_a]
                                .chars()
                                .map(|x| x.to_digit(2).unwrap() as u8)
                                .collect::<Vec<u8>>());
                            let param_b = binary_slice_to_number!(InstructionParamType,s[i+size_a..i+size_a+size_b]
                                .chars().
                                map(|x| x.to_digit(2).unwrap() as u8).
                                collect::<Vec<u8>>());
                            *a  = param_a;
                            *b = param_b;
                            v.push(instruction);
                            i += size_a+size_b;
                            i += step_by; continue;
                        }

                    Display(ref mut a) |
                        Displayf(ref mut a) |
                        DisplayChar(ref mut a)| 
                        PushRegister(ref mut a)| Pop(ref mut a) | 
                        PushFloatRegister(ref mut a)| PopFloat(ref mut a) | 
                        TruncateStack(ref mut a)|
                            Not(ref mut a) 
                            => {
                                let size_reg = param_size.0.unwrap();
                            let param = s[i..i+size_reg]
                                .chars()
                                .map(|x| x.to_digit(2).unwrap() as u8)
                                .collect::<Vec<u8>>();
                            let param_int = binary_slice_to_number!(InstructionParamType,param);
                                *a = param_int;
                            v.push(instruction);   
                            i += size_reg;
                                i += step_by; continue;
                                
                            }

                        // Register, integer 
                        Mov(ref mut a,ref mut b) => {
                            let (size_a,size_b) = (param_size.0.unwrap(),param_size.1.unwrap());
                            let param_a = binary_slice_to_number!(InstructionParamType,s[i..i+size_a]
                                .chars()
                                .map(|x| x.to_digit(2).unwrap() as u8)
                                .collect::<Vec<u8>>());
                            let param_b = binary_slice_to_number!(InstructionParamType,s[i+size_a..i+size_a+size_b]
                                .chars().
                                map(|x| x.to_digit(2).unwrap() as u8).
                                collect::<Vec<u8>>());
                            let param_b_int = integer_from_twos_complement!(iInstructionParamType,InstructionParamType,param_b);
                        *a  = param_a;
                            *b = param_b_int;
                        v.push(instruction.clone());
                            i += size_a+size_b;
                            
                            i += step_by; continue;

                        }

                        // Register, Float 
                        Movf(ref mut a,ref mut b) => {
                          let (size_a,size_b) = (param_size.0.unwrap(),param_size.1.unwrap());
                            let param_a = binary_slice_to_number!(InstructionParamType,s[i..i+size_a]
                                .chars()
                                .map(|x| x.to_digit(2).unwrap() as u8)
                                .collect::<Vec<u8>>());
                            let param_b = binary_slice_to_number!(InstructionParamType,s[i+size_a..i+size_a+size_b]
                                .chars().
                                map(|x| x.to_digit(2).unwrap() as u8).
                                collect::<Vec<u8>>());
                            let param_b_float = binary_to_float!(FloatInstructionParamType,InstructionParamType,param_b);
                        *a  = param_a;
                            *b = param_b_float;
                        v.push(instruction.clone());
                            i += size_a+size_b;
                            
                            i += step_by; continue;
                        }
                        Push(ref mut a) => {
                        let size_a = param_size.0.unwrap();
                        let param_a = integer_from_twos_complement!(iInstructionParamType,InstructionParamType,binary_slice_to_number!(InstructionParamType,s[i..i+size_a]
                                .chars()
                                .map(|x| x.to_digit(2).unwrap() as u8)
                                .collect::<Vec<u8>>()));
                        *a = param_a;
                        v.push(instruction.clone());
                        i += size_a;
                        i+= step_by;
                        continue;

                    }

                        // Jump Instructions
                        Jump(ref mut dest) |
                            JumpIfZero(ref mut dest) |
                            JumpIfNotZero(ref mut dest) |
                            JumpIfEqual(ref mut dest) |
                            JumpIfNotEqual(ref mut dest) |
                            JumpIfGreater(ref mut dest) |
                            JumpIfLess(ref mut dest) => {
                                if param_size.0.is_none() {
                                    println!("Bytecode Error: Argument 0 not found for {:?}",instruction.clone());
                                    std::process::exit(1);
                                }
                            let param = s[i..i+param_size.0.unwrap()]
                                .chars()
                                .map(|x| x.to_digit(2).unwrap() as u8).collect::<Vec<u8>>();
                            let param_int = binary_slice_to_number!(InstructionParamType,param);
                            *dest = crate::instruction::StringNumberUnion::Num(param_int);
                            v.push(instruction.clone());
                            i += param_size.0.unwrap();
                            i += step_by; continue;

                        }

                    _ => unimplemented!("{:?}",instruction)


                }

            }
            v
        };
        let to_fill = 1000-instructions.len();
        for _i in 0..to_fill {
            instructions.push(Instruction::Halt);
        }
        let mut vm = Self::new();
        vm.instructions = instructions.try_into().unwrap();
        vm

    }

        

    pub fn run_instruction(&mut self, inst:&Instruction) {
        use Instruction::*;
        match inst {
          Halt => { std::process::exit(0); }

            Mov(dest,val) => {
                let twos_comp = twos_complement!(RegisterDataType,*val);
                self.registers[*dest as usize] =  twos_comp as RegisterDataType;
            }
            Add(a,b) => {
                let reg_a = integer_from_twos_complement!(iRegisterDataType,RegisterDataType,self.registers[*a as usize]);
                let reg_b = integer_from_twos_complement!(iRegisterDataType,RegisterDataType,self.registers[*b as usize]);
                let sum = reg_a + reg_b; 
                let sum_twos_comp = twos_complement!(RegisterDataType,sum);
                self.registers[*a as usize] = sum_twos_comp;
                //self.registers[a as usize]+=self.registers[b as usize];
            }
            Sub(a,b) => {
                let (a,b) = (*a,*b);
                let reg_a = integer_from_twos_complement!(iRegisterDataType,RegisterDataType,self.registers[a as usize]);
                let reg_b = integer_from_twos_complement!(iRegisterDataType,RegisterDataType,self.registers[b as usize]);
                let dif = reg_a - reg_b; 
                let dif_twos_comp = twos_complement!(RegisterDataType,dif);
                self.registers[a as usize] = dif_twos_comp;          
            }

            Display(a) => {
                
                let reg_a = self.registers[*a as usize];
                let real_num = reg_a;
                //let real_num = binary_slice_to_number!(DestinationType,to_binary_slice!(DestinationType,reg_a)[DESTINATION_SIZE..]);
                let twos_comp = integer_from_twos_complement!(iRegisterDataType,RegisterDataType,real_num);
                println!("{:?}",twos_comp);
            }
          DisplayValue(a) => {
                
                println!("{:?}",a);
            }
            Push(a) => {

                let  a =  to_binary_slice!(RegisterDataType,twos_complement!(RegisterDataType,*a));
                
                self.stack.push(a.as_slice().try_into().unwrap());
                self.sp += 1;
            }
            PushRegister(a) => {
                
                let register_data = to_binary_slice!(RegisterDataType,self.registers[*a as usize]).try_into().unwrap();
                
                self.stack.push(register_data);
                self.sp += 1;
            }
            Pop(a) => {
                
                let pop = self.stack.pop();
                if pop.is_none() {
                    println!("Runtime Error: Stack cannot be popped from as stack is empty.");
                    std::process::exit(1);
                }
                let pop_num = binary_slice_to_number!(RegisterDataType,&pop.unwrap());
                self.registers[*a as usize] = pop_num;
                self.sp -= 1;
            }
            PushFloatRegister(a) => {
                let register_data = to_binary_slice!(RegisterDataType,to_float_repr!(FloatRegisterDataType,RegisterDataType,self.floating_point_registers[*a as usize])).try_into().unwrap();

                self.stack.push(register_data);
                self.sp += 1;
            }
            PopFloat(a) => {

                let pop = self.stack.pop();
                self.floating_point_registers[*a as usize] = binary_to_float!(FloatRegisterDataType,RegisterDataType,binary_slice_to_number!(RegisterDataType,&pop.unwrap()));
                self.sp -= 1;
            }
            Jump(a) => {
                let labels = self.labels.clone();
                let insts = self.instructions.clone();
                let mut s = |ad| { 
                    if self.command_pointer > 0 {

                        self.return_addresses.push(self.command_pointer); 
                    }

                    self.set_command_pointer(ad-1);
                };
                jump!(a,labels,insts,s);
                //  jump_inst!(dest,labels,rn); 
            }
            
            JumpIfZero(a) => {
            
                if self.get_flag(ZERO_FLAG).is_some() && *self.get_flag(ZERO_FLAG).unwrap()  == 0{ return };

                   let labels = self.labels.clone();
                let insts = self.instructions.clone();
                let mut s = |ad| {self.run_label_raw_inst(ad);};
                jump!(a,labels,insts,s); 

            }
            JumpIfNotZero(a) => {
            if self.get_flag(ZERO_FLAG).is_some() && *self.get_flag(ZERO_FLAG).unwrap() !=0 { return };

                  let labels = self.labels.clone();
                let insts = self.instructions.clone();
                let mut s = |ad| { self.run_label_raw_inst(ad)};
                jump!(a,labels,insts,s);            
            }
            JumpIfEqual(a) => {
                if *self.get_flag(EQUAL_FLAG).unwrap() == 0 { return; }
                let labels = self.labels.clone();
                let insts = self.instructions.clone();
                let mut s = |ad| { self.set_command_pointer(ad-1); };
                jump!(a,labels,insts,s);
            } 
            JumpIfNotEqual(a) => {
                if *self.get_flag(EQUAL_FLAG).unwrap() == 1 { return; }
                let labels = self.labels.clone();
                let insts = self.instructions.clone();
                let mut s = |ad| {self.set_command_pointer(ad-1); };
                jump!(a,labels,insts,s);     
               
            }

            JumpIfGreater(a) => {
                if *self.get_flag(GREATER_THAN_FLAG).unwrap() == 0 { return; }
                let labels = self.labels.clone();
                let insts = self.instructions.clone();
                let mut s = |ad| { self.run_label_raw_inst(ad) };
            
                jump!(a,labels,insts,s);   
            }
            JumpIfLess(a) => {
                if *self.get_flag(LESS_THAN_FLAG).unwrap() == 0 { return; }
                let labels = self.labels.clone();
                let insts = self.instructions.clone();
                let mut s = |ad| {  self.run_label_raw_inst(ad)  };
                jump!(a,labels,insts,s);    
            }

            Compare(a,b) => {

                let (a,b) = (*a,*b);
                let reg_a = self.registers[a as usize];
                let reg_b = self.registers[b as usize];
                let _ = self.set_flag(ZERO_FLAG, (reg_a==0 && reg_b == 0) as u8);
                let _ = self.set_flag(EQUAL_FLAG,(reg_a==reg_b) as u8);
                let _ =self.set_flag(GREATER_THAN_FLAG,(reg_a>reg_b) as u8);
                let _ =self.set_flag(LESS_THAN_FLAG,(reg_a<reg_b) as u8);
            }

            GetFromStack(sp,reg) => {
                let (reg,sp) = (*reg,*sp);
                let regsp = integer_from_twos_complement!(iRegisterDataType,RegisterDataType,self.registers[sp as usize]) as RegisterDataType;

                if let Some(content) = self.stack.get(regsp as usize) {
                    self.registers[reg as usize] = binary_slice_to_number!(RegisterDataType,content);
                }else {
                    panic!("Cannot get element number: {:?} from stack with total items: {:?}",regsp,self.stack.len());
                }
            }
            GetFromStackPointer(offset,reg) => {
                let (offset,reg) = (*offset,*reg);
                let sp = self.sp;
                let regoffset = integer_from_twos_complement!(iRegisterDataType,RegisterDataType,self.registers[offset as usize]);
              
                let index = sp -1- regoffset as usize;
                if let Some(content) = self.stack.get(index) {
                    self.registers[reg as usize] = binary_slice_to_number!(RegisterDataType,content);
                }else {
                    panic!("Cannot get element number: {:?} from stack with total items: {:?}",sp,self.stack.len());
                }

            }
            SetFromStackPointer(offset,reg) => {
                let (offset,reg) = (*offset,*reg);
                let sp = self.sp;
                let regoffset = integer_from_twos_complement!(iRegisterDataType,RegisterDataType,self.registers[offset as usize]);
                let index = sp  -1- regoffset as usize;
                if index >= self.stack.len() {
                    panic!("Cannot set element number: {:?} from stack with total items: {:?}",sp,self.stack.len());
                }
                self.stack[index] =  to_binary_slice!(RegisterDataType,self.registers[reg as usize]).as_slice().try_into().unwrap();
                

            }

            Malloc(size) => {
                // Check if any available memory with that size is available
                if let Some(available_memory) = self.is_memory_available(*size as usize) {
                    // all good, idk what to do here
                    self.stack.push(to_binary_slice!(RegisterDataType,available_memory.0).try_into().unwrap());
                    self.sp += 1;
                }else {
                    let p = self.allocate_memory(*size as usize);
                    self.stack.push(to_binary_slice!(RegisterDataType,p).try_into().unwrap());
                    self.sp += 1;
                }
            }

            GetMemory(reg,loc) => {
                
                let (reg,loc) = (*reg,*loc);
                if let Some(val) = self.memory.get(loc as usize) {
                    self.registers[reg as usize] = val.unwrap_or(0);
                }else {
                    panic!("Unable to get location {:?} that is out of memory.",loc);
                }
            }

            SetMemory(reg,loc) => {
                let (reg,loc) = (*reg,*loc);
                if let Some(val) = self.memory.get_mut(loc as usize){
                    *val = self.registers.get(reg as usize).copied();
                }else {
                    panic!("couldn't get memory location:{:?}",loc);
                }
            }

            Return => {
                let last_address = self.return_addresses.pop();
                if let Some(la) = last_address {
                    self.command_pointer  = la;
                }else {
                    panic!("Could not return as last address is not set");
                }
            }

            Mul(a,b) => {
                
                let (a,b) = (*a,*b);
                let reg_a = self.registers[a as usize];
                let reg_b = self.registers[b as usize];
                let mul = integer_from_twos_complement!(iRegisterDataType,RegisterDataType,reg_a) * integer_from_twos_complement!(iRegisterDataType,RegisterDataType,reg_b);
                let mul_twos_comp = twos_complement!(RegisterDataType,mul);
                self.registers[a as usize] = mul_twos_comp;
            } 
            Div(a,b) => {
                
                let (a,b) = (*a,*b);
                let reg_a = self.registers[a as usize];
                let reg_b = self.registers[b as usize];
                let div = integer_from_twos_complement!(iRegisterDataType,RegisterDataType,reg_a) / integer_from_twos_complement!(iRegisterDataType,RegisterDataType,reg_b);
                let div_twos_comp = twos_complement!(RegisterDataType,div);

                self.registers[a as usize] = div_twos_comp;

            }
            Mod(a,b) => {
                
                let (a,b) = (*a,*b);
                let reg_a = self.registers[a as usize];
                let reg_b = self.registers[b as usize];
                let rmod = integer_from_twos_complement!(iRegisterDataType,RegisterDataType,reg_a) % integer_from_twos_complement!(iRegisterDataType,RegisterDataType,reg_b);
                let mod_twos_comp = twos_complement!(RegisterDataType,rmod);
                self.registers[a as usize] = mod_twos_comp;
            } 
            Or(a,b) => {
                let (a,b) = (*a,*b);
                
                self.registers[a as usize]|=self.registers[b as usize];

            }
            And(a,b) => {
                
                let (a,b) = (*a,*b);
                self.registers[a as usize]&=self.registers[b as usize];

            } 
            Not(a) => {
               
                self.registers[*a as usize] = !self.registers[*a as usize];
            }
            Xor(a,b) => {
                
                let (a,b) = (*a,*b);
                self.registers[a as usize] ^= self.registers[b as usize];
            } 
            Nand(_a,_b) => unimplemented!(), //Idk if i should add it
        
            TruncateStack(a) => {
                
                let val = self.registers[*a as usize];
                for _ in 0..val {
                    self.stack.pop();
                }
            }

           Movf(a,b) => {
                let (a,b) = (*a,*b);
                self.floating_point_registers[a as usize] =  b as FloatRegisterDataType;
               
            }
            
            Addf(a,b) =>{ 
                let (a,b) = (*a,*b);
                let reg_a = self.floating_point_registers[a as usize];
                let reg_b = self.floating_point_registers[b as usize];
                let sum = reg_a + reg_b;
                self.floating_point_registers[a as usize] = sum;
                //self.registers[a as usize]+=self.registers[b as usize];
            }
            Subf(a,b) => {
                let (a,b) = (*a,*b);
                let reg_a = self.floating_point_registers[a as usize];
                let reg_b = self.floating_point_registers[b as usize];
                let dif = reg_a - reg_b;
                self.floating_point_registers[a as usize] = dif;

            }

            Displayf(a) => {
                let reg_a = self.floating_point_registers[*a as usize];
                println!("{:?}",reg_a);
            }

            Mulf(a,b) => {
                 let (a,b) = (*a,*b);
                let reg_a = self.floating_point_registers[a as usize];
                let reg_b = self.floating_point_registers[b as usize];
                let prod = reg_a * reg_b;
                self.floating_point_registers[a as usize] = prod;
            }
            Divf(a,b) => {
                let (a,b) = (*a,*b);
                let reg_a = self.floating_point_registers[a as usize];
                let reg_b = self.floating_point_registers[b as usize];
                let q = reg_a / reg_b;
                self.floating_point_registers[a as usize] = q;


            }
            DisplayChar(a) => {
                let a = integer_from_twos_complement!(iRegisterDataType,RegisterDataType,self.registers[*a as usize]);
                

                let ch = char::from_u32(a.try_into().unwrap());
                if ch.is_none() {
                    println!("Run Time Error: Cannot get character from number {:?}",a);
                    std::process::exit(1);
                }
                print!("{}",ch.unwrap());
            }
            GetFlag(dest,flagregno) => {
                let (dest,flagregno) = (*dest,*flagregno);
                let flag = integer_from_twos_complement!(iRegisterDataType,RegisterDataType,self.registers[flagregno as usize]);
                if let Some(f) = self.flags.get(flag as usize) {
                    self.registers[dest as usize] = twos_complement!(RegisterDataType,*f as iRegisterDataType); 
                }else {
                    println!("Runtime Error: Could not get flag number {:?} as it does not exist.",flag);
                    std::process::exit(1);
                }
            }
            _ => unimplemented!()

        }
    }

    pub fn is_memory_available(&self, size:usize) -> Option<(usize,usize)> {
        let mut start = 0;
        let mut end = size as usize-1;
        while end <= self.memory.len() {
            let cur_window = &self.memory[start..end];


            if cur_window.iter().find(|x| x.is_some()).is_some() {
                start += 1;
                end +=1;
                continue
            }else {

                return Some((start,end))
            }
        }
        return None;

    }

    pub fn allocate_memory(&mut self, size:usize) -> usize {
        let p = self.memory.len();
        self.memory.resize(size,None);
        p
    }

    // See eval for more info 
    #[allow(dead_code)]
    pub fn run_label_inst(&mut self, label:String,is_main:bool) {
        if self.labels.get(&label).is_none() { panic!("Label with name {:?} does not exist.",label)}

        let (label_start,label_end_option) = &self.labels.get(&label).unwrap(); 
        
        if label_end_option.is_none() {
            panic!("No end found for label: {:?}",label);
        }

        let label_end = label_end_option.unwrap();

        if *label_start == label_end {
            panic!("Cannot define empty label");
        }

        if is_main {
            self.return_addresses.push(self.last_command);
        }else {
            self.return_addresses.push(self.command_pointer.into());
        }
        self.command_pointer = *label_start;
        while self.command_pointer < label_end-1 {
           self.run_current_inst();
            self.command_pointer +=1;
        }
        self.run_current_inst();
        
    }

    pub fn run_label_raw_inst(&mut self,label_loc:usize) {
             // Push command pointer onto a buffer 
        self.return_addresses.push(self.command_pointer.into());
        self.command_pointer = label_loc;
        while self.instructions[self.command_pointer] != Instruction::Return {
            let cur_inst = self.instructions[self.command_pointer].clone();
            self.run_instruction(&cur_inst);
            self.command_pointer += 1;
        }


        let cur_inst = self.instructions[self.command_pointer].clone();
        self.run_instruction(&cur_inst);

        self.command_pointer += 1;
    }

    pub fn run_current_inst(&mut self) {
        
        let cur_inst = self.instructions[self.command_pointer].clone();
        self.run_instruction(&cur_inst);
     }

     // This is when you manually use the VM by itself 
     // Ex :
     // vm.start_label("main")
     // vm.add_instruction(Instruction::Mov(0,10));
     // vm.add_instruction(Instruction::Display(0));
     // vm.end_label("main")
     // vm.register_start();
     // vm.eval();
    #[allow(dead_code)]
     pub fn eval(&mut self) {
         self.run_label_inst("main".to_string(),true);
     }
    pub fn eval_raw(&mut self) {
        loop {
            self.run_current_inst();
            self.command_pointer += 1;
        }
    }

    pub fn add_instruction(&mut self, inst:Instruction) {
       self.instructions[self.last_command] = inst;
        self.last_command += 1;
   }

    pub fn get_raw_byte_code(&mut self) -> String {
        let mut fin = String::new();
        let mut i = 0;
        while i<=self.last_command {

            use Instruction::*;
            let inst =match &self.instructions[i] {
                Jump(s) => {
                    use crate::instruction::StringNumberUnion::*;
                    match s {
                        String(s) => {
                            if let Some(v) = self.labels.get(s) {
                                let loc = v.0;
                                Jump(Num(loc as u32))
                            }else {
                                unreachable!()
                            }
                        }
                        Num(n) => Jump(Num(*n)),
                    }
                }
                JumpIfNotEqual(s) => {
                    use crate::instruction::StringNumberUnion::*;
                    match s {
                        String(s) => {
                            if let Some(v) = self.labels.get(s) {
                                let loc = v.0;
                                JumpIfNotEqual(Num(loc as u32))
                            }else {
                                unreachable!()
                            }
                        }
                        Num(n) => JumpIfNotEqual(Num(*n)),
                    }
                }
                JumpIfLess(s) => {
                    use crate::instruction::StringNumberUnion::*;
                    match s {
                        String(s) => {
                            if let Some(v) = self.labels.get(s) {
                                let loc = v.0;
                                JumpIfLess(Num(loc as u32))
                            }else {
                                unreachable!()
                            }
                        }
                        Num(n) => JumpIfLess(Num(*n)),
                    }
                }
                JumpIfGreater(s) => {
                    use crate::instruction::StringNumberUnion::*;
                    match s {
                        String(s) => {
                            if let Some(v) = self.labels.get(s) {
                                let loc = v.0;
                                JumpIfGreater(Num(loc as u32))
                            }else {
                                unreachable!()
                            }
                        }
                        Num(n) => JumpIfGreater(Num(*n)),
                    }
                }

                JumpIfZero(s) => {
                    use crate::instruction::StringNumberUnion::*;
                    match s {
                        String(s) => {
                            if let Some(v) = self.labels.get(s) {
                                let loc = v.0;
                                JumpIfZero(Num(loc as u32))
                            }else {
                                unreachable!()
                            }
                        }
                        Num(n) => JumpIfZero(Num(*n)),
                    }
                }
                JumpIfNotZero(s) => {
                    use crate::instruction::StringNumberUnion::*;
                    match s {
                        String(s) => {
                            if let Some(v) = self.labels.get(s) {
                                let loc = v.0;
                                JumpIfNotZero(Num(loc as u32))
                            }else {
                                unreachable!()
                            }
                        }
                        Num(n) => JumpIfNotZero(Num(*n)),
                    }
                }
                JumpIfEqual(s) => {

                    use crate::instruction::StringNumberUnion::*;
                    match s {
                        String(s) => {
                            if let Some(v) = self.labels.get(s) {
                                let loc = v.0;
                                JumpIfEqual(Num(loc as u32))
                            }else {
                                unreachable!()
                            }
                        }
                        Num(n) => JumpIfEqual(Num(*n)),
                    }

                }

                _ => self.instructions[i].clone()
            };

            let inst_string = inst.to_binary().iter().map(|&b| b.to_string()).collect::<Vec<String>>().join("");
            fin.push_str(inst_string.as_str());
            i += 1;
        }
        return fin
    }

    pub fn get_flag(&self,flag:usize) -> Option<&u8> {
        return self.flags.get(flag);
    }

    pub fn set_flag(&mut self, flag:usize,value:u8) -> Result<(),Box<dyn std::error::Error>> {
        
        if let Some(flag)= self.flags.get_mut(flag) {
            *flag = to_binary_slice!(u8,value)[7];            
        }else {
            panic!("Flag {:?} not found",flag);
        }
        Ok(())
    }

    pub fn start_label(&mut self, flag_name:&str) {
        if self.labels.get(&flag_name.to_string()).is_some() { panic!("Cannot create label with name: {:?} as it is already defined.",{flag_name}); }
        
        self.labels.insert(flag_name.to_string(),(self.last_command,None));
    }

    pub fn end_label(&mut self, flag_name:&str) {
        todo!();
        if let Some((_start,end)) = self.labels.get_mut(&flag_name.to_string()) {
                if end.is_some() {
                    panic!("Unable to end label {:?} as it is already ended",flag_name);
                }   
                *end = Some(self.last_command);
        }else {
            panic!("Unable to end label as it does not exist: {:?}",flag_name);
        }
    }

    pub fn register_start(&mut self) {
        if let Some((_start,_)) = self.labels.get(&"main".to_string()) {
            let lc = self.last_command;
            self.last_command = 0;
            self.add_instruction(Instruction::Jump(crate::instruction::StringNumberUnion::String("main".to_string())));
            self.last_command = lc;
        }else {
            panic!("Unable to set main as main label does not exist.");
        }
    }


    pub fn set_command_pointer(&mut self, new_val:usize) {
        self.command_pointer = new_val;
    }
    
    pub fn last_command(&self) -> usize {
        return self.last_command;
    }

    pub fn create_label(&mut self, label_index:usize, label_name:&String) {
        if self.labels.get(label_name).is_some() {
            println!("Cannot create label with name `{:?}` as it already exists.",label_name);
            std::process::exit(1);
        }
        self.labels.insert(label_name.clone(),(label_index,None));
    }

    pub fn labels(&self) -> &HashMap<String,(usize,Option<usize>)> {
        &self.labels
    }

    pub fn labels_mut(&mut self) -> &mut HashMap<String,(usize,Option<usize>)> {
        &mut self.labels
    }
}
