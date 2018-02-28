extern crate md5;
extern crate rand;
extern crate easy_strings;

use std::env;
use std::ops::{Range};
use easy_strings::{EZString};


fn main() {
    let args: Vec<String> = env::args().collect();

    // string to int
    // let maxlen = args[1].parse::<i32>().unwrap();
    // let maxlen: usize = args[1].parse().unwrap();


    let maxlen: usize = match args.len() {
        2 => args[1].parse().unwrap(),
        _ => "10".parse().unwrap(),
    };


    if maxlen > 32 {
        println!("max length 32!", );
        std::process::exit(0);
    }


    for i in 0..10{
        let tuple = rand::random::<(f64, char)>();
        let origin = tuple.0.to_string();
        let digest = md5::compute(origin);

        let m1 = format!("{:x}", digest);
        let m2 = md5::compute(m1);
        let m3 = format!("{:x}", m2);
        let mut result = EZString::from(m3);

        println!("{:02}: {}", i+1, result.substr(Range{start:0, end: maxlen}).to_string());
        // println!("{:02}: {}", i+1, result.substr(0..maxlen).to_string());
    }
}
