#[derive(Debug)]
struct Rectangle {
    width: u32,
    height:u32,
}

impl Rectangle {
    // 定义 method,self 指当前实例
    fn area(&self) -> u32{
        self.width * self.height
    }
    // method 有多个参数
    fn can_hold (&self,other: &Rectangle)-> bool{
        self.height > other.height && self.width > other.width
    }
    // associated functions 关联函数 我们之前遇到过 String::new() String::from()
    // 一般作为构造器返回一个 struct 实例
    fn square(size:u32) ->Rectangle{
        Rectangle{
            width:size,
            height:size,
        }
    }
}

// impl 块可以有多个
impl Rectangle {
    fn size(&self)->(u32,u32){
        (self.height,self.width)
    }
}

fn main(){
    let rect1 = Rectangle{
        width:30,
        height:30,
    };
    let rect2 = Rectangle{
        width:100,
        height:50,
    };
    println!("area:{}",rect1.area());
    println!("can {:#?} hold {:#?}:{}",rect2,rect1,rect2.can_hold(&rect1));
}