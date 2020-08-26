fn main(){
    fn test(){
        println!("inner test")
    }
    fn test1(a:String){
        println!("inner test {}",a)
    }
    test()
}

fn test(){
    println!("outer test")
}