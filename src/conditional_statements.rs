//conditional statements

pub fn con_st(){

    // Normal statement
    let x:i32=  2;
    if x % 2 == 0 {
        println!("{x} is even")
    }
    else{
        println!("{x} is odd")
    }

    // in var condition check

    let mut z:i32 = if x > 0 {
        1
    } else if x > 0  {
        -1
    }else { 0 };

    println!("z is {z}")
}


