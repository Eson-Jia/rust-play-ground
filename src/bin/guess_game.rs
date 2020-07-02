use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("hello");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("input a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("input error");
                continue
            },
        };
        println!("you guess:{}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("bingo");
                break;
            }
            Ordering::Less => println!("less"),
            Ordering::Greater => println!("greater"),
        };
    }
}
