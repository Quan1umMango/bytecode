use crate::constants_and_types::*;

#[macro_export]
macro_rules! to_binary_slice {
    ($t:ty, $num:expr) => {
        {
       let to_binary = {
         let max_power = (<$t>::MAX as f64).log2() as u64 as usize + 1;
                 
                let mut fin = Vec::new();
                
                for i in 0..max_power {
                if $num == 0 { fin.push (0); continue };
                let ok = 1u64.checked_shl(max_power as u32-1-i as u32);

                if ok.is_none() { continue }; 
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
                let n = 1u32.checked_shl($slice.len() as u32 -1-i as u32);
                if n.is_none() { continue; }
                num += 1 << ($slice.len() - 1 - i);
            }
            num
        }
    }
}


#[macro_export]
macro_rules! ones_complement {
    ($t:ty,$num:expr) => {
        {
            let (n,sign) = {
                if $num < 0 {
                    ($num*-1,1)
                }else {
                    ($num,0)
                }
            };
            let mut slice = to_binary_slice!($t,n);
            slice[0] = sign;
            for i in  0..slice.len() {
                if i == 0 {
                    continue;

                }
                slice[i] = {
                    if slice[i] == 0 {
                        1
                    }else {
                        0
                    }
                }
            }
            binary_slice_to_number!($t,slice)
        }
    }
}

#[macro_export]
macro_rules!  twos_complement {
    ($t:ty, $num:expr) => {
        {
            ones_complement!($t,$num)+1
        }
    }
}



#[macro_export]
macro_rules! integer_from_twos_complement {
    ($itype:ty,$utype:ty,$twos_comp:expr) => {
        {
            let twos_comp_slice = to_binary_slice!($utype,$twos_comp);
            let sign = if twos_comp_slice[0] == 1 {
                -1
            }else {
                1
            };
            let new_slice = &twos_comp_slice[1..];
            let num = binary_slice_to_number!($utype,new_slice);
            let ones_comp =  {
                if num != 0 {
                    num-1
                }else {
                   ones_complement!($utype,0 as $itype)
                }
            };
            sign as $itype  * ones_complement!($utype,ones_comp as $itype) as $itype
            
        }
    }
}  


#[macro_export]
macro_rules! to_float_repr {
    ($ftype:ty,$utype:ty,$num:expr) => {
        {
            // not doing this lol
            $num.to_bits()
        
        }
    }
}

#[macro_export]
macro_rules! binary_to_float {
    ($ftype:ty,$utype:ty,$num:expr) => {
        {
            let binary_slice = to_binary_slice!($utype,$num);
            let sign = binary_slice[0];

            let exponent = &binary_slice[1..9];
            let mantissa = &binary_slice[9..];

            let sign = if sign == 0 { 1.0 } else { -1.0 };

            let twos_power = {
                let num = binary_slice_to_number!($utype,exponent) as i32;
            
                2_f32.powi(num-127)
            };
            let mantissa_number = {
                let mut  fract_sum = 1.0; 
                
                    for i in 0..mantissa.len() {
                        if mantissa[i] == 0 { continue; }
                        fract_sum += 1.0/(2_f32.powf(i as f32+1.));   
                    }
                fract_sum
            };
            sign * twos_power * mantissa_number 
        }
    }
}

