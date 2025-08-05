// compound data types like array, tuple

pub fn cmp_type() {
    //tuple
    let tup: (bool, &str, i32) = (false, "nlg", 232);
    //destracture
    let (x, y, z) = tup;

    //ignring with _
    let (_, y, z) = tup;
    // empty tup
    let e_tup = ();
    println!("{:?}", e_tup);

    //arrays
    let nums: [i32; 4] = [1, 2, 3, 4]; // immutable array
    //acc elems
    println!("1st value  of array is {:?}", nums[0]);
    let mut m_array: [i32; 2] = [23, 34]; // mutable array
    println!("current array {:?}", m_array);
    m_array[1] = 343;
    println!("array after changing element 1 = {:?}", m_array);
    // gen arrray
    let g_array: [i32; 100] = [0; 100];
    println!("value of entire arrray is {:?}", g_array);

    // slice
    let s = &nums[0..3];
    println!("1st 3 elems is {:?}", s)
}
