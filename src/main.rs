extern crate tokio;
extern crate core;
extern crate rand;

//use tokio::net::TcpListener;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Cursor;
use std::vec::Vec;
use rand::random;

fn main() {
    //let addr = "127.0.0.1:1337".parse().unwrap();
    //let listener = TcpListener::bind(&addr).expect("Unable to bind to address.");

    let val = rand::random::<i32>();
    println!("myVar: {}", val);
    let v2 = 0b0101_0011_1001;

    let bytes = vec![(val >> 24), (val >> 16), (val >> 8), val];

    println!("{:?}", bytes);

    let first = bytes[0] << 24;
    let second = (bytes[1] & 0xFF) << 16;
    let third = (bytes[2] & 0xFF) << 8;
    let fourth = (bytes[3] & 0xFF);

    println!("first: {}", first);
    println!("sec: {}", second);
    println!("th: {}", third);
    println!("foruth: {}", fourth);


    let a = bytes[0] << 24 | (bytes[1]) << 16 | (bytes[2]) << 8 | (bytes[3] & 0xFF);


    for i in 0..1000 {
        let val = rand::random::<i32>();
        //println!("myVar: {}", val);
        let bytes = vec![(val >> 24), (val >> 16), (val >> 8), val];

        //println!("{:?}", bytes);

        let first = bytes[0] << 24;
        let second = (bytes[1]) << 16;
        let third = (bytes[2]) << 8;
        let fourth = (bytes[3]);

        //println!("first: {}", first);
        //println!("sec: {}", second);
        //println!("th: {}", third);
        //println!("foruth: {}", fourth);
        let a = bytes[0] << 24 | (bytes[1]) << 16 | (bytes[2]) << 8 | bytes[3] & 0xFF;
        println!("{}", val == a);

    }


    println!("{:?}", a);
    println!("{:b}", (val >> 16))

    //write_var_int(1337);
}