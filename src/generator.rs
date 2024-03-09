use crate::{
    parser::{NodeExpr,NodeLabel,NodeInstruction},
    instruction::Instruction,
    vm::VM,commands::*,
    constants_and_types::*, 
    tokens::*,
};

use std::collections::HashMap;

pub struct Generator {
    labels: HashMap<String,NodeLabel>,
    created_labels: Vec<String>,
    pub vm:VM,
}

impl Generator {
    pub fn new(labels:HashMap<String,NodeLabel>) -> Self {
        Self {
            labels,
            created_labels:Vec::new(),
            vm:VM::new()
        }
    }

    pub fn create_label(&mut self, name:String) {
        if self.created_labels.iter().find(|x| *x==&name).is_some() { return }
        if let Some(nl) = self.labels.get(&name) {
            self.vm.start_label(name.as_str());
            if self.generate_program(nl.insts.clone()) {
                self.vm.add_command(Command::Return);
            } 
            self.vm.end_label(name.as_str());
            self.created_labels.push(name);
        }else {
            panic!("Could'nt create label with name: {:?} as it does not exist.",name);
        }
    }
   

    pub fn generate(&mut self) {
        self.create_all_labels();
        self.create_label("main".to_string());
        self.vm.register_start();
    }


    pub fn create_all_labels(&mut self) {
        let labels = self.labels.get("main").cloned();
        if let Some(nl) = labels {
            for inst in nl.insts.iter() {
                use NodeInstruction::*;
                match inst {
                    NodeInstructionJump {value} => self.create_label(get_jump_label(value.clone()).unwrap()),
                    NodeInstructionJumpIfZero {value} => self.create_label(get_jump_label(value.clone()).unwrap()),
                    NodeInstructionJumpIfNotZero {value} =>self.create_label(get_jump_label(value.clone()).unwrap()),
                    NodeInstructionJumpIfEqual {value} => self.create_label(get_jump_label(value.clone()).unwrap()),
                    NodeInstructionJumpIfNotEqual {value} => self.create_label(get_jump_label(value.clone()).unwrap()),

                    NodeInstructionJumpIfLess {value} => self.create_label(get_jump_label(value.clone()).unwrap()),
                    NodeInstructionJumpIfGreater {value} => self.create_label(get_jump_label(value.clone()).unwrap()),
                    _ => {}
                }
            }
        }
    }

    pub fn generate_program(&mut self,nodes:Vec<NodeInstruction>) -> bool {

        use NodeInstruction::*;
        for node in nodes.iter() {
            match node {
                NodeInstructionHalt => self.vm.add_instruction(Instruction::Halt),
                NodeInstructionMov {lhs,rhs} => {
                    let reg = get_register(lhs);
                    match rhs  {
                        NodeExpr::NodeExprIntLit{value} => self.vm.add_instruction(Instruction::Mov(reg,value.value.clone().unwrap().parse::<InstructionParamType>().unwrap())),
                        NodeExpr::NodeExprRegister{value} => {
                            let reg2 = get_register(rhs);
                            self.vm.add_instruction(Instruction::PushRegister(reg2));
                            self.vm.add_instruction(Instruction::Pop(reg));
                        } 
                        _ => unreachable!()
                    }
                }
                NodeInstructionDisplay { value } => {
                    match value {
                        NodeExpr::NodeExprRegister { value } => {
                            if let Some(register) = get_register_value(value.clone()) {
                                self.vm.add_instruction(Instruction::Display(register));
                            }else {
                                panic!("Register {:?} does not exist.",value.value.clone().unwrap());
                            }
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                            let int = value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(REGA));
                            self.vm.add_instruction(Instruction::Mov(REGA,int));
                            self.vm.add_instruction(Instruction::Display(REGA));
                            self.vm.add_instruction(Instruction::Pop(REGA));
                        },
                        _ => unreachable!()
                    }
                }

                NodeInstructionAdd { lhs, rhs } => {
                    let reg = get_register(lhs);
                    match rhs  {
                        NodeExpr::NodeExprIntLit{value} => {
                            let int = value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(reg+1));
                            self.vm.add_instruction(Instruction::Mov(reg+1,int));
                            self.vm.add_instruction(Instruction::Add(reg,reg+1));
                            self.vm.add_instruction(Instruction::Pop(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            self.vm.add_instruction(Instruction::Add(reg,get_register(rhs))); 
                        }
                        _ => unreachable!()
                    }
                }
                NodeInstructionSub { lhs, rhs } => {
                    let reg = get_register(lhs);
                    match rhs  {
                        NodeExpr::NodeExprIntLit{value} => {
                            let int = value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(reg+1));
                            self.vm.add_instruction(Instruction::Mov(reg+1,int));
                            self.vm.add_instruction(Instruction::Sub(reg,reg+1));
                            self.vm.add_instruction(Instruction::Pop(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            let reg2= get_register(rhs); 
                            self.vm.add_instruction(Instruction::Sub(reg,reg2));

                        }
                        _ => unreachable!()
                    }
                }

                NodeInstructionMod { lhs, rhs } => {
                    let reg = get_register(lhs);
                    match rhs  {
                        NodeExpr::NodeExprIntLit{value} => {
                            let int = value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(reg+1));
                            self.vm.add_instruction(Instruction::Mov(reg+1,int));
                            self.vm.add_instruction(Instruction::Mod(reg,reg+1));
                            self.vm.add_instruction(Instruction::Pop(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            let reg2= get_register(rhs); 
                            self.vm.add_instruction(Instruction::Mod(reg,reg2));

                        }
                        _ => unreachable!()
                    }
                }

                NodeInstructionPush{value}=>{ 
                    match value {
                        NodeExpr::NodeExprIntLit { value } => {
                            let int = value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::Push(int));
                        }
                        NodeExpr::NodeExprRegister { value:_value } => {
                            let reg = get_register(value);
                            self.vm.add_instruction(Instruction::PushRegister(reg));
                        }
                        _ => unreachable!()
                    }
                },
                NodeInstructionPop{value}=>{
                    let reg = get_register(value);
                    self.vm.add_instruction(Instruction::Pop(reg));
                },

                NodeInstructionJump{value} => {
                    let label_name=  get_jump_label(value.clone()).unwrap();
                    self.vm.add_instruction(Instruction::Jump(label_name));
                },
                NodeInstructionJumpIfZero{value} => {
                    let label_name=  get_jump_label(value.clone()).unwrap();
                    self.vm.add_instruction(Instruction::JumpIfZero(label_name));
                },
                NodeInstructionJumpIfNotZero{value} => {
                    let label_name=  get_jump_label(value.clone()).unwrap();
                    self.vm.add_instruction(Instruction::JumpIfNotZero(label_name));
                },
                NodeInstructionJumpIfEqual{value} => {
                    let label_name=  get_jump_label(value.clone()).unwrap();
                    self.vm.add_instruction(Instruction::JumpIfEqual(label_name));
                },
                NodeInstructionJumpIfNotEqual{value} => {
                    let label_name=  get_jump_label(value.clone()).unwrap();
                    self.vm.add_instruction(Instruction::JumpIfNotEqual(label_name));
                },
                NodeInstructionJumpIfGreater{value} => {   
                    let label_name=  get_jump_label(value.clone()).unwrap();
                    self.vm.add_instruction(Instruction::JumpIfGreater(label_name));
                },
                NodeInstructionJumpIfLess{value} => {
                    let label_name=  get_jump_label(value.clone()).unwrap();
                    self.vm.add_instruction(Instruction::JumpIfLess(label_name));
                },

                NodeInstructionCompare{lhs, rhs} => {
                    let mut lreg_store = false;
                    let mut rreg_store = false;
                   let lreg = match lhs {
                        NodeExpr::NodeExprRegister { value:_value } => {
                            get_register(lhs)
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                           let val =  value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
                           self.vm.add_instruction(Instruction::PushRegister(REGC));
                            self.vm.add_instruction(Instruction::Mov(REGC,val));
                            lreg_store = true;
                            REGC
                        },
                        _ => unreachable!()
                    }; 

                    let rreg = match rhs {
                        NodeExpr::NodeExprRegister { value: _} => {
                            get_register(rhs)
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                            let val = value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(REGD));
                            self.vm.add_instruction(Instruction::Mov(REGD,val));
                            rreg_store = true;
                            REGD
                        }
                        _ => unreachable!()
                    };
                    self.vm.add_instruction(Instruction::Compare(lreg,rreg));
                    if rreg_store {
                        self.vm.add_instruction(Instruction::Pop(REGD));
                    }
                    if lreg_store {
                        self.vm.add_instruction(Instruction::Pop(REGC));
                    }
                },

                NodeInstructionGetFromStack{lhs, rhs} => {
                    let lreg = get_register(lhs);
                    match rhs {
                        NodeExpr::NodeExprRegister { value: _} => {
                            self.vm.add_instruction(Instruction::GetFromStack(lreg,get_register(rhs)));
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                            let val = value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(lreg+1));
                            self.vm.add_instruction(Instruction::Mov(lreg+1,val));
                            self.vm.add_instruction(Instruction::GetFromStack(lreg,lreg+1));
                            self.vm.add_instruction(Instruction::Pop(lreg+1));
                        }
                        _ => unreachable!()
                    };

                    
                },
                NodeInstructionGetFromStackPointer{lhs, rhs} => {
                    let lreg = get_register(lhs);
                    match rhs {
                        NodeExpr::NodeExprRegister { value: _} => {
                            self.vm.add_instruction(Instruction::GetFromStackPointer(lreg,get_register(rhs)));
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                            let val = value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(lreg+1));
                            self.vm.add_instruction(Instruction::Mov(lreg+1,val));
                            self.vm.add_instruction(Instruction::GetFromStackPointer(lreg,lreg+1));
                            self.vm.add_instruction(Instruction::Pop(lreg+1));
                        }
                        _ => unreachable!()
                    };
                },

                NodeInstructionTruncateStack { value } => {
                    match value {
                        NodeExpr::NodeExprIntLit { value } => {
                            let val = value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::Mov(RESERVEREGISTER,val));
                            self.vm.add_instruction(Instruction::TruncateStack(RESERVEREGISTER));
                        }
                        NodeExpr::NodeExprRegister { value:_ } => {
                            self.vm.add_instruction(Instruction::TruncateStack(get_register(value)));
                        }
                        _ => unreachable!()
                    }
                }

                NodeInstructionMalloc{value} =>{
                    match value {
                        NodeExpr::NodeExprRegister { value } => {
                            if let Some(register) = get_register_value(value.clone()) {
                                self.vm.add_instruction(Instruction::Malloc(register));
                            }else {
                                panic!("Register {:?} does not exist.",value.value.clone().unwrap());
                            }
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                            let int = value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::Mov(REGA,int));
                            self.vm.add_instruction(Instruction::Malloc(REGA));
                            self.vm.add_instruction(Instruction::Pop(REGA)); 
                        },
                        _ => unreachable!()
                    }

                },
                NodeInstructionGetMemory{lhs, rhs} => {
                    let lreg = get_register(lhs);
                    match rhs {
                        NodeExpr::NodeExprRegister { value: _} => {
                            self.vm.add_instruction(Instruction::GetMemory(lreg,get_register(rhs)));
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                            let val = value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(lreg+1));
                            self.vm.add_instruction(Instruction::Mov(lreg+1,val));
                            self.vm.add_instruction(Instruction::GetMemory(lreg,lreg+1));
                            self.vm.add_instruction(Instruction::Pop(lreg+1));
                        }
                        _ => unreachable!()
                    };

                },
                NodeInstructionSetMemory{lhs, rhs} => {
                    let lreg = get_register(lhs);
                    match rhs {
                        NodeExpr::NodeExprRegister { value: _} => {
                            self.vm.add_instruction(Instruction::SetMemory(lreg,get_register(rhs)));
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                            let val = value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(lreg+1));
                            self.vm.add_instruction(Instruction::Mov(lreg+1,val));
                            self.vm.add_instruction(Instruction::SetMemory(lreg,lreg+1));
                            self.vm.add_instruction(Instruction::Pop(lreg+1));
                        }
                        _ => unreachable!()
                    };

                },

                NodeInstructionReturn=>{ return true},
            
              NodeInstructionMul { lhs, rhs } => {
                    let reg = get_register(lhs);
                    match rhs  {
                        NodeExpr::NodeExprIntLit{value} => {
                            let int = value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(reg+1));
                            self.vm.add_instruction(Instruction::Mov(reg+1,int));
                            self.vm.add_instruction(Instruction::Mul(reg,reg+1));
                            self.vm.add_instruction(Instruction::Pop(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            self.vm.add_instruction(Instruction::Mul(reg,get_register(rhs))); 
                        }
                        _ => unreachable!()
                    }
                }
                NodeInstructionDiv { lhs, rhs } => {
                    let reg = get_register(lhs);
                    match rhs  {
                        NodeExpr::NodeExprIntLit{value} => {
                            let int = value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(reg+1));
                            self.vm.add_instruction(Instruction::Mov(reg+1,int));
                            self.vm.add_instruction(Instruction::Div(reg,reg+1));
                            self.vm.add_instruction(Instruction::Pop(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            let reg2= get_register(rhs); 
                            self.vm.add_instruction(Instruction::Div(reg,reg2));

                        }
                        _ => unreachable!()
                    }
                }
                
                NodeInstructionNot{value} => {
                    let reg = get_register(value);
                    self.vm.add_instruction(Instruction::Not(reg));
                }
                NodeInstructionAnd { lhs, rhs } => {
                    let reg = get_register(lhs);
                    match rhs  {
                        NodeExpr::NodeExprIntLit{value} => {
                            let int = value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(reg+1));
                            self.vm.add_instruction(Instruction::Mov(reg+1,int));
                            self.vm.add_instruction(Instruction::And(reg,reg+1));
                            self.vm.add_instruction(Instruction::Pop(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            self.vm.add_instruction(Instruction::And(reg,get_register(rhs))); 
                        }
                        _ => unreachable!()
                    }
                }
                NodeInstructionOr { lhs, rhs } => {
                    let reg = get_register(lhs);
                    match rhs  {
                        NodeExpr::NodeExprIntLit{value} => {
                            let int = value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(reg+1));
                            self.vm.add_instruction(Instruction::Mov(reg+1,int));
                            self.vm.add_instruction(Instruction::Or(reg,reg+1));
                            self.vm.add_instruction(Instruction::Pop(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            let reg2= get_register(rhs); 
                            self.vm.add_instruction(Instruction::Or(reg,reg2));

                        }
                        _ => unreachable!()
                    }
                }
  NodeInstructionXor { lhs, rhs } => {
                    let reg = get_register(lhs);
                    match rhs  {
                        NodeExpr::NodeExprIntLit{value} => {
                            let int = value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(reg+1));
                            self.vm.add_instruction(Instruction::Mov(reg+1,int));
                            self.vm.add_instruction(Instruction::Xor(reg,reg+1));
                            self.vm.add_instruction(Instruction::Pop(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            self.vm.add_instruction(Instruction::Xor(reg,get_register(rhs))); 
                        }
                        _ => unreachable!()
                    }
                }
                NodeInstructionNand { lhs, rhs } => {
                    let reg = get_register(lhs);
                    match rhs  {
                        NodeExpr::NodeExprIntLit{value} => {
                            let int = value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(reg+1));
                            self.vm.add_instruction(Instruction::Mov(reg+1,int));
                            self.vm.add_instruction(Instruction::Nand(reg,reg+1));
                            self.vm.add_instruction(Instruction::Pop(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            let reg2= get_register(rhs); 
                            self.vm.add_instruction(Instruction::Nand(reg,reg2));

                        }
                        _ => unreachable!()
                    }
                }

      
             

            }
        }
        false
    }  

    pub fn write_to_file(&mut self, s:String) -> std::io::Result<()> {
        self.vm.write_to_file(&s)
    }

    pub fn read_from_file(&mut self, file_name:String) -> std::io::Result<()> {
        self.vm.read_from_file(&file_name)
    }
    

}

pub fn get_register_value(reg:Token) -> Option<RegisterDataType> {
    if reg.token_type != TokenType::Register { return None };
    if let Some(val) = reg.value {
        let val = match val.as_str() {
            "rax" => REGA,
            "rbx" => REGB,
            "rcx" => REGC,
            "rdx" => REGD,
            _ => unreachable!()
        };
        return Some(val)
    }
    None
}

pub fn get_jump_label(value:NodeExpr) -> Option<String> {
    match value {
        NodeExpr::NodeExprLabelName { value } => { 
            return value.value;
        },
        _ => unreachable!()
    }
}

pub fn get_register(value:&NodeExpr) -> RegisterDataType {
    match &value  {
        NodeExpr::NodeExprRegister { value } => {
            if let Some(register) = get_register_value(value.clone()) {
                return register
            }else {
                panic!("Register {:?} does not exist.",value.value.clone().unwrap());
            }
        }
        _ => panic!("Internal Error. Expected Register found integer while moving. (1st argument)."),
    };
}
