use crate::tokens::*;
use crate::constants_and_types::*;
use crate::parse_jump;

use std::collections::HashMap;

pub struct Parser {
    tokens:Vec<Token>,
    index: usize,
    instruction_counter:usize,
    pub  labels: HashMap<String,usize>,
    pub builtins: Vec<NodeBuiltin>,
    pub instructions: Vec<NodeInstruction>,
}


#[derive(Debug,Clone)]
pub enum NodeExpr {
    NodeExprRegister {value:Token},
    NodeExprIntLit   {value: Token},
    NodeExprFloat    {value: Token},
    NodeExprLabelName {value: Token},
    NodeExprStringLit {value:Token},
    NodeExprFlag     {value:Token},
}

#[derive(Debug,Clone)]
pub enum NodeBuiltin {
    NodeBuiltinImport {value:NodeExpr},
    NodeBuiltinLoadString { value: NodeExpr , load_len:bool}
}

#[derive(Debug,Clone)]
pub enum NodeInstruction  {
    NodeInstructionHalt,
    NodeInstructionMov    {lhs:NodeExpr,rhs:NodeExpr},
    NodeInstructionAdd    {lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionSub    {lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionDisplay {value:NodeExpr},
    NodeInstructionPush   {value:NodeExpr},
    NodeInstructionPop    {value:NodeExpr},

    NodeInstructionCall           {value:NodeExpr},
    NodeInstructionJump           {value:NodeExpr},
    NodeInstructionJumpIfZero     {value:NodeExpr},
    NodeInstructionJumpIfNotZero  {value:NodeExpr},
    NodeInstructionJumpIfEqual    {value:NodeExpr},
    NodeInstructionJumpIfNotEqual {value:NodeExpr},
    NodeInstructionJumpIfGreater  {value:NodeExpr},
    NodeInstructionJumpIfLess     {value:NodeExpr},

    NodeInstructionCompare  {lhs:NodeExpr, rhs:NodeExpr},

    NodeInstructionGetFromStack        {lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionGetFromStackPointer {lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionSetStack {lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionSetFromStackPointer {lhs:NodeExpr, rhs:NodeExpr},

    NodeInstructionMalloc    {value:NodeExpr},
    NodeInstructionGetMemory {lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionSetMemory {lhs:NodeExpr, rhs:NodeExpr},

    NodeInstructionReturn,

    NodeInstructionMul  {lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionDiv  {lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionMod  {lhs:NodeExpr, rhs:NodeExpr},

    NodeInstructionOr   {lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionAnd  {lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionNot  {value: NodeExpr},
    NodeInstructionXor  {lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionNand {lhs:NodeExpr, rhs:NodeExpr},

    NodeInstructionTruncateStack {value:NodeExpr},

    NodeInstructionMovf {lhs:NodeExpr,rhs:NodeExpr},
    NodeInstructionAddf {lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionSubf {lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionMulf {lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionDivf {lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionModf {lhs:NodeExpr, rhs:NodeExpr},
    NodeInstructionDisplayf {value:NodeExpr},
    NodeInstructionDisplayChar {value: NodeExpr},
    NodeInstructionGetFlag { lhs: NodeExpr, rhs:NodeExpr},
    NodeInstructionGetStackPointer {lhs:NodeExpr},
    NodeInstructionTruncateStackRange {lhs:NodeExpr,rhs:NodeExpr},
    NodeInstructionWrite {value:NodeExpr},
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
            labels:HashMap::new(),
            builtins:Vec::new(),
            instructions:Vec::new(),
            instruction_counter:1,
        }
    }

  
    pub fn parse_halt(&mut self) -> Option<NodeInstruction> {
        if let Some(_halt_tok) = self.try_consume(TokenType::Halt) {
            return Some(NodeInstruction::NodeInstructionHalt);
        } 
        None
    }

    #[allow(unused_assignments)]
    pub fn parse_mov(&mut self) -> Option<NodeInstruction> {
        if let Some(_mov_tok) = self.try_consume(TokenType::Mov) {
            let mut lhs:Option<NodeExpr> = None;

            if let Some(register_tok) = self.try_consume(TokenType::Register) {
                lhs = Some(NodeExpr::NodeExprRegister{value:register_tok});
            }else if let Some(int_tok) = self.try_consume(TokenType::IntLit) {
                let register_tok = get_register_from_number(int_tok.value.clone().unwrap().parse::<i32>().unwrap());
                if register_tok.is_none() {
                    println!("Invalid Register.");
                    std::process::exit(1);
                }        
                lhs = Some(NodeExpr::NodeExprIntLit{value:int_tok})
            }else {
                println!("Expected either register or register number to move into.");
                std::process::exit(1);
            }
            let lhs = lhs.unwrap();
            if self.try_consume(TokenType::Comma).is_none() {
                println!("Expected Comma, found:{:?}",self.peek_token());
                std::process::exit(1);
            }    

            if let Some(int_lit) = self.try_consume(TokenType::IntLit) {
                return Some(NodeInstruction::NodeInstructionMov {
                    lhs,
                    rhs:NodeExpr::NodeExprIntLit{value:int_lit}
                });
            }else if let Some(register2_tok) = self.try_consume(TokenType::Register) {
                return Some(NodeInstruction::NodeInstructionMov {
                    lhs,
                    rhs:NodeExpr::NodeExprRegister{value:register2_tok}
                });

            }else {
                println!("Expected either register or number value to move.");
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
    pub fn parse_displayf(&mut self) -> Option<NodeInstruction> {
        if let Some(_display_tok) = self.try_consume(TokenType::Displayf) {
            if let Some(int_lit) = self.try_consume(TokenType::Float) {
                return Some(NodeInstruction::NodeInstructionDisplayf {
                    value:NodeExpr::NodeExprFloat{value:int_lit}
                });
            }else if let Some(reg) = self.try_consume(TokenType::FloatRegister) {
                return Some(NodeInstruction::NodeInstructionDisplayf {
                    value:NodeExpr::NodeExprRegister{value:reg}
                });
            }else {
                println!("Expected register or floating pointer number to display.");
                std::process::exit(1);
            }
        }
        None
    }

    pub fn parse_displayc(&mut self) -> Option<NodeInstruction> {
        if let Some(_display_tok) = self.try_consume(TokenType::DisplayChar) {
            if let Some(int_lit) = self.try_consume(TokenType::IntLit) {
                return Some(NodeInstruction::NodeInstructionDisplayChar {
                    value:NodeExpr::NodeExprFloat{value:int_lit}
                });
            }else if let Some(reg) = self.try_consume(TokenType::Register) {
                return Some(NodeInstruction::NodeInstructionDisplayChar {
                    value:NodeExpr::NodeExprRegister{value:reg}
                });
            }else {
                println!("Expected register or number to displayc.");
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
            }      
            else {
                println!("Expected register to pop into, found {:?}",self.peek_token());
                std::process::exit(1);
            }
        }
        None
    }

  pub fn parse_call(&mut self) -> Option<NodeInstruction> {
        if let Some(_call_tok) = self.try_consume(TokenType::Call) {

            if let Some(reg) = self.try_consume(TokenType::Ident) {
                return Some(NodeInstruction::NodeInstructionCall {
                    value:NodeExpr::NodeExprLabelName{value:reg}
                });
            }else if let Some(ad) = self.try_consume(TokenType::IntLit) {
                let int = ad.value.as_ref().unwrap().parse::<i32>().unwrap();
                if int < 0 {
                    println!("Expected label or instruction number to call, found negative integer.");
                    std::process::exit(1);
                }
                return Some(NodeInstruction::NodeInstructionCall {
                    value:NodeExpr::NodeExprIntLit{value:ad}
                })

            }
            else {
                println!("Expected label or instruction number to call.");
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
            }else if let Some(ad) = self.try_consume(TokenType::IntLit) {
                let int = ad.value.as_ref().unwrap().parse::<i32>().unwrap();
                if int < 0 {
                    println!("Expected label or instruction number to jump, found negative integer.");
                    std::process::exit(1);
                }
                return Some(NodeInstruction::NodeInstructionJump {
                    value:NodeExpr::NodeExprIntLit{value:ad}
                })

            }
            else {
                println!("Expected label or instruction number to jump.");
                std::process::exit(1);
            }
        }
        None
    }

    pub fn parse_jump_zero(&mut self) -> Option<NodeInstruction> {
        let jmp_token_type=  TokenType::JumpIfZero;
        let mut jmp_node_inst = NodeInstruction::NodeInstructionJumpIfZero {
            value:NodeExpr::NodeExprLabelName{value:Token {token_type:TokenType::IntLit,value:Some("0".to_string())}}
        };
        let mut try_consume = |t| {self.try_consume(t)};
        let res = parse_jump!(jmp_token_type,jmp_node_inst,try_consume); 
        if res.is_ok(){
            return Some(jmp_node_inst);
        }
        None

    }

    pub fn parse_jump_nzero(&mut self) -> Option<NodeInstruction> {
        let jmp_token_type=  TokenType::JumpIfNotZero;
        let mut jmp_node_inst = NodeInstruction::NodeInstructionJumpIfNotZero {
            value:NodeExpr::NodeExprLabelName{value:Token {token_type:TokenType::IntLit,value:Some("0".to_string())}}
        };
        let mut try_consume = |t| {self.try_consume(t)};
        let res = parse_jump!(jmp_token_type,jmp_node_inst,try_consume); 
        if res.is_ok(){
            return Some(jmp_node_inst);
        }
        None
    }

    pub fn parse_jump_equal(&mut self) -> Option<NodeInstruction> {
        let jmp_token_type=  TokenType::JumpIfEqual;
        let mut jmp_node_inst = NodeInstruction::NodeInstructionJumpIfEqual {
            value:NodeExpr::NodeExprLabelName{value:Token {token_type:TokenType::IntLit,value:Some("0".to_string())}}
        };
        let mut try_consume = |t| {self.try_consume(t)};
        let res = parse_jump!(jmp_token_type,jmp_node_inst,try_consume); 
        if res.is_ok(){
            return Some(jmp_node_inst);
        }
        None  
    }


    pub fn parse_jump_nequal(&mut self) -> Option<NodeInstruction> {
        let jmp_token_type=  TokenType::JumpIfNotEqual;
        let mut jmp_node_inst = NodeInstruction::NodeInstructionJumpIfNotEqual {
            value:NodeExpr::NodeExprLabelName{value:Token {token_type:TokenType::IntLit,value:Some("0".to_string())}}
        };
        let mut try_consume = |t| {self.try_consume(t)};
        let res = parse_jump!(jmp_token_type,jmp_node_inst,try_consume); 
        if res.is_ok(){
            return Some(jmp_node_inst);
        }
        None    
    }

    pub fn parse_jump_greater(&mut self) -> Option<NodeInstruction> {
        let jmp_token_type=  TokenType::JumpIfGreater;
        let mut jmp_node_inst = NodeInstruction::NodeInstructionJumpIfGreater {
            value:NodeExpr::NodeExprLabelName{value:Token {token_type:TokenType::IntLit,value:Some("0".to_string())}}
        };
        let mut try_consume = |t| {self.try_consume(t)};
        let res = parse_jump!(jmp_token_type,jmp_node_inst,try_consume); 
        if res.is_ok(){
            return Some(jmp_node_inst);
        }
        None    
    }

    pub fn parse_jump_less(&mut self) -> Option<NodeInstruction> {
        let jmp_token_type=  TokenType::JumpIfLess;
        let mut jmp_node_inst = NodeInstruction::NodeInstructionJumpIfLess {
            value:NodeExpr::NodeExprLabelName{value:Token {token_type:TokenType::IntLit,value:Some("0".to_string())}}
        };
        let mut try_consume = |t| {self.try_consume(t)};
        let res = parse_jump!(jmp_token_type,jmp_node_inst,try_consume); 
        if res.is_ok(){
            return Some(jmp_node_inst);
        }
        None   
    }

    pub fn parse_compare(&mut self) -> Option<NodeInstruction> {
        if let Some(_cmp_tok) = self.try_consume(TokenType::Compare) {
            if let Some(reg1) = self.try_consume(TokenType::Register) {
                if self.try_consume(TokenType::Comma).is_none() {
                    println!("Expected Comma, found:{:?}",self.peek_token());
                }

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
                if self.try_consume(TokenType::Comma).is_none() {
                    println!("Expected Comma, found:{:?}",self.peek_token());
                }

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
            if self.try_consume(TokenType::Comma).is_none() {
                println!("Expected Comma, found:{:?}",self.peek_token());
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
            if self.try_consume(TokenType::Comma).is_none() {
                println!("Expected Comma, found:{:?}",self.peek_token());
                std::process::exit(1);
            }

            if let Some(reg) = self.try_consume(TokenType::Register) {
                rhs =Some(NodeExpr::NodeExprRegister{value:reg});
            }else if let Some(int) = self.try_consume(TokenType::IntLit) {
                rhs =Some(NodeExpr::NodeExprIntLit{value:int});


            }else {
                println!("Expected register to get from stack pointer. (2nd argument)");
                std::process::exit(1);
            }
            return Some(NodeInstruction::NodeInstructionGetFromStackPointer{lhs:lhs.unwrap(),rhs:rhs.unwrap()})

        }
        None
    }

    #[allow(unused_assignments)]
    pub fn parse_setstack(&mut self) -> Option<NodeInstruction> {
        if let Some(_sets_tok) = self.try_consume(TokenType::SetStack) {
            let mut lhs:Option<NodeExpr> = None;
            let mut rhs: Option<NodeExpr> = None;
            if let Some(int) = self.try_consume(TokenType::IntLit) {
                lhs = Some(NodeExpr::NodeExprIntLit{value:int});
            }else if let Some(reg) = self.try_consume(TokenType::Register) {
                lhs =Some(NodeExpr::NodeExprRegister{value:reg});
            }else {
                println!("Expected either integer or register to set from stack.");
                std::process::exit(1);
            }

            if self.try_consume(TokenType::Comma).is_none() {
                println!("Expected Comma, found:{:?}",self.peek_token());
                std::process::exit(1);
            }
            if let Some(reg) = self.try_consume(TokenType::Register) {
                rhs =Some(NodeExpr::NodeExprRegister{value:reg});

            }else {
                println!("Expected register literal to set from stack. (2nd argument)\n Found: {:?}",self.consume_token());
                std::process::exit(1);
            }
            return Some(NodeInstruction::NodeInstructionSetStack{lhs:lhs.unwrap(),rhs:rhs.unwrap()})

        }
        None
    }



    #[allow(unused_assignments)]
    pub fn parse_setfromsp(&mut self) -> Option<NodeInstruction> {
        if let Some(_setsp_tok) = self.try_consume(TokenType::SetFromStackPointer) {
            let mut lhs:Option<NodeExpr> = None;
            let mut rhs: Option<NodeExpr> = None;
            if let Some(int) = self.try_consume(TokenType::IntLit) {
                lhs = Some(NodeExpr::NodeExprIntLit{value:int});
            }else if let Some(reg) = self.try_consume(TokenType::Register) {
                lhs =Some(NodeExpr::NodeExprRegister{value:reg});
            }else {
                println!("Expected either integer or register to set from stack pointer.");
                std::process::exit(1);
            }

            if self.try_consume(TokenType::Comma).is_none() {
                println!("Expected Comma, found:{:?}",self.peek_token());
                std::process::exit(1);
            }
            if let Some(reg) = self.try_consume(TokenType::Register) {
                rhs =Some(NodeExpr::NodeExprRegister{value:reg});

            }else {
                println!("Expected register literal to set from stack pointer. (2nd argument)\n Found: {:?}",self.consume_token());
                std::process::exit(1);
            }
            return Some(NodeInstruction::NodeInstructionSetFromStackPointer{lhs:lhs.unwrap(),rhs:rhs.unwrap()})

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

    #[allow(unused_assignments)]
    pub fn parse_truncstackrange(&mut self) -> Option<NodeInstruction> {
         if let Some(_getsp_tok) = self.try_consume(TokenType::TruncateStackRange) {
            let mut lhs:Option<NodeExpr> = None;
            let mut rhs: Option<NodeExpr> = None;
            if let Some(int) = self.try_consume(TokenType::IntLit) {
                lhs = Some(NodeExpr::NodeExprIntLit{value:int});
            }else if let Some(reg) = self.try_consume(TokenType::Register) {
                lhs =Some(NodeExpr::NodeExprRegister{value:reg});
            }else {
                println!("Expected either integer or register as min value of range to truncate stack from.");
                std::process::exit(1);
            }
            if self.try_consume(TokenType::Comma).is_none() {
                println!("Expected Comma, found:{:?}",self.peek_token());
                std::process::exit(1);
            }

            if let Some(reg) = self.try_consume(TokenType::Register) {
                rhs =Some(NodeExpr::NodeExprRegister{value:reg});
            }else if let Some(int) = self.try_consume(TokenType::IntLit) {
                rhs =Some(NodeExpr::NodeExprIntLit{value:int});


            }else {
                println!("Expected either integer or register as max value of range to truncate stack from.");
                std::process::exit(1);
            }
            return Some(NodeInstruction::NodeInstructionTruncateStackRange{lhs:lhs.unwrap(),rhs:rhs.unwrap()})

        }
        None
    }

    pub fn parse_malloc(&mut self) -> Option<NodeInstruction> {
        if let Some(_malloc_tok) = self.try_consume(TokenType::Malloc) {
            if let Some(int) = self.try_consume(TokenType::IntLit) {
                return Some(NodeInstruction::NodeInstructionMalloc {
                    value: NodeExpr::NodeExprRegister{value:int}
                })               
            }else if let Some(reg) = self.try_consume(TokenType::Register) {
                return Some(NodeInstruction::NodeInstructionMalloc {
                    value: NodeExpr::NodeExprRegister{value:reg}
                })
            }else {
                println!("Expected either integer or register to allocate memory.");
                std::process::exit(1);
            }        
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

            if self.try_consume(TokenType::Comma).is_none() {
                println!("Expected Comma, found:{:?}",self.peek_token());
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

            if self.try_consume(TokenType::Comma).is_none() {
                println!("Expected Comma, found:{:?}",self.peek_token());
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

    pub fn parse_ret(&mut self) -> Option<NodeInstruction> {
        if let Some(_ret) = self.try_consume(TokenType::Return) {
            return Some(NodeInstruction::NodeInstructionReturn)
        }   
        None
    }

    pub fn parse_label(&mut self) -> Option<(String,usize)> {
        if let Some(_label_tok) = self.try_consume(TokenType::Label) {
            if let Some(label_name) = self.try_consume(TokenType::Ident) {
                if let Some(_) = self.labels.get(&label_name.value.clone().unwrap()) {
                    println!("Cannot defined lable with name `{:?}` as it is already defined.",label_name.value.clone().unwrap())
                }
                if let Some(_colon) = self.try_consume(TokenType::Colon) {
                    return  Some((label_name.value.clone().unwrap(),self.instruction_counter));                    
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



    #[allow(unused_assignments)]
    pub fn parse_movf(&mut self) -> Option<NodeInstruction> {
        if let Some(_movf_tok) = self.try_consume(TokenType::Movf) {
            let mut lhs:Option<NodeExpr> = None;

            if let Some(register_tok) = self.try_consume(TokenType::FloatRegister) {
                lhs = Some(NodeExpr::NodeExprRegister{value:register_tok});
            }else if let Some(int_tok) = self.try_consume(TokenType::Float) {
                let register_tok = get_register_from_number(int_tok.value.clone().unwrap().parse::<i32>().unwrap());
                if register_tok.is_none() {
                    println!("Invalid Register.");
                    std::process::exit(1);
                }        
                lhs = Some(NodeExpr::NodeExprIntLit{value:int_tok})
            }else {
                println!("Expected either register or register number to move flaot into. \nGot:{:?}",self.peek_token());
                std::process::exit(1);
            }
            let lhs = lhs.unwrap();
            if self.try_consume(TokenType::Comma).is_none() {
                println!("Expected Comma, found:{:?}",self.peek_token());
                std::process::exit(1);
            }    

            if let Some(f_lit) = self.try_consume(TokenType::Float) {
                return Some(NodeInstruction::NodeInstructionMovf {
                    lhs,
                    rhs:NodeExpr::NodeExprFloat{value:f_lit}
                });
            }else if let Some(register2_tok) = self.try_consume(TokenType::FloatRegister) {
                return Some(NodeInstruction::NodeInstructionMovf {
                    lhs,
                    rhs:NodeExpr::NodeExprRegister{value:register2_tok}
                });

            }else {
                println!("Expected either register or number value to move float into.");
                std::process::exit(1);
            }
        }
        None 
    }




    pub fn parse_addf(&mut self) -> Option<NodeInstruction> {
        if let Some(_add_tok) = self.try_consume(TokenType::Addf) {
            if let Some(register_tok) = self.try_consume(TokenType::FloatRegister) {
                if self.try_consume(TokenType::Comma).is_none() {
                    println!("Expected Comma, found:{:?}",self.peek_token());
                }
                if let Some(f_lit) = self.try_consume(TokenType::Float) {
                    return Some(NodeInstruction::NodeInstructionAddf {
                        lhs: NodeExpr::NodeExprRegister{value:register_tok},
                        rhs:NodeExpr::NodeExprFloat{value:f_lit}
                    });
                }else if let Some(register2_tok) = self.try_consume(TokenType::FloatRegister) {
                    return Some(NodeInstruction::NodeInstructionAddf {
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


    pub fn parse_subf(&mut self) -> Option<NodeInstruction> {
        if let Some(_sub_tok) = self.try_consume(TokenType::Subf) {
            if let Some(register_tok) = self.try_consume(TokenType::FloatRegister) {
                if self.try_consume(TokenType::Comma).is_none() {
                    println!("Expected Comma, found:{:?}", self.peek_token());
                }
                if let Some(f_lit) = self.try_consume(TokenType::Float) {
                    return Some(NodeInstruction::NodeInstructionSubf {
                        lhs: NodeExpr::NodeExprRegister { value: register_tok },
                        rhs: NodeExpr::NodeExprFloat { value: f_lit },
                    });
                } else if let Some(register2_tok) = self.try_consume(TokenType::FloatRegister) {
                    return Some(NodeInstruction::NodeInstructionSubf {
                        lhs: NodeExpr::NodeExprRegister { value: register_tok },
                        rhs: NodeExpr::NodeExprRegister { value: register2_tok },
                    });
                } else {
                    println!("Expected either float register or float literal to subtract from register.");
                    std::process::exit(1);
                }
            } else {
                println!("Expected float register to subtract from.");
                std::process::exit(1);
            }
        } else {
            None
        }
    }




    pub fn parse_mulf(&mut self) -> Option<NodeInstruction> {
        if let Some(_mul_tok) = self.try_consume(TokenType::Mulf) {
            if let Some(register_tok) = self.try_consume(TokenType::FloatRegister) {
                if self.try_consume(TokenType::Comma).is_none() {
                    println!("Expected Comma, found:{:?}", self.peek_token());
                }
                if let Some(f_lit) = self.try_consume(TokenType::Float) {
                    return Some(NodeInstruction::NodeInstructionMulf {
                        lhs: NodeExpr::NodeExprRegister { value: register_tok },
                        rhs: NodeExpr::NodeExprFloat { value: f_lit },
                    });
                } else if let Some(register2_tok) = self.try_consume(TokenType::FloatRegister) {
                    return Some(NodeInstruction::NodeInstructionMulf {
                        lhs: NodeExpr::NodeExprRegister { value: register_tok },
                        rhs: NodeExpr::NodeExprRegister { value: register2_tok },
                    });
                } else {
                    println!("Expected either float register or float literal to multiply into register.");
                    std::process::exit(1);
                }
            } else {
                println!("Expected float register for multiplication.");
                std::process::exit(1);
            }
        } else {
            None
        }
    }

    pub fn parse_divf(&mut self) -> Option<NodeInstruction> {
        if let Some(_div_tok) = self.try_consume(TokenType::Divf) {
            if let Some(register_tok) = self.try_consume(TokenType::FloatRegister) {
                if self.try_consume(TokenType::Comma).is_none() {
                    println!("Expected Comma, found:{:?}", self.peek_token());
                }
                if let Some(f_lit) = self.try_consume(TokenType::Float) {
                    return Some(NodeInstruction::NodeInstructionDivf {
                        lhs: NodeExpr::NodeExprRegister { value: register_tok },
                        rhs: NodeExpr::NodeExprFloat { value: f_lit },
                    });
                } else if let Some(register2_tok) = self.try_consume(TokenType::FloatRegister) {
                    return Some(NodeInstruction::NodeInstructionDivf {
                        lhs: NodeExpr::NodeExprRegister { value: register_tok },
                        rhs: NodeExpr::NodeExprRegister { value: register2_tok },
                    });
                } else {
                    println!("Expected either float register or float literal to divide into register.");
                    std::process::exit(1);
                }
            } else {
                println!("Expected float register for division.");
                std::process::exit(1);
            }
        } else {
            None
        }
    }


    pub fn parse_modf(&mut self) -> Option<NodeInstruction> {
        if let Some(_mod_tok) = self.try_consume(TokenType::Modf) {
            if let Some(register_tok) = self.try_consume(TokenType::FloatRegister) {
                if self.try_consume(TokenType::Comma).is_none() {
                    println!("Expected Comma, found:{:?}", self.peek_token());
                }
                if let Some(f_lit) = self.try_consume(TokenType::Float) {
                    return Some(NodeInstruction::NodeInstructionModf {
                        lhs: NodeExpr::NodeExprRegister { value: register_tok },
                        rhs: NodeExpr::NodeExprFloat { value: f_lit },
                    });
                } else if let Some(register2_tok) = self.try_consume(TokenType::FloatRegister) {
                    return Some(NodeInstruction::NodeInstructionMod {
                        lhs: NodeExpr::NodeExprRegister { value: register_tok },
                        rhs: NodeExpr::NodeExprRegister { value: register2_tok },
                    });
                } else {
                    println!("Expected either float register or float literal to perform modulus operation.");
                    std::process::exit(1);
                }
            } else {
                println!("Expected float register for modulus operation.");
                std::process::exit(1);
            }
        } else {
            None
        }
    }

    pub fn parse_getflag(&mut self) -> Option<NodeInstruction> {
        if let Some(_getflag_tok) = self.try_consume(TokenType::GetFlag) {
            let lhs = {
                if let Some(reg) = self.try_consume(TokenType::Register) {
                    NodeExpr::NodeExprRegister{value:reg}
                }else if let Some(rint) = self.try_consume(TokenType::IntLit) {
                    NodeExpr::NodeExprIntLit{value:rint}
                } else {
                    println!("Expected register to get flag into, found {:?}",self.peek_token());
                    std::process::exit(1);
                }
            };
            if self.try_consume(TokenType::Comma).is_none() {
                println!("Expected `,`, found {:?}",self.peek_token());
                std::process::exit(1);
            }
            let rhs = {
                if let Some(reg) = self.try_consume(TokenType::Register) {
                    NodeExpr::NodeExprRegister{value:reg}
                }else if let Some(rint) = self.try_consume(TokenType::IntLit) {
                    NodeExpr::NodeExprIntLit{value:rint}
                }else if let Some(flag) = self.try_consume(TokenType::Flag) {
                    NodeExpr::NodeExprFlag{value:flag}
                }
                else {
                    println!("Expected flag to get, found {:?}",self.peek_token());
                    std::process::exit(1);
                }
            };
            return Some(NodeInstruction::NodeInstructionGetFlag{lhs,rhs});
        } 
        None
    }
    pub fn parse_getsp(&mut self) -> Option<NodeInstruction> {
        if let Some(_getsp_tok) = self.try_consume(TokenType::GetStackPointer) {
            let lhs = {
                if let Some(reg) = self.try_consume(TokenType::Register) {
                    NodeExpr::NodeExprRegister{value:reg}
                }else if let Some(rint) = self.try_consume(TokenType::IntLit) {
                    NodeExpr::NodeExprIntLit{value:rint}
                } else {
                    println!("Expected register to get stack pointer into, found {:?}",self.peek_token());
                    std::process::exit(1);
                }
            };
            
            return Some(NodeInstruction::NodeInstructionGetStackPointer{lhs});
        } 
        None
    }


    pub fn parse_write(&mut self) -> Option<NodeInstruction> {
        if self.try_consume(TokenType::Write).is_none() { return None }
        if let Some(reg) = self.try_consume(TokenType::Register) {
            return Some(NodeInstruction::NodeInstructionWrite {
                value: NodeExpr::NodeExprRegister{value:reg}
            })
        }else if let Some(int_lit) = self.try_consume(TokenType::IntLit) {
            return Some(NodeInstruction::NodeInstructionWrite {
                value: NodeExpr::NodeExprIntLit{value:int_lit}
            })
        }else {
            println!("Expected either register or integer literal for write, found {:?}",self.peek_token());
            std::process::exit(1);
        }
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

            if let Some(inst_sub) = self.parse_sub() {
                return Some(inst_sub);
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

            if let Some(inst_mov) = self.parse_movf() {
                return Some(inst_mov);
            }
            if let Some(inst_add) = self.parse_addf() {
                return Some(inst_add);
            }

            if let Some(inst_sub) = self.parse_subf() {
                return Some(inst_sub);
            } 

            if let Some(mul) = self.parse_mulf() {
                return Some(mul)
            }
            if let Some(div) = self.parse_divf() {
                return Some(div)
            }
            if let Some(inst_mod) = self.parse_modf() {
                return Some(inst_mod)
            }

            if let Some(logial) = self.parse_logical() {
                return Some(logial)
            }
            if let Some(inst_display) = self.parse_display() {
                return Some(inst_display);
            } 
            if let Some(inst_displayf) = self.parse_displayf() {
                return Some(inst_displayf);
            } 
            if let Some(inst_displayc) = self.parse_displayc() {
                return Some(inst_displayc);
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
            if let Some(call) = self.parse_call() {
                return Some(call)
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
            if let Some(ss) = self.parse_setstack() {
                return Some(ss)
            }
            if let Some(sfsp) = self.parse_setfromsp() {
                return Some(sfsp)
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
            if let Some(ret) = self.parse_ret() {
                return Some(ret);
            } 
            if let Some(getflag) = self.parse_getflag() {
                return Some(getflag);
            }
            if let Some(getsp) = self.parse_getsp() {
                return Some(getsp);
            }
            if let Some(tsr) = self.parse_truncstackrange() {
                return Some(tsr);
            }
            if let Some(write) = self.parse_write(){
                return Some(write);
            }
            else {
                break;
            }
        }
        None
    } 

  pub fn parse_builtin(&mut self) -> Option<NodeBuiltin> {
        if let Some(_builtin_tok) = self.try_consume(TokenType::BuiltinStart) {
            if let Some(builtin_ident) = self.try_consume(TokenType::Ident) {
                let builtin_ident = builtin_ident.value.unwrap();
                match builtin_ident.as_str() {
                    "import" => {
                        if self.try_consume(TokenType::LParen).is_none() {
                            println!("Expected ( after @import, found {:?}",self.peek_token());
                            std::process::exit(1);
                        }
                        if let Some(string) = self.try_consume(TokenType::StringLit) {
                            if self.try_consume(TokenType::RParen).is_none() {
                                println!("Expected ( to close @import function, found {:?}",self.peek_token());
                                std::process::exit(1);   
                            }  
                            return Some(NodeBuiltin::NodeBuiltinImport {
                                value: NodeExpr::NodeExprStringLit{value:string},
                            })

                        }else {
                            println!("Expected string in @import, found {:?}",self.peek_token());
                            std::process::exit(1);
                        }
                    }
                   

                    "loadstring" | "loadstringn" => {
                        if self.try_consume(TokenType::LParen).is_none() {
                            println!("Expected ( after @loadstring, found {:?}",self.peek_token());
                            std::process::exit(1);
                        }
                        if let Some(string) = self.try_consume(TokenType::StringLit) {
                            if self.try_consume(TokenType::RParen).is_none() {
                                println!("Expected ( to close @loadstring function, found {:?}",self.peek_token());
                                std::process::exit(1);   
                            }  

                            return Some(NodeBuiltin::NodeBuiltinLoadString{
                                value: NodeExpr::NodeExprStringLit{value:string},
                                load_len: builtin_ident.as_str() == "loadstringn"
                            })

                        }else {
                            println!("Expected string in @loadstring, found {:?}",self.peek_token());
                            std::process::exit(1);
                        }
                    }
                    _ => unreachable!()
                  
                }
            }else {
                println!("Expected a builtin function type, found token {:?}",self.peek_token());
                std::process::exit(1);
            }
        }
        None
    }


    pub fn parse(&mut self)  {
      //-> Option<Vec<NodeBuiltin>,(HashMap<String,NodeLabel>)> 
        while self.peek_token().is_some() {
            if let Some(builtin) = self.parse_builtin() {
                match builtin{
                    NodeBuiltin::NodeBuiltinLoadString { value, load_len } => {
                       match value {
                           NodeExpr::NodeExprStringLit { value } => {
                                for v in value.value.as_ref().unwrap().chars() {
                                    self.instructions.push(
                                        NodeInstruction::NodeInstructionPush{
                                            value:NodeExpr::NodeExprIntLit{
                                                value:Token {
                                                    value: Some((v as u8).to_string()),
                                                    token_type: TokenType::IntLit,
                                                }
                                            }
                                        }
                                    );
                                }
                                if !load_len { continue }
                                // Push length of string 
                                    self.instructions.push(
                                        NodeInstruction::NodeInstructionPush{
                                            value:NodeExpr::NodeExprIntLit{
                                                value:Token {
                                                    value: Some(value.value.unwrap().len().to_string()),
                                                    token_type: TokenType::IntLit,
                                                }
                                            }
                                        }
                                    )
                           }
                           _ => unreachable!()
                       } 
                    }
                    _ => self.builtins.push(builtin)
                }
            }else  if let Some((name,labelindex)) = self.parse_label() {
                self.labels.insert(name,labelindex);
            }else if let Some(inst) = self.parse_inst() {
                self.instructions.push(inst);
                self.instruction_counter += 1;
            }
            else {
                println!("Undefined instruction: {:?}",self.peek_token());
                std::process::exit(1);
            }
        } 
    }

    pub fn peek_token(&self) -> Option<Token> {
        return self.tokens.get(self.index).cloned()
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

pub fn get_register_from_number(num:i32) -> Option<InstructionParamType> {
    if num < 0 {
        println!("Expected register or register number, found negative integer.");
        std::process::exit(1);
    }
    match num {
        0 => Some(REGA),
        1 => Some(REGB),
        2 => Some(REGC),
        3 => Some(REGD),
        4 => Some(RESERVEREGISTER),
        _ => None
    }
}
