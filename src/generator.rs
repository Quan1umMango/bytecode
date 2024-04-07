use crate::{
    parser::{NodeExpr,NodeLabel,NodeInstruction,NodeBuiltin,Parser},
    instruction::Instruction,
    constants_and_types::*, 
    tokens::*,
    vm::VM,
};

use std::collections::HashMap;

pub struct Generator {
    labels: HashMap<String,NodeLabel>,
    created_labels: Vec<String>,
    visited_labels:Vec<String>,
    builtins: Vec<NodeBuiltin>,
    pub vm:VM,
}

impl Generator {
    pub fn new(builtins:Vec<NodeBuiltin>,labels:HashMap<String,NodeLabel>) -> Self {
        Self {
            labels,
            created_labels:Vec::new(),
            vm:VM::new(),
            visited_labels:Vec::new(),
            builtins,
        }
    }



    pub fn create_label(&mut self, name:String) {
        if self.created_labels.iter().find(|x| *x==&name).is_some() { return }
        if let Some(nl) = self.labels.get(&name) {
            self.vm.start_label(name.as_str());
            self.generate_program(nl.insts.clone());
            self.vm.end_label(name.as_str());
            self.created_labels.push(name);
        }else {
            panic!("Couldn't create label with name: {:?} as it does not exist.",name);
        }
    }


    pub fn generate(&mut self) {
        self.generate_builtin();
        self.create_all_labels_in_main();
        self.create_label("main".to_string());
        self.vm.register_start();
    }

    pub fn generate_builtin(&mut self) {
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
                    let mut g =Generator::new(parsed.builtins,parsed.labels);
                    g.generate();
                    //let insts = g.vm.instructions[1..g.vm.last_command()-1].to_vec();
                    for (k,v) in g.labels.iter_mut()  {
                        if *k == "main".to_string() {
                            continue;
                        }
                        self.labels.insert(k.clone(),v.clone());
                    }
                    /*for i in insts {
                        self.vm.add_instruction(i);
                    }*/
                }
            }
        }
    }


    pub fn register_label_in(&mut self, label_name:String) {
        if self.visited_labels.contains(&label_name) { return  }

        let labels = self.labels.get(&label_name).cloned();

        if let Some(nl) = labels {
            self.visited_labels.push(label_name.clone());
            for inst in nl.insts.iter() {
                use NodeInstruction::*;
                match inst {
                    NodeInstructionJump {value} => {
                        match value {
                            NodeExpr::NodeExprIntLit{value:_} => {
                            }
                            NodeExpr::NodeExprLabelName{value:_v} => {
                                let jmp_label = get_jump_label(value.clone());

                                self.register_label_in(jmp_label.clone().unwrap()); 
                                self.create_label(jmp_label.unwrap())

                            },

                            _ => unreachable!()
                        }
                    }

                    NodeInstructionJumpIfZero {value} => {
                        let jmp_label = get_jump_label(value.clone()).unwrap();

                        self.register_label_in(jmp_label.clone()); 

                        if jmp_label != label_name {
                            self.create_label(jmp_label);
                        }
                    },
                    NodeInstructionJumpIfNotZero {value} =>{
                        let jmp_label = get_jump_label(value.clone()).unwrap();

                        self.register_label_in(jmp_label.clone()); 

                        if jmp_label != label_name {
                            self.create_label(jmp_label);
                        }
                    },
                    NodeInstructionJumpIfEqual {value} => {
                        let jmp_label = get_jump_label(value.clone());

                        self.register_label_in(jmp_label.clone().unwrap()); 
                        self.create_label(jmp_label.unwrap())

                    }
                    NodeInstructionJumpIfNotEqual {value} => {
                        let jmp_label = get_jump_label(value.clone()).unwrap();

                        self.register_label_in(jmp_label.clone()); 

                        if jmp_label != label_name {
                            self.create_label(jmp_label);
                        }
                    },

                    NodeInstructionJumpIfLess {value} => {
                        let jmp_label = get_jump_label(value.clone());

                        self.register_label_in(jmp_label.clone().unwrap()); 
                        self.create_label(jmp_label.unwrap())

                    },
                    NodeInstructionJumpIfGreater {value} => {
                        let jmp_label = get_jump_label(value.clone());

                        self.register_label_in(jmp_label.clone().unwrap()); 
                        self.create_label(jmp_label.unwrap())

                    }
                    _ => {}
                }
            }
        }

    } 

    pub fn create_all_labels_in_main(&mut self) {
        self.register_label_in("main".to_owned());
    }

    pub fn generate_program(&mut self,nodes:Vec<NodeInstruction>) -> bool {

        use NodeInstruction::*;
        for node in nodes.iter() {
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
                    let lreg = get_register(&lhs);
                    match rhs {
                        NodeExpr::NodeExprRegister { value: _} => {
                            self.vm.add_instruction(Instruction::GetFromStack(lreg,get_register(&rhs)));
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                            let val = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(lreg+1));
                            self.vm.add_instruction(Instruction::Mov(lreg+1,val));
                            self.vm.add_instruction(Instruction::GetFromStack(lreg,lreg+1));
                            self.vm.add_instruction(Instruction::Pop(lreg+1));
                        }
                        _ => unreachable!()
                    };


                },
                NodeInstructionGetFromStackPointer{lhs, rhs} => {
                    let lreg = get_register(&lhs);
                    match rhs {
                        NodeExpr::NodeExprRegister { value: _} => {
                            self.vm.add_instruction(Instruction::GetFromStackPointer(lreg,get_register(&rhs)));
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                            let val = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
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
                            let val = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
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
                            let int = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::Mov(REGA,int));
                            self.vm.add_instruction(Instruction::Malloc(REGA));
                            self.vm.add_instruction(Instruction::Pop(REGA)); 
                        },
                        _ => unreachable!()
                    }

                },
                NodeInstructionGetMemory{lhs, rhs} => {
                    let lreg = get_register(&lhs);
                    match rhs {
                        NodeExpr::NodeExprRegister { value: _} => {
                            self.vm.add_instruction(Instruction::GetMemory(lreg,get_register(&rhs)));
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                            let val = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(lreg+1));
                            self.vm.add_instruction(Instruction::Mov(lreg+1,val));
                            self.vm.add_instruction(Instruction::GetMemory(lreg,lreg+1));
                            self.vm.add_instruction(Instruction::Pop(lreg+1));
                        }
                        _ => unreachable!()
                    };

                },
                NodeInstructionSetMemory{lhs, rhs} => {
                    let lreg = get_register(&lhs);
                    match rhs {
                        NodeExpr::NodeExprRegister { value: _} => {
                            self.vm.add_instruction(Instruction::SetMemory(lreg,get_register(&rhs)));
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                            let val = value.value.clone().unwrap().parse::<iInstructionParamType>().unwrap();
                            self.vm.add_instruction(Instruction::PushRegister(lreg+1));
                            self.vm.add_instruction(Instruction::Mov(lreg+1,val));
                            self.vm.add_instruction(Instruction::SetMemory(lreg,lreg+1));
                            self.vm.add_instruction(Instruction::Pop(lreg+1));
                        }
                        _ => unreachable!()
                    };

                },

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

                NodeInstructionSetFromStackPointer { lhs, rhs } => {
                    let lreg = get_register(&lhs);
                    match rhs {
                        NodeExpr::NodeExprRegister { value: _} => {
                            self.vm.add_instruction(Instruction::SetFromStackPointer(lreg,get_register(&rhs)));
                        }
                        NodeExpr::NodeExprIntLit { value } => {
                            let int = value.value.clone().unwrap().parse::<FloatInstructionParamType>().unwrap();                  
                            self.vm.add_instruction(Instruction::PushRegister(lreg+1));
                            self.vm.add_instruction(Instruction::Movf(lreg+1,int));
                            self.vm.add_instruction(Instruction::GetFromStackPointer(lreg,lreg+1));
                            self.vm.add_instruction(Instruction::Pop(lreg+1));
                        }
                        _ => unreachable!()
                    };

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
            }
        }
        false
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
