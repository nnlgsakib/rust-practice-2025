use std::ops::Mul;

#[derive(Debug)]
struct cord_pints {
    x: i32,
    y: i32,
    z: i32,
}

// struct method impl

impl cord_pints {
    //associated funcs -- strict methods
    fn clc() -> cord_pints {
        cord_pints {
            x: 34,
            y: 343,
            z: 34,
        }
    }

    // methods
    fn clc_2(&mut self, x: i32, y: i32, z: i32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    // calculating volume of the space
    fn clc_vol(&self) -> i32 {
        self.x * self.y * self.z
    }
}

pub fn all_caller() {
    // asso method
    let mut o_1 = cord_pints::clc();
    println!("{:?}", o_1);
    //changing value of cord y
    let o_2 = o_1.clc_2(333, 2, 433);
    println!("{:?}", o_2);

    //calculating the volume of the 3 cord values
    let o_3 = o_1.clc_vol();
    println!("THE VOLUME OF 3 CORD {}", o_3)
}
