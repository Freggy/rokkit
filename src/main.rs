extern crate tokio;
extern crate bytes;
extern crate core;

//use tokio::net::TcpListener;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::io::Error;
use std::io::ErrorKind;
use std::io::Cursor;
use std::vec::Vec;

mod var;

fn main() {
    //let addr = "127.0.0.1:1337".parse().unwrap();
    //let listener = TcpListener::bind(&addr).expect("Unable to bind to address.");

    let mut vec: Vec<u8> = vec![];
    let mut val = 1337;

    let mut c = Cursor::new(vec);


    var::write_var_int(&mut vec, val);
    println!("{:?}", vec);

    let d = var::read_var_int(&mut c).unwrap();

    println!("{}", d)



    //write_var_int(1337);
}