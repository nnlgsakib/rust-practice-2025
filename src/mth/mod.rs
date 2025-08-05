

pub fn add(x:u32 , y:u32) ->u32 { // the output sshould be u32
    x+y
}
//ret val using return keyword
pub fn sub(x:u32 , y:u32) ->u32 { // the output sshould be u32
    let  z=x-y;
    return z;
}
// module inside module
pub mod mul_div{
    pub fn mul(x:u32 , y:u32) ->u32 { // the output sshould be u32
        x*y
    }
    //ret val using return keyword
    pub fn div(x:u32 , y:u32) ->u32 { // the output sshould be u32
        let  z=x/y;
        return z;
    }

    pub fn partial_add(x:u32 , y:u32) -> u32{
        super::add(x,y)
    }
    //struct in module
    pub struct ran{
        pub(crate) val:u32,
        pub(crate) endcode:u32
    }
}


