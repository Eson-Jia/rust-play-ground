fn main() {
    //创建一个空 immutable vector
    let v: Vec<i32> = Vec::new();

    //创建一个有初始值的，immutable vector
    let v = vec![123,12];
    //创建一个 mutable vector,因为随后调用 push 函数并传入一个整数，所以能推导出元素类型为 i32
    let mut v = Vec::new();
    v.push(12);

    //访问 vector
    //两种方法 下标访问 get方法
    let mut v = vec![1,2,3,4,5];
    let first = v[0];
    let second  = v.get(1);

    // 两种方式对于下标越界有不同的行为 下标方式会 panic,方法会返回 No]ne
    let does_not_exist  = v[100];
    let does_not_exist= v.get(100);

    // 不能在 v 上同时使用可变引用和不可变引用
    // push 可能会导致内存重新分配，导致引用数组中第一个元素失效
    let mut v = vec![1,2,3,4,5];
    // 不变引用
    let first =  &v[0];
    v.push(6);
    println!("the first element is :{}",first);
}