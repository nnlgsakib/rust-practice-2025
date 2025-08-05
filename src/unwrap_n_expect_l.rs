// its a more smart way to handle error

use std::ptr::with_exposed_provenance;

pub fn u_e(){
    let x:Option<i32> =  Some(34);
    let val = x.unwrap();
    // expect
    println!("val is == {val}");
    let z = x.expect("its a None");
    println!("val is == {z}");



}