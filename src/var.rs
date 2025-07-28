pub fn var_practice() {
    //normal imut var
    let x: i32 = 100;
    // mut vat
    let mut y: i32 = 200;
    println!("initial value {}", y);
    // changing val of mutable y
    y = 32;
    println!("changed value {}", y);

    //undefined type var
    let z = 2.32;
    println!("this is a float {}", z);
    // predefining variable
    let n: i32;
    n = 323;
    println!("n is = {}", n);
    //dec cosnt
    const some_name: &str = "Rook";
    println!("Kind black destroyed the {}", some_name);

    let v_ec: Vec<_> = vec![2, 4, 3];
    println!("a vec {:?}", v_ec)
}
