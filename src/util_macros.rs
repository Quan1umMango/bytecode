#[macro_export]
macro_rules! create_command_single_arg {
    ($dest:expr, $code:expr,$command_code:expr) => {
        {


            let (a,_) = get_destinations_slice($dest);
            $code[0..COMMAND_NAME_SIZE].copy_from_slice(&to_binary_slice!(CommandNameType,$command_code));
            add_destination_to_command_binary(&mut $code,(Some(a),None));

                $code.try_into().unwrap()
        }
    }
}

#[macro_export]
macro_rules! create_command_single_arg_full_dest {
    ($dest:expr, $code:expr,$command_code:expr) => {
        {


            let a = to_binary_slice!(DestinationType,$dest);
            $code[0..COMMAND_NAME_SIZE].copy_from_slice(&to_binary_slice!(CommandNameType,$command_code));
            add_full_destination_to_command_binary(&mut $code,a.try_into().unwrap());

                $code.try_into().unwrap()
        }
    }
}

#[macro_export]
macro_rules! create_command_two_arg {
    ($dest:expr, $code:expr,$command_code:expr) => {
        {


            let (a,b) = get_destinations_slice($dest);
            $code[0..COMMAND_NAME_SIZE].copy_from_slice(&to_binary_slice!(CommandNameType,$command_code));
            add_destination_to_command_binary(&mut $code,(Some(a),Some(b)));

                $code.try_into().unwrap()
        }
    }
}

#[macro_export]
macro_rules! create_command_no_arg {
    ( $code:expr,$command_code:expr) => {
        {

            $code[0..COMMAND_NAME_SIZE].copy_from_slice(&to_binary_slice!(CommandNameType,$command_code));
            $code.try_into().unwrap()
        }
    }
}

#[macro_export]
macro_rules! create_fcommand_two_arg {
    ($dest:expr, $code:expr,$command_code:expr) => {
        {


            let (a,b) = get_destinations_slice($dest);
            $code[0..COMMAND_NAME_SIZE].copy_from_slice(&to_binary_slice!(CommandNameType,$command_code));
            add_destination_to_command_binary(&mut $code,(Some(a),Some(b)));

                $code.try_into().unwrap()
        }
    }
}

#[macro_export]
macro_rules! jump {
    ($a:expr,$labels:expr,$insts:expr,$run_label_raw_inst:expr) =>  {
        {
               use crate::instruction::StringNumberUnion;
                let label_address = match $a {
                    StringNumberUnion::String(s) => {
                       if let Some(value) = $labels.get(s.into()) {
                                value.0
                        }else {
                            panic!("Cannot jump to label with name: {:?}  as it does not exist.",s);
                        }
                    },
                    StringNumberUnion::Num(n) => *n as usize
                };
                if let Some(_) = $insts.get(label_address) {
                    $run_label_raw_inst(label_address); 
                }else {
                    panic!("Cannot jump to address: {:?} as it does not exist.", label_address);
                }

        }
    }
}

#[macro_export]
macro_rules! parse_jump {
    ($jump_token_type:expr,$jump_node_inst:expr,$try_consume:expr) => {
        {
            if let Some(_jmp_tok) = $try_consume($jump_token_type) {
                use crate::parser::NodeInstruction::*;
                if let Some(label_name) = $try_consume(TokenType::Ident){ 

                    match $jump_node_inst {
                        NodeInstructionJump{value: ref mut value} |  
                            NodeInstructionJumpIfZero{value: ref mut value}|
                            NodeInstructionJumpIfNotZero{value: ref mut value}|
                            NodeInstructionJumpIfEqual{value: ref mut value}|
                            NodeInstructionJumpIfNotEqual{value: ref mut value}|
                            NodeInstructionJumpIfGreater{value: ref mut value}|
                            NodeInstructionJumpIfLess{value: ref mut value}
                            =>  {
                                *value = crate::parser::NodeExpr::NodeExprLabelName {
                                    value:label_name,
                                };
                            }, 
                        _ => {unreachable!()}
                    }
                    Ok(())
                }else {
                    println!("Expected register or number to jump.");
                    std::process::exit(1);
                }
            }else {
                Err(())
            }

        }               
    }
}
