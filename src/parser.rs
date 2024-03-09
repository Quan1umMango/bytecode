use crate::tokens::*;

use std::collections::HashMap;

pub struct Parser {
    tokens:Vec<Token>,
    index: usize,
    labels: HashMap<String,NodeLabel>
}


#[derive(Debug,Clone)]
pub enum NodeExpr {
    NodeExprRegister {value:Token},
    NodeExprIntLit   {value: Token},
    NodeExprLabelName {value: Token},
}

#[derive(Debug,Clone)]
pub enum NodeInstruction {
    NodeInstructionHalt,
    NodeInstructionMov {lhs:NodeExpr,rhs:NodeExpr},
    NodeInstructionAdd {lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionSub{lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionDisplay{value:NodeExpr},
    NodeInstructionPush{value:NodeExpr},
    NodeInstructionPop{value:NodeExpr},

    NodeInstructionJump{value:NodeExpr},
    NodeInstructionJumpIfZero{value:NodeExpr},
    NodeInstructionJumpIfNotZero{value:NodeExpr},
    NodeInstructionJumpIfEqual{value:NodeExpr},
    NodeInstructionJumpIfNotEqual{value:NodeExpr},
    NodeInstructionJumpIfGreater{value:NodeExpr},
    NodeInstructionJumpIfLess{value:NodeExpr},

    NodeInstructionCompare{lhs:NodeExpr, rhs:NodeExpr},

    NodeInstructionGetFromStack{lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionGetFromStackPointer{lhs:NodeExpr, rhs:NodeExpr},

    NodeInstructionMalloc{value:NodeExpr},
    NodeInstructionGetMemory{lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionSetMemory{lhs:NodeExpr, rhs:NodeExpr},

    NodeInstructionReturn,

    NodeInstructionMul{lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionDiv{lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionMod{lhs:NodeExpr, rhs:NodeExpr},

    NodeInstructionOr{lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionAnd{lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionNot{value: NodeExpr},
    NodeInstructionXor{lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionNand{lhs:NodeExpr, rhs:NodeExpr},

    NodeInstructionTruncateStack{value:NodeExpr},

}


#[derive(Debug,Clone)]
pub struct NodeLabel {
   pub insts:Vec<NodeInstruction>
}


impl  Parser {
    pub fn new(tokens:Vec<Token>) -> Self {
        Self {
            tokens,
            index:0,
            labels:HashMap::new()
        }
    }

    pub fn parse_halt(&mut self) -> Option<NodeInstruction> {
        if let Some(_halt_tok) = self.try_consume(TokenType::Halt) {
            return Some(NodeInstruction::NodeInstructionHalt);
        } 
        None
    }

    pub fn parse_mov(&mut self) -> Option<NodeInstruction> {

        if let Some(_mov_tok) = self.try_consume(TokenType::Mov) {
            if let Some(register_tok) = self.try_consume(TokenType::Register) {
                   if self.try_consume(TokenType::Comma).is_none() {
                    println!("Expected Comma, found:{:?}",self.peek_token());
                }
                if let Some(int_lit) = self.try_consume(TokenType::IntLit) {
                    return Some(NodeInstruction::NodeInstructionMov {
                        lhs: NodeExpr::NodeExprRegister{value:register_tok},
                        rhs:NodeExpr::NodeExprIntLit{value:int_lit}
                    });
                }else if let Some(register2_tok) = self.try_consume(TokenType::Register) {
                    return Some(NodeInstruction::NodeInstructionMov {
                        lhs: NodeExpr::NodeExprRegister{value:register_tok},
                        rhs:NodeExpr::NodeExprRegister{value:register2_tok}
                    });
                }else {
                    println!("Expected either register or number value to push into register.");
                    std::process::exit(1);
                }

            }else {
                println!("Expected register to move into.");
                std::process::exit(1);
            }
        }
        None
    }



    pub fn parse_add(&mut self) -> Option<NodeInstruction> {
        if let Some(_add_tok) = self.try_consume(TokenType::Add) {
            if let Some(register_tok) = self.try_consume(TokenType::Register) {
                if self.try_consume(TokenType::Comma).is_none() {
                    println!("Expected Comma, found:{:?}",self.peek_token());
                }
                if let Some(int_lit) = self.try_consume(TokenType::IntLit) {
                    return Some(NodeInstruction::NodeInstructionAdd {
                        lhs: NodeExpr::NodeExprRegister{value:register_tok},
                        rhs:NodeExpr::NodeExprIntLit{value:int_lit}
                    });
                }else if let Some(register2_tok) = self.try_consume(TokenType::Register) {
                    return Some(NodeInstruction::NodeInstructionAdd {
                        lhs: NodeExpr::NodeExprRegister{value:register_tok},
                        rhs:NodeExpr::NodeExprRegister{value:register2_tok}
                    });
                }else {
                    println!("Expected either register or number value to add into register.");
                    std::process::exit(1);
                }

            }else {
                println!("Expected register for to add.");
                std::process::exit(1);

            }
        }else {
            None
        }
    }

    pub fn parse_sub(&mut self) -> Option<NodeInstruction> {
        if let Some(_sub_tok) = self.try_consume(TokenType::Sub) {
            if let Some(register_tok) = self.try_consume(TokenType::Register) {
                if self.try_consume(TokenType::Comma).is_none() {
                    println!("Expected Comma, found:{:?}",self.peek_token());
                }
                if let Some(int_lit) = self.try_consume(TokenType::IntLit) {
                    return Some(NodeInstruction::NodeInstructionSub {
                        lhs: NodeExpr::NodeExprRegister{value:register_tok},
                        rhs:NodeExpr::NodeExprIntLit{value:int_lit}}
                    );
                }else if let Some(register2_tok) = self.try_consume(TokenType::Register) {
                    return Some(NodeInstruction::NodeInstructionSub {
                        lhs: NodeExpr::NodeExprRegister{value:register_tok},
                        rhs:NodeExpr::NodeExprRegister{value:register2_tok}}
                    );
                }else {
                    println!("Expected either register or number value to subtract into register.");
                    std::process::exit(1);
                }

            }else {
                println!("Expected register for to subtract.");
                std::process::exit(1);

            }
        }else {
            None
        }
    }




    pub fn parse_mul(&mut self) -> Option<NodeInstruction> {
        if let Some(_mul_tok) = self.try_consume(TokenType::Mul) {
            if let Some(register_tok) = self.try_consume(TokenType::Register) {
   if self.try_consume(TokenType::Comma).is_none() {
                    println!("Expected Comma, found:{:?}",self.peek_token());
                }
                if let Some(int_lit) = self.try_consume(TokenType::IntLit) {
                    return Some(NodeInstruction::NodeInstructionMul {
                        lhs: NodeExpr::NodeExprRegister{value:register_tok},
                        rhs:NodeExpr::NodeExprIntLit{value:int_lit}
                    });
                }else if let Some(register2_tok) = self.try_consume(TokenType::Register) {
                    return Some(NodeInstruction::NodeInstructionMul {
                        lhs: NodeExpr::NodeExprRegister{value:register_tok},
                        rhs:NodeExpr::NodeExprRegister{value:register2_tok}
                    });
                }else {
                    println!("Expected either register or number value to multiply into register.");
                    std::process::exit(1);
                }

            }else {
                println!("Expected register for to multiply.");
                std::process::exit(1);

            }
        }else {
            None
        }
    }
   
      pub fn parse_div(&mut self) -> Option<NodeInstruction> {
        if let Some(_div_tok) = self.try_consume(TokenType::Div) {
            if let Some(register_tok) = self.try_consume(TokenType::Register) {
                if self.try_consume(TokenType::Comma).is_none() {
                    println!("Expected Comma, found:{:?}",self.peek_token());
                }
                if let Some(int_lit) = self.try_consume(TokenType::IntLit) {
                    return Some(NodeInstruction::NodeInstructionDiv {
                        lhs: NodeExpr::NodeExprRegister{value:register_tok},
                        rhs:NodeExpr::NodeExprIntLit{value:int_lit}}
                    );
                }else if let Some(register2_tok) = self.try_consume(TokenType::Register) {
                    return Some(NodeInstruction::NodeInstructionDiv {
                        lhs: NodeExpr::NodeExprRegister{value:register_tok},
                        rhs:NodeExpr::NodeExprRegister{value:register2_tok}
                    });
                }else {
                    println!("Expected either register or number value to subtract into register.");
                    std::process::exit(1);
                }

            }else {
                println!("Expected register for to subtract.");
                std::process::exit(1);

            }
        }else {
            None
        }
    }
   
      pub fn parse_mod(&mut self) -> Option<NodeInstruction> {
        if let Some(_mod_tok) = self.try_consume(TokenType::Mod) {
            if let Some(register_tok) = self.try_consume(TokenType::Register) {
                if self.try_consume(TokenType::Comma).is_none() {
                    println!("Expected Comma, found:{:?}",self.peek_token());
                }
                if let Some(int_lit) = self.try_consume(TokenType::IntLit) {
                    return Some(NodeInstruction::NodeInstructionMod {
                        lhs: NodeExpr::NodeExprRegister{value:register_tok},
                        rhs:NodeExpr::NodeExprIntLit{value:int_lit}}
                    );
                }else if let Some(register2_tok) = self.try_consume(TokenType::Register) {
                    return Some(NodeInstruction::NodeInstructionMod {
                        lhs: NodeExpr::NodeExprRegister{value:register_tok},
                        rhs:NodeExpr::NodeExprRegister{value:register2_tok}
                    });
                }else {
                    println!("Expected either register or number value to mod into register.");
                    std::process::exit(1);
                }

            }else {
                println!("Expected register for to subtract.");
                std::process::exit(1);

            }
        }else {
            None
        }
    }



    pub fn parse_logical(&mut self) -> Option<NodeInstruction> {
        let tok = self.peek_token();
        if tok.is_none() { return None; }
        use TokenType::{Not,Or,And,Nand,Xor};
        match tok.unwrap().token_type {
            Not => {
                self.consume_token();
                if let Some(register_tok) = self.try_consume(TokenType::Register) {
                    return Some(NodeInstruction::NodeInstructionNot {
                        value:NodeExpr::NodeExprRegister{value:register_tok}
                    })
                }else {
                    println!("Expected register to perform Not operation.");
                    std::process::exit(1);
                }
            }
            Or => {

                self.consume_token();
                if let Some(register_tok) = self.try_consume(TokenType::Register) {
                    if self.try_consume(TokenType::Comma).is_none() {
                        println!("Expected Comma, found:{:?}",self.peek_token());
                        std::process::exit(1);
                    }
                    if let Some(int_lit) = self.try_consume(TokenType::IntLit) {
                        return Some(NodeInstruction::NodeInstructionOr {
                            lhs: NodeExpr::NodeExprRegister{value:register_tok},
                            rhs:NodeExpr::NodeExprIntLit{value:int_lit}
                        });
                    }else if let Some(register2_tok) = self.try_consume(TokenType::Register) {
                        return Some(NodeInstruction::NodeInstructionOr {
                            lhs: NodeExpr::NodeExprRegister{value:register_tok},
                            rhs:NodeExpr::NodeExprRegister{value:register2_tok}
                        });
                    }else {
                        println!("Expected either register or number value to perform or operteration into register.");
                        std::process::exit(1);
                    }

                }else {
                    println!("Expected register for to perform or operteration.");
                    std::process::exit(1);

                }
            }
            And => {

                self.consume_token();
                if let Some(register_tok) = self.try_consume(TokenType::Register) {
                    if self.try_consume(TokenType::Comma).is_none() {
                        println!("Expected Comma, found:{:?}",self.peek_token());
                    }
                    if let Some(int_lit) = self.try_consume(TokenType::IntLit) {
                        return Some(NodeInstruction::NodeInstructionAnd {
                            lhs: NodeExpr::NodeExprRegister{value:register_tok},
                            rhs:NodeExpr::NodeExprIntLit{value:int_lit}
                        });
                    }else if let Some(register2_tok) = self.try_consume(TokenType::Register) {
                        return Some(NodeInstruction::NodeInstructionAnd {
                            lhs: NodeExpr::NodeExprRegister{value:register_tok},
                            rhs:NodeExpr::NodeExprRegister{value:register2_tok}
                        });
                    }else {
                        println!("Expected either register or number value to perform and operteration into register.");
                        std::process::exit(1);
                    }

                }else {
                    println!("Expected register for to perform and operteration.");
                    std::process::exit(1);

                }
            }
            Nand => {
self.consume_token();

                    if let Some(register_tok) = self.try_consume(TokenType::Register) {
                    if self.try_consume(TokenType::Comma).is_none() {
                        println!("Expected Comma, found:{:?}",self.peek_token());
                    }
                    if let Some(int_lit) = self.try_consume(TokenType::IntLit) {
                        return Some(NodeInstruction::NodeInstructionNand {
                            lhs: NodeExpr::NodeExprRegister{value:register_tok},
                            rhs:NodeExpr::NodeExprIntLit{value:int_lit}
                        });
                    }else if let Some(register2_tok) = self.try_consume(TokenType::Register) {
                        return Some(NodeInstruction::NodeInstructionNand {
                            lhs: NodeExpr::NodeExprRegister{value:register_tok},
                            rhs:NodeExpr::NodeExprRegister{value:register2_tok}
                        });
                    }else {
                        println!("Expected either register or number value to perform Nand op operteration into register.");
                        std::process::exit(1);
                    }

                }else {
                    println!("Expected register for to perform Nand op operteration.");
                    std::process::exit(1);

                }
            }
            Xor => {

                self.consume_token();
        if let Some(register_tok) = self.try_consume(TokenType::Register) {
                    if self.try_consume(TokenType::Comma).is_none() {
                        println!("Expected Comma, found:{:?}",self.peek_token());
                    }
                    if let Some(int_lit) = self.try_consume(TokenType::IntLit) {
                        return Some(NodeInstruction::NodeInstructionXor {
                            lhs: NodeExpr::NodeExprRegister{value:register_tok},
                            rhs:NodeExpr::NodeExprIntLit{value:int_lit}
                        });
                    }else if let Some(register2_tok) = self.try_consume(TokenType::Register) {
                        return Some(NodeInstruction::NodeInstructionXor {
                            lhs: NodeExpr::NodeExprRegister{value:register_tok},
                            rhs:NodeExpr::NodeExprRegister{value:register2_tok}
                        });
                    }else {
                        println!("Expected either register or number value to perform xor operteration into register.");
                        std::process::exit(1);
                    }

                }else {
                    println!("Expected register for to perform xor operteration.");
                    std::process::exit(1);

                }
            }
            _ => None
        }

    }
    pub fn parse_display(&mut self) -> Option<NodeInstruction> {
        if let Some(_display_tok) = self.try_consume(TokenType::Display) {
            if let Some(int_lit) = self.try_consume(TokenType::IntLit) {
                return Some(NodeInstruction::NodeInstructionDisplay {
                    value:NodeExpr::NodeExprIntLit{value:int_lit}
                });
            }else if let Some(reg) = self.try_consume(TokenType::Register) {
                return Some(NodeInstruction::NodeInstructionDisplay {
                    value:NodeExpr::NodeExprRegister{value:reg}
                });
            }else {
                println!("Expected register or number to display.");
                std::process::exit(1);
            }
         }
        None
    }
    pub fn parse_push(&mut self) -> Option<NodeInstruction> {
        if let Some(_push_tok) = self.try_consume(TokenType::Push) {
            if let Some(int_lit) = self.try_consume(TokenType::IntLit) {
                return Some(NodeInstruction::NodeInstructionPush {
                    value:NodeExpr::NodeExprIntLit{value:int_lit}
                });
            }else if let Some(reg) = self.try_consume(TokenType::Register) {
                return Some(NodeInstruction::NodeInstructionPush {
                    value:NodeExpr::NodeExprRegister{value:reg}
                });
            }else {
                println!("Expected register or number to push.");
                std::process::exit(1);
            }
         }
        None
    }

    pub fn parse_pop(&mut self) -> Option<NodeInstruction> {
        if let Some(_pop_tok) = self.try_consume(TokenType::Pop) {
        if let Some(reg) = self.try_consume(TokenType::Register) {
                return Some(NodeInstruction::NodeInstructionPop {
                    value:NodeExpr::NodeExprRegister{value:reg}
                });
            }else {
                println!("Expected register to pop.");
                std::process::exit(1);
            }
         }
        None
    }

    pub fn parse_jump(&mut self) -> Option<NodeInstruction> {
        if let Some(_jump_tok) = self.try_consume(TokenType::Jump) {
           
            if let Some(reg) = self.try_consume(TokenType::Ident) {
                return Some(NodeInstruction::NodeInstructionJump {
                    value:NodeExpr::NodeExprLabelName{value:reg}
                });
            }else {
                println!("Expected register or number to jump.");
                std::process::exit(1);
            }
         }
        None
    }

  pub fn parse_jump_zero(&mut self) -> Option<NodeInstruction> {
        if let Some(_jump_tok) = self.try_consume(TokenType::JumpIfZero) {
           
            if let Some(reg) = self.try_consume(TokenType::Ident) {
                return Some(NodeInstruction::NodeInstructionJumpIfZero {
                    value:NodeExpr::NodeExprLabelName{value:reg}
                });
            }else {
                println!("Expected register or number to jump.");
                std::process::exit(1);
            }
         }
        None
    }

  pub fn parse_jump_nzero(&mut self) -> Option<NodeInstruction> {
        if let Some(_jump_tok) = self.try_consume(TokenType::JumpIfNotZero) {
           
          
            if let Some(reg) = self.try_consume(TokenType::Ident) {

                return Some(NodeInstruction::NodeInstructionJumpIfNotZero {
                   
                    value:NodeExpr::NodeExprLabelName{value:reg}

                });
            }else {
                println!("Expected register or number to jump.");
                std::process::exit(1);
            }
         }
        None
    }

  pub fn parse_jump_equal(&mut self) -> Option<NodeInstruction> {
        if let Some(_jump_tok) = self.try_consume(TokenType::JumpIfEqual) {
           
            
            if let Some(reg) = self.try_consume(TokenType::Ident) {
                return Some(NodeInstruction::NodeInstructionJumpIfEqual { 
                    value:NodeExpr::NodeExprLabelName{value:reg}
                });
            }else {
                println!("Expected register or number to jump.");
                std::process::exit(1);
            }
         }
        None
    }


  pub fn parse_jump_nequal(&mut self) -> Option<NodeInstruction> {
        if let Some(_jump_tok) = self.try_consume(TokenType::JumpIfNotEqual) {
           
            
            if let Some(reg) = self.try_consume(TokenType::Ident) {
                return Some(NodeInstruction::NodeInstructionJumpIfNotEqual {
               
                    value:NodeExpr::NodeExprLabelName{value:reg}

                });
            }else {
                println!("Expected register or number to jump.");
                std::process::exit(1);
            }
         }
        None
    }

    pub fn parse_jump_greater(&mut self) -> Option<NodeInstruction> {
        if let Some(_jump_tok) = self.try_consume(TokenType::JumpIfGreater) {
           
            
            if let Some(reg) = self.try_consume(TokenType::Ident) {
                return Some(NodeInstruction::NodeInstructionJumpIfGreater {
               
                    value:NodeExpr::NodeExprLabelName{value:reg}

                });
            }else {
                println!("Expected register or number to jump.");
                std::process::exit(1);
            }
         }
        None
    }

pub fn parse_jump_less(&mut self) -> Option<NodeInstruction> {
        if let Some(_jump_tok) = self.try_consume(TokenType::JumpIfLess) {
           
            
            if let Some(reg) = self.try_consume(TokenType::Ident) {
                return Some(NodeInstruction::NodeInstructionJumpIfLess {
               
                    value:NodeExpr::NodeExprLabelName{value:reg}

                });
            }else {
                println!("Expected register or number to jump.");
                std::process::exit(1);
            }
         }
        None
    }

    pub fn parse_compare(&mut self) -> Option<NodeInstruction> {
        if let Some(_cmp_tok) = self.try_consume(TokenType::Compare) {
            if let Some(reg1) = self.try_consume(TokenType::Register) {

                if let Some(reg2) = self.try_consume(TokenType::Register) {
                    return Some(NodeInstruction::NodeInstructionCompare {
                        lhs:NodeExpr::NodeExprRegister{value:reg1},
                        rhs: NodeExpr::NodeExprRegister{value:reg2}
                    })
                }else if let Some(int) = self.try_consume(TokenType::IntLit) {
                    return Some(NodeInstruction::NodeInstructionCompare {
                        lhs:NodeExpr::NodeExprRegister{value:reg1},
                        rhs: NodeExpr::NodeExprIntLit{value:int}
                    })
                }else {

                    println!("Expected register or number to compare.");
                    std::process::exit(1);
                }

            }else if let Some(int) = self.try_consume(TokenType::IntLit) {
                if let Some(reg) = self.try_consume(TokenType::Register) {
                     return Some(NodeInstruction::NodeInstructionCompare {
                        lhs:NodeExpr::NodeExprIntLit{value:int},
                        rhs: NodeExpr::NodeExprRegister{value:reg}
                    })
                }else if let Some(int2) = self.try_consume(TokenType::IntLit) {
                        return Some(NodeInstruction::NodeInstructionCompare {
                        lhs:NodeExpr::NodeExprIntLit{value:int},
                        rhs: NodeExpr::NodeExprIntLit{value:int2}
                    })
                }else {

                    println!("Expected register or number to compare.");
                    std::process::exit(1);
                }
            }else {
                println!("Expected register or number to compare.");
                std::process::exit(1);
            }
        } 
        None
    }

    pub fn parse_getstack(&mut self) -> Option<NodeInstruction> {
        if let Some(_getstack_tok) = self.try_consume(TokenType::GetFromStack) {
            let mut lhs:Option<NodeExpr> = None;
            let mut rhs: Option<NodeExpr> = None;
                       if let Some(int) = self.try_consume(TokenType::IntLit) {
                lhs = Some(NodeExpr::NodeExprIntLit{value:int});
            }else if let Some(reg) = self.try_consume(TokenType::Register) {
                lhs =Some(NodeExpr::NodeExprRegister{value:reg});
            }else {
                println!("Expected either integer or register to get from stack");
                std::process::exit(1);
            }

            if let Some(int) = self.try_consume(TokenType::IntLit) {
                rhs = Some(NodeExpr::NodeExprIntLit{value:int});
            }else if let Some(reg) = self.try_consume(TokenType::Register) {
                rhs =Some(NodeExpr::NodeExprRegister{value:reg});
            }else {
                println!("Expected either integer or register to get from stack. (2nd argument)");
                std::process::exit(1);
            }
            return Some(NodeInstruction::NodeInstructionGetFromStack{lhs:lhs.unwrap(),rhs:rhs.unwrap()})

        }
        None
    }

    pub fn parse_getfromsp(&mut self) -> Option<NodeInstruction> {
        if let Some(_getsp_tok) = self.try_consume(TokenType::GetFromStackPointer) {
             let mut lhs:Option<NodeExpr> = None;
            let mut rhs: Option<NodeExpr> = None;
                       if let Some(int) = self.try_consume(TokenType::IntLit) {
                lhs = Some(NodeExpr::NodeExprIntLit{value:int});
            }else if let Some(reg) = self.try_consume(TokenType::Register) {
                lhs =Some(NodeExpr::NodeExprRegister{value:reg});
            }else {
                println!("Expected either integer or register to get from stack pointer.");
                std::process::exit(1);
            }

            if let Some(reg) = self.try_consume(TokenType::Register) {
                rhs =Some(NodeExpr::NodeExprRegister{value:reg});
            }else {
                println!("Expected register to get from stack pointer. (2nd argument)");
                std::process::exit(1);
            }
            return Some(NodeInstruction::NodeInstructionGetFromStackPointer{lhs:lhs.unwrap(),rhs:rhs.unwrap()})

        }
        None
    }

    pub fn parse_truncate_stack(&mut self) -> Option<NodeInstruction> {
        if let Some(_tok) = self.try_consume(TokenType::TruncateStack) {
            if let Some(reg) = self.try_consume(TokenType::Register) {
                return Some(NodeInstruction::NodeInstructionTruncateStack {
                    value:NodeExpr::NodeExprRegister{value:reg}
                } );
            }else if let Some(num) = self.try_consume(TokenType::IntLit) {
                return Some(NodeInstruction::NodeInstructionTruncateStack {
                    value:NodeExpr::NodeExprIntLit{value:num}
                } );
            }else {
                println!("Expected Register or number to truncate stack from, found: {:?}",self.peek_token());
                std::process::exit(1);
            }
        } 
        None
    }

    pub fn parse_malloc(&mut self) -> Option<NodeInstruction> {
        if let Some(_malloc_tok) = self.try_consume(TokenType::Malloc) {
            let mut lhs:Option<NodeExpr> = None;
            if let Some(int) = self.try_consume(TokenType::IntLit) {
                lhs = Some(NodeExpr::NodeExprIntLit{value:int});
            }else if let Some(reg) = self.try_consume(TokenType::Register) {
                lhs =Some(NodeExpr::NodeExprRegister{value:reg});
            }else {
                println!("Expected either integer or register to allocate memory.");
                std::process::exit(1);
            }        
            return Some(NodeInstruction::NodeInstructionMalloc {value: lhs.unwrap()});
        }
        None
    }
    pub fn parse_getmem(&mut self) -> Option<NodeInstruction> {
        if let Some(_getmem_tok) = self.try_consume(TokenType::GetMemory) {
          let mut lhs:Option<NodeExpr> = None;
            let mut rhs: Option<NodeExpr> = None;
           
            if let Some(int) = self.try_consume(TokenType::IntLit) {
                lhs = Some(NodeExpr::NodeExprIntLit{value:int});
            }else if let Some(reg) = self.try_consume(TokenType::Register) {
                lhs =Some(NodeExpr::NodeExprRegister{value:reg});
            }else {
                println!("Expected either integer or register to get from memory.");
                std::process::exit(1);
            }

            if let Some(reg) = self.try_consume(TokenType::Register) {
                rhs =Some(NodeExpr::NodeExprRegister{value:reg});
            }else {
                println!("Expected register to get from memory. (2nd argument)");
                std::process::exit(1);
            }
            return Some(NodeInstruction::NodeInstructionGetMemory{lhs:lhs.unwrap(),rhs:rhs.unwrap()})

        }
        None

    }
    pub fn parse_setmem(&mut self) -> Option<NodeInstruction> {
        if let Some(_setmem_tok) = self.try_consume(TokenType::SetMemory) {
            let mut lhs:Option<NodeExpr> = None;
            let mut rhs: Option<NodeExpr> = None;
            if let Some(int) = self.try_consume(TokenType::IntLit) {
                lhs = Some(NodeExpr::NodeExprIntLit{value:int});
            }else if let Some(reg) = self.try_consume(TokenType::Register) {
                lhs =Some(NodeExpr::NodeExprRegister{value:reg});
            }else {
                println!("Expected either integer or register to set from memory.");
                std::process::exit(1);
            }

            if let Some(reg) = self.try_consume(TokenType::Register) {
                rhs =Some(NodeExpr::NodeExprRegister{value:reg});
            }else {
                println!("Expected register to set from memory. (2nd argument)");
                std::process::exit(1);
            }
            return Some(NodeInstruction::NodeInstructionSetMemory{lhs:lhs.unwrap(),rhs:rhs.unwrap()})

        }
        None

    }

    pub fn parse_label(&mut self) -> Option<(String,NodeLabel)> {
        if let Some(_label_tok) = self.try_consume(TokenType::Label) {
            if let Some(label_name) = self.try_consume(TokenType::Ident) {
                if let Some(_) = self.labels.get(&label_name.value.clone().unwrap()) {
                    println!("Cannot defined lable with name `{:?}` as it is already defined.",label_name.value.clone().unwrap())
                }
                if let Some(_colon) = self.try_consume(TokenType::Colon) {
                    let mut label_inst:Vec<NodeInstruction> = Vec::new();
                    while let Some(inst) = self.parse_inst() {
                        label_inst.push(inst);
                    }

                    while let Some(tok) = self.peek_token() {
                        if tok.token_type == TokenType::Return {
                            let inst = NodeInstruction::NodeInstructionReturn;
                            label_inst.push(inst);
                            self.consume_token();
                            continue;
                        }else  if tok.token_type == TokenType::EndLabel  {
                            self.consume_token();                         
                            return Some((label_name.value.unwrap(),NodeLabel {
                                insts:label_inst
                            }));
                        } else {
                            println!("Undefined Instruction. {:?}",tok);
                            std::process::exit(1);
                        }

                    }
                    if let Some(tok) = self.peek_token_back(1) {
                        if tok.token_type == TokenType::EndLabel {
                            return Some((label_name.value.unwrap(),NodeLabel {
                                insts:label_inst
                            }));

                        }
                    } 

                    println!("Cannot have empty or label with name end: `{:?}`",label_name.value.unwrap());
                    std::process::exit(1);
                }else {
                    println!("Expected colon `:` after label name.");
                    std::process::exit(1);
                }
            }else {
                println!("Expected label name after label keyword.");
                std::process::exit(1);
            }
        }

        None
    }

    pub fn parse_inst(&mut self) -> Option<NodeInstruction> {
        while let Some(_cur_token) = self.peek_token() {
            if let Some(inst_halt) = self.parse_halt() {
                return Some(inst_halt);
            }
            if let Some(inst_mov) = self.parse_mov() {
                return Some(inst_mov);
            }
            if let Some(inst_add) = self.parse_add() {
                return Some(inst_add);
            }
            if let Some(mul) = self.parse_mul() {
                return Some(mul)
            }
            if let Some(div) = self.parse_div() {
                return Some(div)
            }
            if let Some(inst_mod) = self.parse_mod() {
                return Some(inst_mod)
            }
            if let Some(logial) = self.parse_logical() {
                return Some(logial)
            }

            if let Some(inst_sub) = self.parse_sub() {
                return Some(inst_sub);
            } 
            if let Some(inst_display) = self.parse_display() {
                return Some(inst_display);
            } 
            if let Some(push) = self.parse_push() {
                return Some(push)
            }
            if let Some(pop) = self.parse_pop() {
                return Some(pop)
            }
            if let Some(jump) = self.parse_jump() {
                return Some(jump)
            }
            if let Some(jz) = self.parse_jump_zero() {
                return Some(jz)
            }

            if let Some(jnz) = self.parse_jump_nzero() {
                return Some(jnz)
            }
            if let Some(je) = self.parse_jump_equal() {
                return Some(je)
            }
            if let Some(jne) = self.parse_jump_nequal() {
                return Some(jne)
            }
            if let Some(jg) = self.parse_jump_greater() {
                return Some(jg)
            }
            if let Some(jl) = self.parse_jump_less() {
                return Some(jl)
            }
            if let Some(cmp) = self.parse_compare() {
                return Some(cmp)
            }
            if let Some(gfs) = self.parse_getstack() {
                return Some(gfs)
            }
            
            if let Some(gfsp) = self.parse_getfromsp() {
                return Some(gfsp)
            }
            if let Some(trunstack) = self.parse_truncate_stack() {
                return Some(trunstack)
            }

            if let Some(malloc) = self.parse_malloc() {
                return Some(malloc)
            }
            if let Some(gm) = self.parse_getmem() {
                return Some(gm)
            }
            if let Some(sm) = self.parse_setmem() {
                return Some(sm);
            }
           
            else {
                break;
            }
        }
        None
    } 

    pub fn parse(&mut self) -> Option<HashMap<String,NodeLabel>> {
        while self.peek_token().is_some() {
            if let Some((name,label)) = self.parse_label() {
                self.labels.insert(name,label);
            }else {
                println!("Statement is out of a label. {:?}",self.peek_token());
                std::process::exit(1);
            }
        } 
        Some(self.labels.clone())
    }

    pub fn peek_token(&self) -> Option<Token> {
        return self.tokens.get(self.index).cloned()
    } 

    pub fn peek_token_back(&self,i:usize) -> Option<Token> {
        return self.tokens.get(self.index-i).cloned();
    }

    pub fn try_consume(&mut self, token_type:TokenType) -> Option<Token> {
        if self.peek_token().is_some() && self.peek_token().unwrap().token_type == token_type  {
            return self.consume_token()
        }
        None
    }

    pub fn consume_token(&mut self) -> Option<Token> {
        if let Some(token) = self.peek_token() {
            self.index += 1;
            return Some(token)
        }
        return None;
    }

}
