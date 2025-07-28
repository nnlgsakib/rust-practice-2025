// enum


#[derive(Debug,PartialEq)]
enum col{
    Red,
    Green,
    Blue,
    RGBA(u8,u8,u8)
}


pub fn col_def(){
    //dec enums
    let col:col = col::Blue;
    let col:col = col::Green;
    let col:col = col::Red;
    let col:col = col::RGBA(2,34,5);
    println!("{:?}",col);


    //partial equal
    println!("{}",col::Red == col::Green);
    println!("{}",col::Red == col::Red);

    //option
    let x:Option<i32>  = None;
    let x:Option<i32> = Some(-273);
    println!("options{:?}",x);

    //result
     let res:Result<u32,String>  = Ok(23);
    let res:Result<u32,String> = Err("DING DING DONG".to_string());
    println!("{:?}",res)
}