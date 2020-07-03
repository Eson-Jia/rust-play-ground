fn main(){
    let u_32:u32 = 212;
    println!("the u32 is {}",u_32);
    let b = true;
    println!("boolean:{}",b);
    let c = 'a';
    //tuple
    let tup:(i32,f64,u8)=(500,6.4,1);
    println!("the tuple value:{:?}",tup);
    // 元组支持解构操作
    let tup = (500,6.4,1);
    let (x,y,z) = tup;
    println!("the value x:{},y:{},z:{}",x,y,z);
    // 元组支持点操作符 . 操作
    let tup = (500,6.4,1);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    // 数组
    // 数组有固定长度而且在栈上申请内存
    let a =[1,2,3,4,5];
    // 声明方式为 [type:length]
    let a:[i32;5]= [1,2,3,4,5];
    // 创建数组,并使用相同值初始化
    let a =[3;5];
    // 访问数组元素
    let first = a[0];
    let second  = a[1];
    // 支持解构赋值
    let[a0,a1,a2,a3,a4] = a;
    println!("{},{}",a0,a1);
}