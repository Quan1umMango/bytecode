use crate::{tokens::*,parser::*,generator::*};

pub struct Basm;

impl Basm {
    pub fn run_string(input:String) {
        let tokens = Tokenizer::new(input).tokenize();
        let mut parsed = Parser::new(tokens);
        parsed.parse();
        let mut generator = Generator::new(parsed.builtins,parsed.labels);
        generator.generate();
        generator.vm.register_start();
        let bc = generator.vm.get_raw_byte_code();
        Basm::run_raw_string(bc); 
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


        
    pub fn run_raw_string(s:String) {
        let mut vm = crate::vm::VM::from_raw_instructions(s);
        vm.eval_raw();
    }
}
