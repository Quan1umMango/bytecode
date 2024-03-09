
use crate::{
    instruction::Instruction,
    commands::*, 
    constants_and_types::*,
    conversions::*
};

use crate::{to_binary_slice,binary_slice_to_number};
use crate::jump;

use std::collections::HashMap;
pub struct VM {
    registers: [RegisterDataType;5],
    stack:Vec<[u8;STACK_DATA_SIZE]>,
    sp:usize,
    commands:[CommandType;1000],
    command_pointer: usize, 
    last_command:usize,
    flags:[u8;16],
    return_addresses:Vec<usize>,
    labels:HashMap<String,(usize,Option<usize>)>,
    memory: Vec<Option<RegisterDataType>>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            registers: [0; 5],
            stack: Vec::new(),
            sp: 0,
            commands: [0; 1000],
            command_pointer: 0,
            last_command:1, // 0th element will be a Jump to main label
            flags: [0;16],
            labels:HashMap::new(),
            return_addresses: Vec::new(),
            memory:Vec::new(),
        }
    }
    pub fn run_command(&mut self,command:Command) {
        use Command::*;
        match command {
            Halt => { self.command_pointer = self.last_command }
            Add(dest) => {
                let (a,b) = get_destinations(dest);
                self.registers[a as usize]+=self.registers[b as usize];
            }
            Sub(dest) => {
                let (a,b) = get_destinations(dest);
                self.registers[a as usize] -= self.registers[b as usize];
            }

            Display(dest) => {
                let (a,_) = get_destinations(dest);
                let reg_a = self.registers[a as usize];
                println!("{:?}",reg_a);
            }
            Mov(dest) => {
                let (a,b) = get_destinations(dest);
                self.registers[a as usize] =  b;
                
            }
            Push(dest) => {
                let a = to_binary_slice!(RegisterDataType,dest);
                self.stack.push(a.as_slice().try_into().unwrap());
                self.sp += 1;
            }
            PushRegister(dest) => {
                let (a,_) = get_destinations(dest);
                let register_data = to_binary_slice!(RegisterDataType,self.registers[a as usize]).try_into().unwrap();

                self.stack.push(register_data);
                self.sp += 1;
            }
            Pop(dest) => {
                let (a,_) = get_destinations(dest);
                let pop = self.stack.pop();
                self.registers[a as usize] = binary_slice_to_number!(RegisterDataType,&pop.unwrap()).try_into().unwrap();
                self.sp -= 1;
            }
            Jump(dest) => {
                let labels = self.labels.clone();
                let mut rn = |name:String,b:bool,loc:usize,raw:bool| { if !raw {self.run_label(name,b);}else {self.run_label_raw(loc)} };
                jump!(dest,labels,rn); 
            }
            
            JumpIfZero(dest) => {
            
                if self.get_flag(ZERO_FLAG).is_some() && *self.get_flag(ZERO_FLAG).unwrap()  == 0{ return };

                let labels = self.labels.clone();
                let mut rn = |name:String,b:bool,loc:usize,raw:bool| { if !raw {self.run_label(name,b);}else {self.run_label_raw(loc)} };
                jump!(dest,labels,rn);         
            }
            JumpIfNotZero(dest) => {
            if self.get_flag(ZERO_FLAG).is_some() && *self.get_flag(ZERO_FLAG).unwrap() !=0 { return };

                let labels = self.labels.clone();
                let mut rn = |name:String,b:bool,loc:usize,raw:bool| { if !raw {self.run_label(name,b);}else {self.run_label_raw(loc)} };
                jump!(dest,labels,rn); 
            }
            JumpIfEqual(dest) => {
                if *self.get_flag(EQUAL_FLAG).unwrap() == 0 { return; }
                let labels = self.labels.clone();
                let mut rn = |name:String,b:bool,loc:usize,raw:bool| { if !raw {self.run_label(name,b);}else {self.run_label_raw(loc)} };
                jump!(dest,labels,rn); 
            }
            JumpIfNotEqual(dest) => {
                if *self.get_flag(EQUAL_FLAG).unwrap() == 1 { return; }
                
                let labels = self.labels.clone();
                let mut rn = |name:String,b:bool,loc:usize,raw:bool| { if !raw {self.run_label(name,b);}else {self.run_label_raw(loc)} };
                jump!(dest,labels,rn); 
            }

            JumpIfGreater(dest) => {
                if *self.get_flag(GREATER_THAN_FLAG).unwrap() == 0 { return; }

                let labels = self.labels.clone();
                let mut rn = |name:String,b:bool,loc:usize,raw:bool| { if !raw {self.run_label(name,b);}else {self.run_label_raw(loc)} };
                jump!(dest,labels,rn); 
            }
            JumpIfLess(dest) => {
                if *self.get_flag(LESS_THAN_FLAG).unwrap() == 0 { return; }
                let labels = self.labels.clone();
                let mut rn = |name:String,b:bool,loc:usize,raw:bool| { if !raw {self.run_label(name,b);}else {self.run_label_raw(loc)} };
                jump!(dest,labels,rn); 
            }
            Compare(dest) => {
                let (a,b) = get_destinations(dest);
                let reg_a = self.registers[a as usize];
                let reg_b = self.registers[b as usize];
                let _ = self.set_flag(EQUAL_FLAG,(reg_a==reg_b) as u8);
                let _ =self.set_flag(GREATER_THAN_FLAG,(reg_a>reg_b) as u8);
                let _ =self.set_flag(LESS_THAN_FLAG,(reg_a<reg_b) as u8);
            }

            GetFromStack(dest) => {
                let (reg,sp) = get_destinations(dest);
                if let Some(content) = self.stack.get(sp as usize) {
                    self.registers[reg as usize] = binary_slice_to_number!(RegisterDataType,content);
                }else {
                    panic!("Cannot get element number: {:?} from stack with total items: {:?}",sp,self.stack.len());
                }
            }
            GetFromStackPointer(dest) => {
                let (reg,offset) = get_destinations(dest);
                let sp = self.sp;
                let index = sp -1- offset as usize;
                if let Some(content) = self.stack.get(index) {
                    self.registers[reg as usize] = binary_slice_to_number!(RegisterDataType,content);
                }else {
                    panic!("Cannot get element number: {:?} from stack with total items: {:?}",sp,self.stack.len());
                }

            }

            Malloc(dest) => {
                let size = self.registers[dest as usize] as usize;
                // Check if any available memory with that size is available
                if let Some(available_memory) = self.is_memory_available(size) {
                    // all good, idk what to do here
                    self.stack.push(to_binary_slice!(RegisterDataType,available_memory.0).try_into().unwrap());
                    self.sp += 1;
                }else {
                    let p = self.allocate_memory(size);
                    self.stack.push(to_binary_slice!(RegisterDataType,p).try_into().unwrap());
                    self.sp += 1;
                }
            }

            GetMemory(dest) => {
                let (reg,loc) = get_destinations(dest);
                
                if let Some(val) = self.memory.get(loc as usize) {
                    self.registers[reg as usize] = val.unwrap_or(0);
                }else {
                    panic!("Unable to get location {:?} that is out of memory.",loc);
                }
            }

            SetMemory(dest) => {
                let (reg,loc) = get_destinations(dest);
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

            Mul(dest) => {
                let (a,b) = get_destinations(dest);
                self.registers[a as usize]*=self.registers[b as usize];

            } 
            Div(dest) => {
                let (a,b) = get_destinations(dest);
                self.registers[a as usize]/=self.registers[b as usize];

            }
            Mod(dest) => {
                let (a,b) = get_destinations(dest);
                self.registers[a as usize]%=self.registers[b as usize];

            } 
            Or(dest) => {
                let (a,b) = get_destinations(dest);
                self.registers[a as usize]|=self.registers[b as usize];

            }
            And(dest) => {
                let (a,b) = get_destinations(dest);
                self.registers[a as usize]&=self.registers[b as usize];

            } 
            Not(dest) => {
                todo!()
            }
            Xor(dest) => {
                todo!()
            } 
            Nand(dest) => todo!(), 
        
            TruncateStack(dest) => {
                let (a,_) = get_destinations(dest);
                let val = self.registers[a as usize];
                for _ in 0..val {
                    self.stack.pop();
                }
            }
            
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

    pub fn run_label(&mut self,label:String,is_main:bool) {
        if self.labels.get(&label).is_none() { panic!("Label with name {:?} doesnt exist.",label);}

        let (label_start,label_end_option)= &self.labels.get(&label).unwrap(); 

        if label_end_option.is_none() {
            panic!("No end found for label: {:?}",label);
        }

        let label_end = label_end_option.unwrap();
        if *label_start == label_end {
            panic!("Cannot define empty label");
        }
        // Push command pointer onto a buffer 
        if is_main {
            self.return_addresses.push(self.last_command);
        }else {
        self.return_addresses.push(self.command_pointer.into());
        }
        self.command_pointer = *label_start;
        while self.command_pointer < label_end-1 {
            self.run_current_command();
            self.command_pointer += 1;            
        }
        self.run_current_command(); // return cxommand'
    }

    pub fn run_label_raw(&mut self,label_loc:usize) {
             // Push command pointer onto a buffer 
        self.return_addresses.push(self.command_pointer.into());
        self.command_pointer = label_loc;
        while to_binary_slice!(CommandType,self.commands[self.command_pointer])[0..COMMAND_NAME_SIZE] != to_binary_slice!(DestinationType,21) {
            self.run_current_command();
            self.command_pointer += 1;            
        }
        
        self.run_current_command(); // return cxommand'

    }

    pub fn jump_raw(&mut self, loc:usize,_store_return:bool) {
        self.return_addresses.push(self.command_pointer);
        self.command_pointer = loc;
        self.run_current_command();
        self.command_pointer = self.return_addresses.pop().unwrap();
    }

    pub fn run_current_command(&mut self) {
        let command = self.commands[self.command_pointer];
        let command_byte = command_binary_to_slice(command);
        let command_name = binary_slice_to_number!(CommandNameType,&command_byte[0..COMMAND_NAME_SIZE]);
        let command_dest = binary_slice_to_number!(DestinationType,&command_byte[COMMAND_NAME_SIZE..]);
        if let Some(command) = validate_command(command_name.into(),command_dest.into()) {


            self.run_command(command);
        } else {
            panic!("Invalid Command");
        }
    }

    pub fn eval(&mut self) {
        self.run_label("main".to_string(),true);

    }

    pub fn add_command(&mut self, command:Command) {
        if self.last_command >= self.commands.len() {
            panic!("Maximum command length exceeded");
        }
        let code=  command_slice_to_binary(&command.to_binary_code());

    assert_eq!(command.to_binary_code().to_vec(),to_binary_slice!(CommandType ,code));
        self.commands[self.last_command] = code;
        self.last_command += 1;
    }

    pub fn add_command_raw(&mut self, code:CommandBinary) {
        if self.last_command >= self.commands.len() {
            panic!("Maximum command length exceeded");
        }
        self.commands[self.last_command] = binary_slice_to_number!(CommandType,code);
   
        self.last_command += 1;
    }

    pub fn add_instruction(&mut self, inst:Instruction) {
        let command = inst.to_command(&self.labels);
        self.add_command(command);
    }

    pub fn get_raw_byte_code(&mut self) -> Vec<[u8;COMMAND_SIZE]> {
        let mut fin = Vec::new();
        let mut i =0;
        while i <= self.last_command {
            let command = self.commands[i];
            let command_byte = command_binary_to_slice(command);
            let command_name = binary_slice_to_number!(CommandNameType,&command_byte[0..COMMAND_NAME_SIZE]);
            let command_dest = binary_slice_to_number!(DestinationType,&command_byte[COMMAND_NAME_SIZE..]);

            if let Some(command) = validate_command(command_name.into(),command_dest.into()) {
                fin.push(command.to_binary_code());  
            } else {
                panic!("Invalid Command");
            }
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
        if let Some((_start,end)) = self.labels.get_mut(&flag_name.to_string()) {
                if end.is_some() {
                    panic!("Unable to end label {:?} as it is already ended",flag_name);
                }   
                *end = Some(self.last_command);
                self.add_command(Command::Return);
        }else {
            panic!("Unable to end label as it does not exist: {:?}",flag_name);
        }
    }

    pub fn register_start(&mut self) {
        if let Some((_start,_)) = self.labels.get(&"main".to_string()) {
            let lc = self.last_command;
            self.last_command = 0;
            self.add_instruction(Instruction::Jump("main".to_string()));
            self.last_command = lc;
        }else {
            panic!("Unable to set main as main label does not exist.");
        }
    }

    pub fn write_to_file(&mut self,file_name:&str) -> std::io::Result<()> {
        use std::fs;
        let mut content  = "".to_string();
        let bytecode = self.get_raw_byte_code();
        for line in bytecode.iter() {
            for byte in line.iter() {
                content.push_str(&byte.to_string());
            }
            content.push_str("\n");
        }
        fs::write(file_name,content)?;
        Ok(())
    }

    pub fn read_from_file(&mut self, file_name:&str) -> std::io::Result<()> {
        use std::fs;
        let b = &fs::read(file_name)?;
        let content = String::from_utf8_lossy(b);
        self.last_command = 0;

        for line in content.lines() {
            let mut raw_command = Vec::new();
            for byte in line.chars() {
                if byte == '1' {
                    raw_command.push(1);
                }else if byte == '0' {
                    raw_command.push(0);
                }else {
                    panic!("Unrecognized character: {:?}",byte);
                }
            }
            self.add_command_raw(raw_command.try_into().unwrap());
        }
        // Setting up VM to add main label
        let command_slice = to_binary_slice!(CommandType,self.commands[0]);
        let main_start =binary_slice_to_number!(DestinationType,command_slice[COMMAND_NAME_SIZE..][0..DESTINATION_SIZE]);
        self.labels.insert("main".to_string(),(main_start as usize,Some(self.last_command)));
        Ok(())
    }

}

pub fn validate_command(command:CommandType,dest:DestinationType) -> Option<Command> {
    use Command::*;
    match command {
        0 => return Some(Halt),
        1 => return Some(Mov(dest)), 
        2 => return Some(Add(dest)),
        3 => return Some(Sub(dest)), 
        4 => return Some(Display(dest)),
        5 => return Some(Push(dest)),
        6 => return Some(PushRegister(dest)),
        7 => return Some(Pop(dest)),
        8 => return Some(Jump(dest)),
        9 => return Some(JumpIfZero(dest)),
        10 => return Some(JumpIfNotZero(dest)),
        11 => return Some(JumpIfEqual(dest)),
        12 => return Some(JumpIfNotEqual(dest)),
        13 => return Some(JumpIfGreater(dest)),
        14 => return Some(JumpIfLess(dest)),
        15 => return Some(Compare(dest)),
        16 => return Some(GetFromStack(dest)),
        17 => return Some(GetFromStackPointer(dest)),
        18 => return Some(Malloc(dest)),
        19 => return Some(GetMemory(dest)),
        20 => return Some(SetMemory(dest)),
        21 => return Some(Return),
        22 => return Some(Mul(dest)),
        23 => return Some(Div(dest)),
        24 => return Some(Or(dest)),
        25 => return Some(And(dest)),
        26 => return Some(Not(dest)),
        27 => return Some(Xor(dest)),
        28 => return Some(Nand(dest)),
        29 => return Some(TruncateStack(dest)),
        30 => return Some(Mod(dest)),
            _ => panic!("Unhandled command {:?}",command),
    }
}


