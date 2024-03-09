use crate::constants_and_types::*;

#[macro_export]
macro_rules! to_binary_slice {
    ($t:ty, $num:expr) => {
        {
       let to_binary = {
         let max_power = (<$t>::MAX as f64).log2() as u64 as usize + 1;
                 
                let mut fin = Vec::new();
            for i in 0..max_power {
                if $num & (1 << (max_power-1 - i)) != 0 {
                    fin.push(1);
                }else {
                        fin.push(0);
                    }
            }
            fin
        };
        to_binary
        }
    }
}

#[macro_export]
macro_rules!  binary_slice_to_number {
    ($t:ty, $slice:expr) => {
        {
           let mut num: $t = 0;
            for i in 0..$slice.len() {
                if $slice[i] == 0 {
                    continue;
                }
                num += 1 << ($slice.len() - 1 - i);
            }
            num
        }
    }
}



pub fn command_binary_to_slice(code:CommandType) -> CommandBinary {
    return to_binary_slice!(CommandType,code).try_into().unwrap();
} 

pub fn combined_destination_binary_to_slice(dest:DestinationType) -> CombinedDestinationBinary {
    let mut fin = CombinedDestinationBinary::default();
    for i in 0..DESTINATION_SIZE*2 {
        if dest & (1 << (DESTINATION_SIZE*2-1-i)) !=0 {
            fin[i] = 1;
        }
    }

    fin

}

pub fn destination_binary_to_slice(dest:DestinationType) ->DestinationBinary {
    let mut fin =DestinationBinary::default();
    for i in 0..DESTINATION_SIZE {
        if dest & (1 << (DESTINATION_SIZE-1-i)) !=0 {
            fin[i] = 1;
        }
    }

    fin
}


