
const REGISTERS:[&str;4] = ["rax","rbx","rcx","rdx"];

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum TokenType {
    Halt,
    Mov,
     Add,
     Sub, 
    Mul,
    Div,
    Mod,
    Display,
     Push,
     PushRegister,
     Pop,
     
    Jump,
    JumpIfZero,
    JumpIfNotZero,
    JumpIfEqual,
    JumpIfNotEqual,
    JumpIfGreater,
    JumpIfLess,

    Compare,

    GetFromStack,
    GetFromStackPointer,

    Malloc,
    GetMemory,
    SetMemory,


    Label,
    EndLabel,
    Return,
    IntLit,
    Ident,
    Register,

    Colon,
    Comma,
   

    Or,
    And,
    Not,
    Xor,
    Nand,

    TruncateStack,
    
}

#[derive(Debug,Clone,PartialEq)]
pub struct Token {
   pub token_type: TokenType,
   pub value:Option<String>,
}


impl Token {
    fn process_word(word: String) -> Token {
        match word.as_str() {
            "halt" => Token { token_type: TokenType::Halt, value: None },
            "mov" => Token { token_type: TokenType::Mov, value: None },
            "add" => Token { token_type: TokenType::Add, value: None },
            "sub" => Token { token_type: TokenType::Sub, value: None },
            "display" => Token { token_type: TokenType::Display, value: None },
            "push" => Token { token_type: TokenType::Push, value: None },
            "pushr" => Token { token_type: TokenType::PushRegister, value: None },
            "pop" => Token { token_type: TokenType::Pop, value: None },
            "jmp" => Token { token_type: TokenType::Jump, value: None },
            "jz" => Token { token_type: TokenType::JumpIfZero, value: None },
            "jnz" => Token { token_type: TokenType::JumpIfNotZero, value: None },
            "je" => Token { token_type: TokenType::JumpIfEqual, value: None },
            "jne" => Token { token_type: TokenType::JumpIfNotEqual, value: None },
            "jg" => Token { token_type: TokenType::JumpIfGreater, value: None },
            "jl" => Token { token_type: TokenType::JumpIfLess, value: None },
            "cmp" => Token { token_type: TokenType::Compare, value: None },
            "getfromstack" => Token { token_type: TokenType::GetFromStack, value: None },
            "getfromstackpointer" => Token { token_type: TokenType::GetFromStackPointer, value: None },
            "malloc" => Token { token_type: TokenType::Malloc, value: None },
            "getmem" => Token { token_type: TokenType::GetMemory, value: None },
            "setmem" => Token { token_type: TokenType::SetMemory, value: None },
            "label" => Token { token_type: TokenType::Label, value: None },
            "endlabel" => Token {token_type:TokenType::EndLabel,value:None},
            "ret" => Token {token_type:TokenType::Return, value:None},
            "mul" => Token {token_type:TokenType::Mul,value:None},
            "div" => Token {token_type:TokenType::Div,value:None},
            "or" => Token {token_type:TokenType::Or,value:None},
            "and" => Token {token_type:TokenType::And,value:None},
            "not" => Token {token_type:TokenType::Not,value:None},
            "nand" => Token {token_type:TokenType::Nand,value:None},
            "xor" => Token {token_type:TokenType::Xor,value:None},
            "truncstack" => Token {token_type:TokenType::TruncateStack,value:None},
            "mod" => Token {token_type:TokenType::Mod,value:None},
            _ => { 
                let is_reg = REGISTERS.iter().find(|x| ***x == *word.to_lowercase().as_str());
                if let Some(reg) = is_reg {
                    return Token { token_type: TokenType::Register, value: Some(reg.to_string())};
                }else {
                    return Token { token_type: TokenType::Ident, value:Some(word)};
                }
            }, 
        }
    }
}



pub struct Tokenizer {
    input:String,
    char_index:usize,
}

impl Tokenizer {
    pub fn new(input:String) -> Self {
        return Self {
            input,
            char_index: 0,
        }
    }

    pub fn tokenize(&mut self)->Vec<Token> {
        let mut buf = String::new();
        let mut tokens: Vec<Token> = Vec::new();    

        while let Some(ch) = self.peek_char() {
            if ch.is_alphabetic() || ch =='_' {
                buf.push(ch);
                self.consume_char();

                while let Some(next_ch) = self.peek_char() {

                    if next_ch.is_alphanumeric() || next_ch == '_' {
                        buf.push(self.consume_char().unwrap());
                    } else {
                        break;
                    }
                }

                tokens.push(Token::process_word(buf.clone()));
                buf.clear();
            } else if ch.is_numeric() {
                buf.push(ch);
                self.consume_char();

                while let Some(next_ch) = self.peek_char() {
                    if next_ch.is_numeric() {
                        buf.push(self.consume_char().unwrap());
                    } else {
                        break;
                    }
                }

                tokens.push(Token {
                    token_type: TokenType::IntLit,
                    value: Some(buf.clone()), 
                });
                buf.clear();
            } else if ch.is_whitespace() {
                self.consume_char(); // Skip whitespace
                if !buf.is_empty() {
                    tokens.push(Token::process_word(buf.clone()));
                    buf.clear();
                }
            }else {
                match ch {
                    ':' => {self.consume_char();tokens.push(Token {token_type:TokenType::Colon,value:None});},
                    ',' =>  {self.consume_char(); tokens.push(Token {token_type:TokenType::Comma,value:None});},
                    ';' =>  { 
                        self.consume_char();
                        let mut is_comment = true;
                        while is_comment {
                            if let Some(c) = self.peek_char() {
                                if c == '\r' {
                                    self.consume_char();    
                                }
                                if let Some(nl) = self.peek_char() {
                                        if nl == '\n' {
                                            is_comment = false;
                                        self.consume_char();
                                        continue;
                                    }
                                }
                                self.consume_char();
                            }
                        } 
                    }
                    _ => panic!("Unrecognized token {:?}",ch),
                }
            }
        }
        
        return tokens
    }


  fn peek_char(&self) -> Option<char> {
        self.peek_char_offset(0)
    }

    fn peek_char_offset(&self, offset: usize) -> Option<char> {
        self.input.chars().nth(self.char_index + offset)
    }

    fn consume_char(&mut self) -> Option<char> {
        let ch = self.peek_char();
        if ch.is_some() {
            self.char_index += 1;
        }
        ch
    }
}
