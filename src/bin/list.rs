use std::collections::LinkedList;

fn main() {
    let mut l:LinkedList<String> = LinkedList::new();
    l.push_back(String::from("one"));
    print!("{:?}",l);
}