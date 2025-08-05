// modulo , lit ,  bool , bit

use std::arch::is_riscv_feature_detected;
use std::pin::pin;

pub fn un_c() {
    // mod

    // so in rust remainder  != mod
    // mod only returns pos values but remainder can return neg values too
    let num_1: i32 = -24;
    let num_2: i32 = 3748;
    let mod_val: i32 = num_1 % num_2;
    println!("{num_1} % {num_2}={mod_val}");

    //Literals
    let eth =  1i32;
    let wei = 1e18;
    let sat = 233343333333333333usize;


    println!("got {eth} eth and as per smaller unit 1 eth = {wei} wei and my wallet also contains wort {sat} of btc ");

    //bitwise
    let num_z:i32 =  23;
    let num_z_2:i32  = 34;
    println!("num_z and num_z_2 == {:03b}",num_z & num_z_2);
    println!("num_z or num_z_2 == {:03b}",num_z | num_z_2);
    println!("!num_z == {:03b}",!num_z )
}
