pub struct ByteBuf {
    data: Vec<u8>,
    read_pointer: usize,
    write_pointer: usize
}

impl ByteBuf {

    pub fn new() -> ByteBuf {
        return ByteBuf {
            data: vec![],
            read_pointer: 0,
            write_pointer: 0
        }
    }


    //// Write functions ////

    pub fn write_u8(value: u8) {

    }

    pub fn write_i32(value: i32) {

    }

    pub fn write_i64(value: i64) {

    }

    pub fn write_var_int(value: i32) {

    }

    //// Read functions ////

    pub fn read_u8() -> u8 {

    }

    pub fn read_i32() -> i32 {

    }

    pub fn read_i64() -> i64 {

    }

    pub fn read_var_int() -> i32 {

    }
}


