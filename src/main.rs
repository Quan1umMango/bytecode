
#![recursion_limit = "10000"]
mod instruction;
mod conversions;
mod util_macros;
mod vm; 
mod constants_and_types;
mod tokens;
mod parser;

mod generator;
mod assembler;


fn main() {

    use assembler::Basm;
    use std::env;

    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Incorrect Usage.\nCorrect usage:\n\tbytecode <file-name>.basm");
        std::process::exit(1);
    }
    
    let filename = &args[1];
   Basm::run_file(filename.clone()); 

}

