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
macro_rules! jump {
    ($dest:expr,$labels:expr,$run_label:expr) =>  {
        {
                let (a,_) = get_destinations($dest);
                let label_name = $labels.iter().find(|&x| {
                   if x.1.0 == a.into() {
                        return true
                    }
                    false

                });
                if label_name.is_none() {
                    let jmp_loc = get_destinations($dest);
                    //println!("{:?}",jmp_loc.0);
                   $run_label("".to_string(),false,jmp_loc.0 as usize,true);
                }else {
                    $run_label(label_name.unwrap().0.to_string(),false,0,false);
                }

        }
    }
}


