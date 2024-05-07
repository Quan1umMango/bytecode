use crate::{
    parser::{NodeExpr,NodeInstruction,NodeBuiltin,Parser},
    instruction::Instruction,
    constants_and_types::*, 
    tokens::*,
    vm::VM,
};

use std::collections::HashMap;

pub struct Generator {
    node_instructions: Vec<NodeInstruction>,
    labels: HashMap<String,usize>,
    builtins: Vec<NodeBuiltin>,
    pub vm:VM,
}

impl Generator {
    pub fn new(builtins:Vec<NodeBuiltin>,labels:HashMap<String,usize>,node_instructions:Vec<NodeInstruction>) -> Self {
        Self {
            labels,
            vm:VM::new(),
            builtins,
            node_instructions,
        }
    }

    // NOTE: Make this code better lol.
    pub fn generate(&mut self,no_main:bool) {
        let external_labels: Vec<(Vec<(String,usize)>,NodeInstruction)> = self.generate_imports();
        let mut new_instructions: Vec<(Vec<String>,NodeInstruction)> = Vec::new();
        // We first push the external instructions into new_instructions and attach any label
        // identifier they have.
        for i in 0..external_labels.len() {
            let cur = external_labels[i].clone();
            if cur.0.is_empty() {
                new_instructions.push((Vec::new(),cur.1));
            }else {
                let v = cur.0.iter().map(|(x,_)| x.as_str().to_string()).collect::<Vec<String>>(); 
                new_instructions.push((v,cur.1));
            }
        }

        let l =new_instructions.len();
        
        for inst in self.node_instructions.iter() {
            new_instructions.push((Vec::new(),inst.clone()));
        }
        // We search if there are any labels in our main file then attach them onto the
        // new_instructions 
        for (name,start) in self.labels.iter() {
            new_instructions[*start+l-1].0.push(name.clone());
        }

        self.generate_instructions(new_instructions);
        if !no_main {

            self.vm.register_start();
        }
    }


    pub fn generate_imports(&mut self) -> Vec<(Vec<(String,usize)>,NodeInstruction)> {
        let mut out = Vec::new();
        for builtin in self.builtins.iter() {
            match builtin {
                NodeBuiltin::NodeBuiltinImport { value } => {
                    use std::fs;
                    
                    let file_loc = match value {
                        NodeExpr::NodeExprStringLit { value } => value.value.clone().unwrap(),
                        _ => unreachable!()
                    };
                    let file = match fs::read_to_string(file_loc.as_str()) {
                        Ok(f) => f,
                        Err(e) => {
                            println!("Builtin function error: @Import({:?}) failed. {:?}",file_loc,e);
                            std::process::exit(1);
                        }
                    };
                    let tokens = Tokenizer::new(file).tokenize();
                    let mut parsed = Parser::new(tokens);
                    parsed.parse();
                    'outer: for i in 1..parsed.instructions.len()+1 {
                        let mut cur_labels = Vec::new();
                        for (label_name,start) in parsed.labels.iter() {
                            if *start == i && *label_name !="main".to_string(){
                                cur_labels.push((label_name.clone(),*start));
                              // out.push((Some((label_name.clone(),*start)),parsed.instructions[i-1].clone()));
                               
                            }
                            if cur_labels.len() != 0 {
                                out.push((cur_labels,parsed.instructions[i-1].clone()));
                                continue 'outer;
                            } 
                        }
                        out.push((Vec::new(),parsed.instructions[i-1].clone()));
                    }

                }

                _ => () 
            }
        }
        out
    }


    pub fn generate_instructions(&mut self,insts:Vec<(Vec<String>,NodeInstruction)>){

        use NodeInstruction::*;
        for (label_vec,node) in insts.iter() {
            if !label_vec.is_empty() {
                for label_name in label_vec.iter() {
                    self.vm.create_label(self.vm.last_command(),&label_name.clone());
                }
            }
            match node {
                NodeInstructionHalt => self.vm.add_instruction(Instruction::Halt),
                NodeInstructionMov {lhs,rhs} => {
                    let reg = get_register(&lhs);
                    match rhs  {
                        NodeExpr::NodeExprIntLit{value} => self.vm.add_instruction(Instruction::Mov(reg as InstructionParamType,value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap())),
                        NodeExpr::NodeExprRegister{value:_} => {
                            let reg2 = get_register(&rhs);
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
                            self.vm.add_instruction(Instruction::DisplayValue(value.value.clone().unwrap().parse::<InstructionParamType>().unwrap())); 
                        },
                        _ => unreachable!()
                    }
                }
NodeInstructionDisplayf { value } => {
                    match value {
                        NodeExpr::NodeExprRegister { value } => {
                            if let Some(register) = get_fregister_value(value.clone()) {
                                self.vm.add_instruction(Instruction::Displayf(register));
                            }else {
                                panic!("Register {:?} does not exist.",value.value.clone().unwrap());
                            }
                        }
                        NodeExpr::NodeExprFloat { value } => {
                            let val = value.value.clone().unwrap().parse::<FloatInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushFloatRegister(0));
                            self.vm.add_instruction(Instruction::Movf(0,val));
                            self.vm.add_instruction(Instruction::Displayf(0));
                            self.vm.add_instruction(Instruction::PopFloat(0));
                        },
                        _ => unreachable!()
                    }
                }

NodeInstructionDisplayChar { value } => {
                    match value {
                        NodeExpr::NodeExprRegister { value } => {
                            if let Some(register) = get_register_value(value.clone()) {
                                self.vm.add_instruction(Instruction::DisplayChar(register));
                            }else {
                                panic!("Register {:?} does not exist.",value.value.clone().unwrap());
                            }
                        }
                        NodeExpr::NodeExprFloat { value } => {
                            let val = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(0));
                            self.vm.add_instruction(Instruction::Mov(0,val));
                            self.vm.add_instruction(Instruction::DisplayChar(0));
                            self.vm.add_instruction(Instruction::Pop(0));
                        },
                        _ => unreachable!()
                    }
                }



                NodeInstructionAdd { lhs, rhs } => {
                    let reg = get_register(&lhs);
                    match rhs  {
                        NodeExpr::NodeExprIntLit{value} => {
                            let int = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(reg+1));
                            self.vm.add_instruction(Instruction::Mov(reg+1,int));
                            self.vm.add_instruction(Instruction::Add(reg,reg+1));
                            self.vm.add_instruction(Instruction::Pop(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            self.vm.add_instruction(Instruction::Add(reg,get_register(&rhs))); 
                        }
                        _ => unreachable!()
                    }
                }
                NodeInstructionSub { lhs, rhs } => {
                    let reg = get_register(&lhs);
                    match rhs  {

                        NodeExpr::NodeExprIntLit{value} => {
                            let int = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(reg+1));
                            self.vm.add_instruction(Instruction::Mov(reg+1,int));
                            self.vm.add_instruction(Instruction::Sub(reg,reg+1));
                            self.vm.add_instruction(Instruction::Pop(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            let reg2= get_register(&rhs); 
                            self.vm.add_instruction(Instruction::Sub(reg,reg2));

                        }
                        _ => unreachable!()
                    }
                }

                NodeInstructionMod { lhs, rhs } => {
                    let reg = get_register(&lhs);
                    match rhs  {
                        NodeExpr::NodeExprIntLit{value} => {
                            let int = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(reg+1));
                            self.vm.add_instruction(Instruction::Mov(reg+1,int));
                            self.vm.add_instruction(Instruction::Mod(reg,reg+1));
                            self.vm.add_instruction(Instruction::Pop(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            let reg2= get_register(&rhs); 
                            self.vm.add_instruction(Instruction::Mod(reg,reg2));

                        }
                        _ => unreachable!()
                    }
                }

                NodeInstructionPush{value}=>{ 
                    match value {
                        NodeExpr::NodeExprIntLit { value } => {
                            let int = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
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
                    match value {
                        NodeExpr::NodeExprIntLit{value} => {
                            let inst_address = value.value.as_ref().unwrap().parse::<u32>().unwrap();
                            self.vm.add_instruction(Instruction::Jump(crate::instruction::StringNumberUnion::Num(inst_address)));
                        }
                        NodeExpr::NodeExprLabelName{value:_v} => {
                            let label_name=  get_jump_label(value.clone()).unwrap();
                            self.vm.add_instruction(Instruction::Jump(crate::instruction::StringNumberUnion::String(label_name)));
                        }
                        _ => unreachable!()
                    }
                },
                NodeInstructionJumpIfZero{value} => {
                    match value {
                        NodeExpr::NodeExprIntLit{value} => {
                            let inst_address = value.value.as_ref().unwrap().parse::<u32>().unwrap();
                            self.vm.add_instruction(Instruction::JumpIfZero(crate::instruction::StringNumberUnion::Num(inst_address)));
                        }
                        NodeExpr::NodeExprLabelName{value:_v} => {
                            let label_name=  get_jump_label(value.clone()).unwrap();
                            self.vm.add_instruction(Instruction::JumpIfZero(crate::instruction::StringNumberUnion::String(label_name)));
                        }
                        _ => unreachable!()
                    }                },
                NodeInstructionJumpIfNotZero{value} => {
                    match value {
                        NodeExpr::NodeExprIntLit{value} => {
                            let inst_address = value.value.as_ref().unwrap().parse::<u32>().unwrap();
                            self.vm.add_instruction(Instruction::JumpIfNotZero(crate::instruction::StringNumberUnion::Num(inst_address)));
                        }
                        NodeExpr::NodeExprLabelName{value:_v} => {
                            let label_name=  get_jump_label(value.clone()).unwrap();
                            self.vm.add_instruction(Instruction::JumpIfNotZero(crate::instruction::StringNumberUnion::String(label_name)));
                        }
                        _ => unreachable!()
                    }                },
                NodeInstructionJumpIfEqual{value} => {
                    match value {
                        NodeExpr::NodeExprIntLit{value} => {
                            let inst_address = value.value.as_ref().unwrap().parse::<u32>().unwrap();
                            self.vm.add_instruction(Instruction::JumpIfEqual(crate::instruction::StringNumberUnion::Num(inst_address)));
                        }
                        NodeExpr::NodeExprLabelName{value:_v} => {
                            let label_name=  get_jump_label(value.clone()).unwrap();
                            self.vm.add_instruction(Instruction::JumpIfEqual(crate::instruction::StringNumberUnion::String(label_name)));
                        }
                        _ => unreachable!()
                    }                },
                NodeInstructionJumpIfNotEqual{value} => {
                    match value {
                        NodeExpr::NodeExprIntLit{value} => {
                            let inst_address = value.value.as_ref().unwrap().parse::<u32>().unwrap();
                            self.vm.add_instruction(Instruction::JumpIfNotEqual(crate::instruction::StringNumberUnion::Num(inst_address)));
                        }
                        NodeExpr::NodeExprLabelName{value:_v} => {
                            let label_name=  get_jump_label(value.clone()).unwrap();
                            self.vm.add_instruction(Instruction::JumpIfNotEqual(crate::instruction::StringNumberUnion::String(label_name)));
                        }
                        _ => unreachable!()
                    }
                },
                NodeInstructionJumpIfGreater{value} => {   
                    match value {
                        NodeExpr::NodeExprIntLit{value} => {
                            let inst_address = value.value.as_ref().unwrap().parse::<u32>().unwrap();
                            self.vm.add_instruction(Instruction::JumpIfGreater(crate::instruction::StringNumberUnion::Num(inst_address)));
                        }
                        NodeExpr::NodeExprLabelName{value:_v} => {
                            let label_name=  get_jump_label(value.clone()).unwrap();
                            self.vm.add_instruction(Instruction::JumpIfGreater(crate::instruction::StringNumberUnion::String(label_name)));
                        }
                        _ => unreachable!()
                    }                },
                NodeInstructionJumpIfLess{value} => {
                    match value {
                        NodeExpr::NodeExprIntLit{value} => {
                            let inst_address = value.value.as_ref().unwrap().parse::<u32>().unwrap();
                            self.vm.add_instruction(Instruction::JumpIfLess(crate::instruction::StringNumberUnion::Num(inst_address)));
                        }
                        NodeExpr::NodeExprLabelName{value:_v} => {
                            let label_name=  get_jump_label(value.clone()).unwrap();
                            self.vm.add_instruction(Instruction::JumpIfLess(crate::instruction::StringNumberUnion::String(label_name)));
                        }
                        _ => unreachable!()
                    }              
                },

                NodeInstructionCompare{lhs, rhs} => {
                    let mut lreg_store = false;
                    let mut rreg_store = false;
                    let lreg = match lhs {
                        NodeExpr::NodeExprRegister { value:_value } => {
                            get_register(&lhs)
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                            let val =  value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(REGC));
                            self.vm.add_instruction(Instruction::Mov(REGC,val));
                            lreg_store = true;
                            REGC
                        },
                        _ => unreachable!()
                    }; 

                    let rreg = match rhs {
                        NodeExpr::NodeExprRegister { value: _} => {
                            get_register(&rhs)
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                            let val = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
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
                    let dest = get_register(&rhs);
                    match lhs {
                        NodeExpr::NodeExprRegister { value: _} => {
                            self.vm.add_instruction(Instruction::GetFromStack(get_register(&lhs),dest));
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                            let val = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(dest+1));
                            self.vm.add_instruction(Instruction::Mov(dest+1,val));
                            self.vm.add_instruction(Instruction::GetFromStack(dest+1,dest));
                            self.vm.add_instruction(Instruction::Pop(dest+1));
                        }
                        _ => unreachable!()
                    };
                },
                NodeInstructionGetFromStackPointer{lhs, rhs} => {
                    let dest = get_register(&rhs);
                    match lhs {
                        NodeExpr::NodeExprRegister { value: _} => {
                            self.vm.add_instruction(Instruction::GetFromStackPointer(get_register(&lhs),dest));
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                            let val = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(dest+1));
                            self.vm.add_instruction(Instruction::Mov(dest+1,val+1));
                            self.vm.add_instruction(Instruction::GetFromStackPointer(dest+1,dest));
                            self.vm.add_instruction(Instruction::Pop(dest+1));
                        }
                        _ => unreachable!()
                    };
                },
                
                NodeInstructionSetStack { lhs, rhs } => {
                    let dest = get_register(&rhs);
                    match lhs {
                        NodeExpr::NodeExprRegister { value: _} => {
                            self.vm.add_instruction(Instruction::SetStack(get_register(&lhs),dest));
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                            let int = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();                  
                            self.vm.add_instruction(Instruction::PushRegister(dest+1));
                            self.vm.add_instruction(Instruction::Mov(dest+1,int));
                            self.vm.add_instruction(Instruction::SetStack(dest+1,dest));
                            self.vm.add_instruction(Instruction::Pop(dest+1));
                        }
                        _ => unreachable!()
                    };

                }

                NodeInstructionSetFromStackPointer { lhs, rhs } => {
                    let dest = get_register(&rhs);
                    match lhs {
                        NodeExpr::NodeExprRegister { value: _} => {
                            self.vm.add_instruction(Instruction::SetFromStackPointer(get_register(&lhs),dest));
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                            let int = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();                  
                            self.vm.add_instruction(Instruction::PushRegister(dest+1));
                            self.vm.add_instruction(Instruction::Mov(dest+1,int+1));
                            self.vm.add_instruction(Instruction::SetFromStackPointer(dest+1,dest));
                            self.vm.add_instruction(Instruction::Pop(dest+1));
                        }
                        _ => unreachable!()
                    };

                }

                NodeInstructionTruncateStack { value } => {
                    match value {
                        NodeExpr::NodeExprIntLit { value } => {
                            let val = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::Mov(RESERVEREGISTER1,val));
                            self.vm.add_instruction(Instruction::TruncateStack(RESERVEREGISTER1));
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
                            let int = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::Mov(RESERVEREGISTER1,int));
                            self.vm.add_instruction(Instruction::Malloc(RESERVEREGISTER1));
                        },
                        _ => unreachable!()
                    }

                }
                NodeInstructionFree{value} =>{
                    match value {
                        NodeExpr::NodeExprRegister { value } => {
                            if let Some(register) = get_register_value(value.clone()) {
                                self.vm.add_instruction(Instruction::Free(register));
                            }else {
                                panic!("Register {:?} does not exist.",value.value.clone().unwrap());
                            }
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                            let int = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::Mov(RESERVEREGISTER1,int));
                            self.vm.add_instruction(Instruction::Free(RESERVEREGISTER1));
                        },
                        _ => unreachable!()
                    }

                }
                NodeInstructionGetMemory{lhs, rhs,offset} => {
                    let lreg = get_register(&lhs);
                    let rreg = { 
                        match rhs {
                            NodeExpr::NodeExprRegister { value: _} => {
                                get_register(&rhs)
                                //self.vm.add_instruction(Instruction::GetMemory(lreg,get_register(&rhs)));
                            }
                            NodeExpr::NodeExprIntLit { value } => {
                                let val = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                                self.vm.add_instruction(Instruction::Mov(RESERVEREGISTER1,val));
                                //self.vm.add_instruction(Instruction::GetMemory(lreg,lreg+1));
                                //self.vm.add_instruction(Instruction::Pop(lreg+1));
                                RESERVEREGISTER1
                            }
                            _ => unreachable!()
                        }
                    };
                    match offset {
                        NodeExpr::NodeExprRegister { value: _ } => {
                            self.vm.add_instruction(Instruction::GetMemory(lreg,rreg,get_register(&offset)));
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                                let val = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                                self.vm.add_instruction(Instruction::Mov(RESERVEREGISTER2,val));
                                self.vm.add_instruction(Instruction::GetMemory(lreg,rreg,RESERVEREGISTER2)); 
                        }
                        _ => unreachable!()
                    }
                },
                NodeInstructionSetMemory{lhs, rhs,offset} => {
                    let lreg = get_register(&lhs);
                    let rreg = { 
                        match rhs {
                            NodeExpr::NodeExprRegister { value: _} => {
                                get_register(&rhs)
                                //self.vm.add_instruction(Instruction::GetMemory(lreg,get_register(&rhs)));
                            }
                            NodeExpr::NodeExprIntLit { value } => {
                                let val = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                                self.vm.add_instruction(Instruction::Mov(RESERVEREGISTER1,val));
                                //self.vm.add_instruction(Instruction::SetMemory(lreg,lreg+1));
                                //self.vm.add_instruction(Instruction::Pop(lreg+1));
                                RESERVEREGISTER1
                            }
                            _ => unreachable!()
                        }
                    };
                    match offset {
                        NodeExpr::NodeExprRegister { value: _ } => {
                            self.vm.add_instruction(Instruction::SetMemory(lreg,rreg,get_register(&offset)));
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                                let val = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                                self.vm.add_instruction(Instruction::Mov(RESERVEREGISTER2,val));
                                self.vm.add_instruction(Instruction::SetMemory(lreg,rreg,RESERVEREGISTER2)); 
                        }
                        _ => unreachable!()
                    }                },

                NodeInstructionReturn=>{ self.vm.add_instruction(Instruction::Return); },

                NodeInstructionMul { lhs, rhs } => {
                    let reg = get_register(&lhs);
                    match rhs  {
                        NodeExpr::NodeExprIntLit{value} => {
                            let int = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(reg+1));
                            self.vm.add_instruction(Instruction::Mov(reg+1,int));
                            self.vm.add_instruction(Instruction::Mul(reg,reg+1));
                            self.vm.add_instruction(Instruction::Pop(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            self.vm.add_instruction(Instruction::Mul(reg,get_register(&rhs))); 
                        }
                        _ => unreachable!()
                    }
                }
                NodeInstructionDiv { lhs, rhs } => {
                    let reg = get_register(&lhs);
                    match rhs  {
                        NodeExpr::NodeExprIntLit{value} => {
                            let int = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(reg+1));
                            self.vm.add_instruction(Instruction::Mov(reg+1,int));
                            self.vm.add_instruction(Instruction::Div(reg,reg+1));
                            self.vm.add_instruction(Instruction::Pop(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            let reg2= get_register(&rhs); 
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
                    let reg = get_register(&lhs);
                    match rhs  {
                        NodeExpr::NodeExprIntLit{value} => {
                            let int = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(reg+1));
                            self.vm.add_instruction(Instruction::Mov(reg+1,int));
                            self.vm.add_instruction(Instruction::And(reg,reg+1));
                            self.vm.add_instruction(Instruction::Pop(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            self.vm.add_instruction(Instruction::And(reg,get_register(&rhs))); 
                        }
                        _ => unreachable!()
                    }
                }
                NodeInstructionOr { lhs, rhs } => {
                    let reg = get_register(&lhs);
                    match rhs  {
                        NodeExpr::NodeExprIntLit{value} => {
                            let int = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(reg+1));
                            self.vm.add_instruction(Instruction::Mov(reg+1,int));
                            self.vm.add_instruction(Instruction::Or(reg,reg+1));
                            self.vm.add_instruction(Instruction::Pop(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            let reg2= get_register(&rhs); 
                            self.vm.add_instruction(Instruction::Or(reg,reg2));

                        }
                        _ => unreachable!()
                    }
                }
                NodeInstructionXor { lhs, rhs } => {
                    let reg = get_register(&lhs);
                    match rhs  {
                        NodeExpr::NodeExprIntLit{value} => {
                            let int = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(reg+1));
                            self.vm.add_instruction(Instruction::Mov(reg+1,int));
                            self.vm.add_instruction(Instruction::Xor(reg,reg+1));
                            self.vm.add_instruction(Instruction::Pop(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            self.vm.add_instruction(Instruction::Xor(reg,get_register(&rhs))); 
                        }
                        _ => unreachable!()
                    }
                }
                NodeInstructionNand { lhs, rhs } => {
                    let reg = get_register(&lhs);
                    match rhs  {
                        NodeExpr::NodeExprIntLit{value} => {
                            let int = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();             
                            self.vm.add_instruction(Instruction::PushRegister(reg+1));
                            self.vm.add_instruction(Instruction::Mov(reg+1,int));
                            self.vm.add_instruction(Instruction::Nand(reg,reg+1));
                            self.vm.add_instruction(Instruction::Pop(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            let reg2= get_register(&rhs); 
                            self.vm.add_instruction(Instruction::Nand(reg,reg2));

                        }
                        _ => unreachable!()
                    }
                }

               


                NodeInstructionMovf {lhs,rhs} => {
                    let reg = get_fregister(&lhs);
                    match rhs  {
                        NodeExpr::NodeExprFloat{value} => self.vm.add_instruction(Instruction::Movf(reg as InstructionParamType,value.value.clone().unwrap().parse::<FloatInstructionParamType>().unwrap())),
                        NodeExpr::NodeExprRegister{value:_} => {
                            let reg2 = get_fregister(&rhs);
                            self.vm.add_instruction(Instruction::PushFloatRegister(reg2));
                            self.vm.add_instruction(Instruction::PopFloat(reg));
                        } 
                        _ => unreachable!()
                    }
                }


                NodeInstructionAddf { lhs, rhs } => {
                    let reg = get_fregister(&lhs);
                    match rhs  {
                        NodeExpr::NodeExprFloat{value} => {
                            let int = value.value.clone().unwrap().parse::<FloatInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushFloatRegister(reg+1));
                            self.vm.add_instruction(Instruction::Movf(reg+1,int));
                            self.vm.add_instruction(Instruction::Addf(reg,reg+1));
                            self.vm.add_instruction(Instruction::PopFloat(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            self.vm.add_instruction(Instruction::Add(reg,get_fregister(&rhs))); 
                        }
                        _ => unreachable!()
                    }
                }
                NodeInstructionSubf { lhs, rhs } => {
                    let reg = get_fregister(&lhs);
                    match rhs  {

                        NodeExpr::NodeExprFloat{value} => {
                            let int = value.value.clone().unwrap().parse::<FloatInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushFloatRegister(reg+1));
                            self.vm.add_instruction(Instruction::Movf(reg+1,int));
                            self.vm.add_instruction(Instruction::Subf(reg,reg+1));
                            self.vm.add_instruction(Instruction::PopFloat(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            let reg2= get_fregister(&rhs); 
                            self.vm.add_instruction(Instruction::Subf(reg,reg2));

                        }
                        _ => unreachable!()
                    }
                }
                NodeInstructionMulf { lhs, rhs } => {
                    let reg = get_fregister(&lhs);
                    match rhs  {
                        NodeExpr::NodeExprFloat{value} => {
                            let int = value.value.clone().unwrap().parse::<FloatInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushFloatRegister(reg+1));
                            self.vm.add_instruction(Instruction::Movf(reg+1,int));
                            self.vm.add_instruction(Instruction::Mulf(reg,reg+1));
                            self.vm.add_instruction(Instruction::PopFloat(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            self.vm.add_instruction(Instruction::Mulf(reg,get_fregister(&rhs))); 
                        }
                        _ => unreachable!()
                    }
                }
                NodeInstructionDivf { lhs, rhs } => {
                   let reg = get_fregister(&lhs);
                    match rhs  {
                        NodeExpr::NodeExprFloat{value} => {
                            let int = value.value.clone().unwrap().parse::<FloatInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushFloatRegister(reg+1));
                            self.vm.add_instruction(Instruction::Movf(reg+1,int));
                            self.vm.add_instruction(Instruction::Divf(reg,reg+1));
                            self.vm.add_instruction(Instruction::PopFloat(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            self.vm.add_instruction(Instruction::Divf(reg,get_fregister(&rhs))); 
                        }
                        _ => unreachable!()
                    }
                }

                NodeInstructionModf { lhs, rhs } => {
                    let reg = get_fregister(&lhs);
                    match rhs  {
                        NodeExpr::NodeExprFloat{value} => {
                            let int = value.value.clone().unwrap().parse::<FloatInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushFloatRegister(reg+1));
                            self.vm.add_instruction(Instruction::Movf(reg+1,int));
                            self.vm.add_instruction(Instruction::Modf(reg,reg+1));
                            self.vm.add_instruction(Instruction::PopFloat(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            let reg2= get_fregister(&rhs); 
                            self.vm.add_instruction(Instruction::Modf(reg,reg2));

                        }
                        _ => unreachable!()
                    }
                }
                NodeInstructionGetFlag { lhs, rhs} => {
                    let reg = get_register(&lhs);
                    match rhs {
                        NodeExpr::NodeExprIntLit { value } => {
                            let int = value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(reg+1));
                            self.vm.add_instruction(Instruction::Mov(reg+1,int as iInstructionParamType));
                            self.vm.add_instruction(Instruction::GetFlag(reg,reg+1));
                            self.vm.add_instruction(Instruction::Pop(reg+1))
                        }
                        NodeExpr::NodeExprFlag { value } => {
                            let int = get_flag(value);
                            self.vm.add_instruction(Instruction::PushRegister(reg+1));
                            self.vm.add_instruction(Instruction::Mov(reg+1,int as iInstructionParamType));
                            self.vm.add_instruction(Instruction::GetFlag(reg,reg+1));
                            self.vm.add_instruction(Instruction::Pop(reg+1))
                        }
                        NodeExpr::NodeExprRegister { value:_ } => {
                            self.vm.add_instruction(Instruction::GetFlag(reg,get_register(&rhs)));
                        } 
                        _ => unreachable!()
                    }
                }

                NodeInstructionGetStackPointer { lhs } => {
                    let reg = get_register(&lhs);
                    self.vm.add_instruction(Instruction::GetStackPointer(reg)); 
                }

                NodeInstructionTruncateStackRange { lhs, rhs } => {
                    let reg = get_register(&lhs);
                    match rhs  {
                        NodeExpr::NodeExprIntLit{value} => {
                            let int = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(reg+1));
                            self.vm.add_instruction(Instruction::Mov(reg+1,int));
                            self.vm.add_instruction(Instruction::TruncateStackRange(reg,reg+1));
                            self.vm.add_instruction(Instruction::Pop(reg+1));
                        }
                        NodeExpr::NodeExprRegister{value:_} => {
                            self.vm.add_instruction(Instruction::TruncateStackRange(reg,get_register(&rhs))); 
                        }
                        _ => unreachable!()
                    }

                }

                NodeInstructionCall { value } => {
                    match value {
                        NodeExpr::NodeExprIntLit{value} => {
                            let inst_address = value.value.as_ref().unwrap().parse::<u32>().unwrap();
                            self.vm.add_instruction(Instruction::Call(crate::instruction::StringNumberUnion::Num(inst_address)));
                        }
                        NodeExpr::NodeExprLabelName{value:_v} => {
                            let label_name=  get_jump_label(value.clone()).unwrap();
                            self.vm.add_instruction(Instruction::Call(crate::instruction::StringNumberUnion::String(label_name)));
                        }
                        _ => unreachable!()
                    }
                }

                NodeInstructionWrite { value } => {
                    match value {
                        NodeExpr::NodeExprIntLit { value } => {
                            let int = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::Mov(RESERVEREGISTER1,int));
                            self.vm.add_instruction(Instruction::Write(RESERVEREGISTER1));

                        }
                        NodeExpr::NodeExprRegister { value:_ } => {
                            self.vm.add_instruction(Instruction::Write(get_register(&value)));
                        }
                        _ => unreachable!()
                    }
                }

            }
        }
    }

}

pub fn get_register_value(reg:Token) -> Option<InstructionParamType> {
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

pub fn get_fregister_value(reg:Token) -> Option<InstructionParamType> {
    
    if reg.token_type != TokenType::FloatRegister { return None };
    
    if let Some(val) = reg.value {
        let val = match val.as_str() {
            "fa" => REGA,
            "fb" => REGB,
            "fc" => REGC,
            "fd" => REGD,
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

pub fn get_register(value:&NodeExpr) -> InstructionParamType {
    match value  {
        NodeExpr::NodeExprRegister { value } => {
            if let Some(register) = get_register_value(value.clone()) {
                return register
            }else {
                panic!("Register {:?} does not exist.",value.value.clone().unwrap());
            }
        }
        NodeExpr::NodeExprIntLit{value} => {
            return value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
        }
        _ => panic!("Internal Error. Expected Register found integer while moving. (1st argument)."),
    };
}

pub fn get_fregister(value:&NodeExpr) -> InstructionParamType {
    match value  {
        NodeExpr::NodeExprRegister { value } => {
            if let Some(register) = get_fregister_value(value.clone()) {
                return register
            }else {
                panic!("Float Register {:?} does not exist.",value.value.clone().unwrap());
            }
        }
        NodeExpr::NodeExprIntLit{value} => {
            return value.value.clone().unwrap().parse::<InstructionParamType>().unwrap();
        }
        _ => panic!("Internal Error. Expected Register found integer while moving. (1st argument)."),
    };
}

pub fn get_flag(value:&Token) -> InstructionParamType {
    let v = match value.value.clone().unwrap().as_str() {
        "zf" => ZERO_FLAG ,
        "eqf" => EQUAL_FLAG,
        "gf" => GREATER_THAN_FLAG,

        "lf" => LESS_THAN_FLAG,
         _ => {
            println!("Internal Error. Flag {:?} does note exist.",value);
            std::process::exit(1);
        }
    };
    v as u32
}
