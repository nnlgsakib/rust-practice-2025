
pub fn scaler_types() {
    // scaler types returns a single type  value like bool:"true or false"

    //  signed int
    let a: i8 = 1;
    let b: i32 = 2;
    let c: i64 = 3;
    let d: i128 = 4;
    let e: isize = 43474785;
    println!("{a},{b},{c},{d},{e}");
    // unsigned int
    let f: u8 = 1;
    let g: u32 = 374;
    let h: u64 = 33443;
    let i: u128 = 33444;
    let j: usize = 3772347;
    println!("{f},{g},{h},{i},{j}");
    //float
    let k: f32 = 48.43;
    let l: f64 = 8549.344;

    println!("{k},{l}");
    // bool
    let a_1: bool = true;
    //char
    let c: char = 'a';
    println!("{c} is a char = {}", a_1);
    //type conversion
    let z: i128 = 37483;
    let u: u32 = 484;
    let i = u + (z as u32);
    println!("{i}");

    //min & max

    let int_max: i32 = i32::MAX;
    let int_min: i32 = i32::MIN;
    println!("max of int32 is {} and min of i32 is {}", int_max, int_min)
}
