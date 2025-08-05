//  creating some function to learn function variations

//   func

fn add(x:u32 , y:u32) ->u32 { // the output sshould be u32
    x+y
}
//ret val using return keyword
fn sub(x:u32 , y:u32) ->u32 { // the output sshould be u32
    let  z=x-y;
    return z;
}

// a non returning function
fn non_ret() ->! {
    loop {
        println!("loops");
    }
}
//panic
fn crasher(){
    panic!("code crashed ")
}

pub fn caller(){
    let x:u32= 1233;
    let y:u32= 1233;
    let plus =add (x,y);
    let min = sub(x,y);
    println!("plus {plus} ,  min{min}");
    // non_ret()
    // crasher()
}
