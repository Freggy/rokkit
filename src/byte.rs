use bytes::{BufMut};

// Implementation details provided by #mcdevs

mod byte {
    fn read_var_int(buf: &Buf) -> Result<i32, Error> {
        let mut num_read = 0;
        let mut result = 0;
        let mut read: u8 = 0;
        let mut cont = true;

        while (read & 0b10000000) != 0 {
            read = buf.get_u8();
            let val = read & 0b01111111;
            result |= val << (7 * num_read);
            num_read += 1;

            if num_read > 5 {
                return Err("VarInt too big.")
            }
        }
        Ok(result)
    }

    fn read_var_long(buf: &Buf) {

    }

    fn write_var_int(buf: &Buf) {

    }

    fn write_var_long(buf: &Buf) {

    }
}