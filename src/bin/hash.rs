use std::collections::HashMap;

fn main(){
 let mut m = HashMap::new();
    m.insert(String::from("key"),String::from("value"));
    print!("{:?}",m);
}