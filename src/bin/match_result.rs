use std::fs::File;
use std::io::{ErrorKind, Read, Write};

fn main(){
    let mut f =  match File::open("1.txt") {
        Ok(f) => f,
         Err(err)=>match err.kind(){
             ErrorKind::NotFound => match File::create("1.txt"){
                 Ok(fc)=>fc,
                 Err(e)=>panic!("Tried to create file but there was a problem: {:?}",e),
             },
             other_error =>panic!(""),
        },
    };
    let mut buff = String::new();
    f.write(b"hello,world");
    f.read_to_string( & mut buff);
    println!("{}",buff);
}