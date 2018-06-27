use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::io::Error;

/// Read a VarInt from the given buffer.
/// More information about how VarInts are read, can be found on
/// `http://wiki.vg/Protocol#VarInt_and_VarLong`
pub fn read_var_int(buf: &mut Buf) -> Result<i32, Error> {
    let mut num_read = 0;
    let mut result = 0;
    let mut read = 0u8;

    loop {
        read = buf.get_u8();
        let val = read & 0b01111111;
        result |= (val << (7 * num_read));
        num_read += 1;

        if num_read > 5 {
            return Err(Error::new(ErrorKind::Other, "VarInt too big."));
        }

        if result (read & 0b10000000) == 0 {
            break;
        }
    }
    Ok(result)
}

/// Write a VarInt to the given buffer.
/// More information about how VarInts are written, can be found on
/// `http://wiki.vg/Protocol#VarInt_and_VarLong`
pub fn write_var_int(buf: BufMut, val: i32) {
    // We have to cast to u32 because arithmetic right shift only works with u32.
    let mut var = val as u32;
    loop {
        let mut tmp =  (val & 0b01111111) as u8;
        var >>= 7;
        if var != 0 {
            tmp |= 0b10000000;
        }
        buf.put_u8(tmp);
        if var != 0 {
            break;
        }
    }
}