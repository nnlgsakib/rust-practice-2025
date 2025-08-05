//loops:
    // infinity loop
    //for loop
    //while lopp
    //fo loop array
    //usize and range
    // FOR LOOP VECTOR
    //iter
    //return value
    //Labels


pub fn loop_l(){
    //infinity loop

    // limiting infinite loop
    let mut i = 0;

    loop{
        println!("life goes on n on");
        if i == 5 {
            break;
        }
        i += 1;

    }
    // while loop
    let mut j = 0;
    while j<= 5{
        println!("yngmi {j}");
        j+=1
    }

    //for loops
    for j in 0..=10{
        println!("destoy nga{j}")
    }

    // array loop
    let array = [1,2,43,43];
    for a in array{
        println!("{a}")
    }

    //usize and range
    let u = array.len();
    for z in 0..u{
        println!("arr = {}",array[1] )
    }

    // for loop vector
    let v = vec![23,4,3,4,3];
    for x_1 in v.iter() {
        println!("vector {x_1}")
    }

    for x_1 in v.iter() {
        println!("vector {x_1}")
    }
    //rreturn value

    let mut i_1 = 0;

    let l_1 = loop{
        println!("life goes on n on");
        if i_1 == 5 {
            break 3343;
        }
        i_1 += 1;

    };
    println!("{l_1}");

    //labels
    'outer: for o in 0..4 {
        'inner: for k in 0..34 {
            println!("nested loop {}, {}", o, k);
            if o == 2 && k == 1 {
                break 'outer;
            }
}
    }
}