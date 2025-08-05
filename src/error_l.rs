// error handling or

use std::net::Incoming;

pub fn error_handling(){
    //panic: crashes the program if any error happens
    let mut val:i32 = 5;
    if val  == 0  {
        panic!("value is not acceptable")
    } else {
        println!("everyting is fine")
    }

    println!("{val}");
    //option based error handling
    // it retunes a value or None
    let some_dat:[i32; 4] = [23,4,5,43];
    let get_arr:Option<&i32> = some_dat.get(3);
    println!("{:?}",get_arr);
    //handling panic using result
    //creating a faulty div function which have no div by zero protection
    fn div(x:i32,y:i32)->Result<i32,String>{
        // so this function will ether return valid result i32  value or a string
        if y == 0{
            return Err("undiv".to_string())
        }else {
           Ok(x / y)
        }
    }
    //calling this func
    println!("{:?}",div(12,44))

}