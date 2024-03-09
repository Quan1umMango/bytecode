use crate::{tokens::*,parser::*,generator::*};

pub struct Basm;

impl Basm {
    pub fn run_string(input:String) {
        let tokens = Tokenizer::new(input).tokenize();
        let parsed = Parser::new(tokens).parse().unwrap();
        let mut generator = Generator::new(parsed);
        generator.generate();
        generator.vm.eval();
    }

    pub fn run_file(file_name:String) {
       let s = match std::fs::read_to_string(file_name) {
            Err(err) => {
                println!("Error in reading file: {:?}",err);
                std::process::exit(1);
            }
            Ok(s)=> s
        };
        Basm::run_string(s);
    }

    pub fn run_raw_file(file_name:String) {
        let mut vm = crate::vm::VM::new();
        match vm.read_from_file(&file_name) {
            Err(err) => {
                println!("Error in reading bytecode file: {:?}",err);
                std::process::exit(1);
            }
            _ => ()
        }
        vm.eval();
    }
}
