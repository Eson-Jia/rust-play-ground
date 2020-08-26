
fn with_one_return() -> &'static str {
    "hello,world"
}

fn one_return_string()->String{
    String::from("hello")
}

fn with_multi_return() -> (i32,f32){
    (12,12.0)
}

fn main() {
    let one_return  = with_one_return();
    let (one,two) =with_multi_return();
    let the_return = one_return_string();
    println!("{},{},{},{}",one_return,one,two,the_return);
}