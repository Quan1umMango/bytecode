mod instruction;
mod conversions;
mod util_macros;
mod vm; 
mod commands;
mod constants_and_types;
mod tokens;
mod parser;

mod generator;
mod assembler;


fn print_bytecode(code:Vec<[u8;constants_and_types::COMMAND_SIZE]>) {
    use constants_and_types::*;
    for line in code.iter() {
        println!("{:?}  {:?} {:?}",&line[0..COMMAND_NAME_SIZE],&line[COMMAND_NAME_SIZE..COMMAND_NAME_SIZE+DESTINATION_SIZE],&line[COMMAND_NAME_SIZE+DESTINATION_SIZE..]);
    } 
}

fn main() {
    use assembler::Basm;
    let filename = "test.basm".to_string();
    Basm::run_file(filename);

}
