pub fn if_let(){
    // normal statement match
    let x:Option<u32> = Some(23);
    match x {
        Some(v) => println!("some {v}"),
        _=>{}
    }
    //if let based matching

    if let Some(v) = x{
        println!("IF LET {v}")
    }
    // let else
    let Some(v) = x else{
        panic!("x is none")
    };

    println!("v= {v}")
}