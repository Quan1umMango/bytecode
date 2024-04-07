
const REGISTERS:[&str;4] = ["rax","rbx","rcx","rdx"];
const FLOAT_REGISTERS:[&str;4] = ["fa","fb","fc","fd"];

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
    StringLit,
    Register,
    Float, 
    FloatRegister,
    
    Colon,
    Comma,
   

    Or,
    And,
    Not,
    Xor,
    Nand,

    TruncateStack,
    SetFromStackPointer,

    Movf,
    Addf,
    Subf, 
    Mulf,
    Divf,
    Modf,
    Displayf,
    PushFloatRegister,
    PopFloat,

    DisplayChar,

    BuiltinStart, // '@'
    LParen,
    RParen,
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
            "pushr" | "pushreg" => Token { token_type: TokenType::PushRegister, value: None },
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
            "getfromstackpointer" | "getfromsp" => Token { token_type: TokenType::GetFromStackPointer, value: None },
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
            "setfromsp" => Token {token_type:TokenType::SetFromStackPointer,value:None},

            "movf" => Token { token_type: TokenType::Movf, value: None },
            "addf" => Token { token_type: TokenType::Addf, value: None },
            "subf" => Token { token_type: TokenType::Subf, value: None },
            "mulf" => Token {token_type:TokenType::Mulf,value:None},
            "divf" => Token {token_type:TokenType::Divf,value:None},
            "modf" => Token {token_type:TokenType::Modf,value:None},
            "displayf" => Token { token_type: TokenType::Displayf, value: None },
            "pushrf" | "pushregf" => Token { token_type: TokenType::PushFloatRegister, value: None },
            "popf" => Token { token_type: TokenType::PopFloat, value: None },
            "displaychar" | "displayc" | "putc" => Token {token_type: TokenType::DisplayChar, value:None},
            _ => { 
                let is_reg = REGISTERS.iter().find(|x| ***x == *word.to_lowercase().as_str());
                let is_freg = FLOAT_REGISTERS.iter().find(|x| ***x == *word.to_lowercase().as_str());
                if let Some(reg) = is_reg {
                    return Token { token_type: TokenType::Register, value: Some(reg.to_string())};
                } if let Some(freg) = is_freg {
                    return Token { token_type: TokenType::FloatRegister, value: Some(freg.to_string())}
                }
                else {
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

   
pub fn tokenize(&mut self) -> Vec<Token> {
    let mut buf = String::new();
    let mut tokens: Vec<Token> = Vec::new();
    let mut is_float = false;
    let mut is_string = false;

    while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.consume_char();
                continue;
            }
        if ch.is_alphabetic() || ch == '_' {
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
        } else if ch == '-' {
            buf.push(ch);
            self.consume_char();
            while let Some(next_ch) = self.peek_char() {
                if next_ch.is_numeric() {
                    buf.push(self.consume_char().unwrap());
                } else if next_ch == '.' {
                    if is_float {
                        println!("Tokenization Error.\nInvalid Floating Point number: Floating point numbers cannot contain more than one period (.) ");
                        std::process::exit(1);
                    }
                    is_float = true;
                    buf.push(self.consume_char().unwrap());
                } else {
                    break;
                }
            }
            if buf.is_empty() {
                println!("Cannot have empty negative value.");
                std::process::exit(1);
            }
            if is_float {
                tokens.push(Token {
                    token_type: TokenType::Float,
                    value: Some(buf.clone()),
                });
            } else {
                tokens.push(Token {
                    token_type: TokenType::IntLit,
                    value: Some(buf.clone()),
                });
            }
            buf.clear();
            is_float = false;
        } else if ch.is_numeric() {
            buf.push(ch);
            self.consume_char();

            while let Some(next_ch) = self.peek_char() {
                if next_ch.is_numeric() {
                    buf.push(self.consume_char().unwrap());
                } else if next_ch == '.' {
                    if is_float {
                        println!("Tokenization Error.\nInvalid Floating Point number: Floating point numbers cannot contain more than one period (.) Got number: {:?}", buf);
                        std::process::exit(1);
                    }
                    is_float = true;
                    buf.push(self.consume_char().unwrap());
                } else {
                    break;
                }
            }
            if is_float {
                tokens.push(Token {
                    token_type: TokenType::Float,
                    value: Some(buf.clone()),
                });
            } else {
                tokens.push(Token {
                    token_type: TokenType::IntLit,
                    value: Some(buf.clone()),
                });
            }
            is_float = false;
            buf.clear();
        } else {
        
            match ch {
                ':' => {
                    self.consume_char();
                    tokens.push(Token { token_type: TokenType::Colon, value: None });
                }
                ',' => {
                    self.consume_char();
                    tokens.push(Token { token_type: TokenType::Comma, value: None });
                }
                '(' => {
                    self.consume_char();
                    tokens.push(Token { token_type: TokenType::LParen, value: None });
                    }
                ')' => {
                    self.consume_char();
                    tokens.push(Token { token_type: TokenType::RParen, value: None });
                    }
                ';' => {
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
                '@' => {
                        self.consume_char();
                        tokens.push(Token {token_type:TokenType::BuiltinStart, value:None});
                    } 
                '"' => {
                        self.consume_char();

                            is_string = true;
                        while let Some(c) = self.peek_char() {
                            if c == '\0' {
                               tokens.push(Token {token_type: TokenType::StringLit,value: Some(buf.clone())});
                                buf.clear();
                                self.consume_char();
                                is_string = false;
                                break;
                            }else if c == '"' {
                                tokens.push(Token {token_type: TokenType::StringLit,value: Some(buf.clone())});
                                buf.clear();
                                is_string = false;
                                self.consume_char();
                                break;
                            }else {
                                buf.push(c);
                                self.consume_char();
                            }
                            
                        }
                    }
                 _ => panic!("Unrecognized token {:?}", ch),
            }
        }
    }
    if is_string {
            println!("Did not find closing \".");
            std::process::exit(1);
        }
    tokens
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
