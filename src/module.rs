
use crate::mth;
pub fn caller(){
    let x:u32= 1233;
    let y:u32= 1233;
     let rn = mth::mul_div::ran{
         val:234,
         endcode:2233,
     };
    let add = mth::add(x,y);
    println!("{add}");
    let mul = mth::mul_div::mul(x,y);
    println!("{mul}");
    println!("str : {:?}",mth::add(rn.val , rn.endcode));
    println!("partial add {}" , mth::mul_div::partial_add(x,y))
}