// case based matching
// called match statement

pub fn match_st(){
    let x = 1;
    // multi defined single match
    match x {
        1 => println!("one"),
        2=> println!("two"),
        _ => println!("unable to match")
    }

    //multicase
    match x {
        2|4|6 => println!("even num"),
        _ => println!("odd num")
    }
    //range based multicase
    match x {
        2..3 =>println!("num found in range"),
        1..=10=>println!("between 1 n 10"),
        _=> println!("NOT IN RANGE")
    }
}