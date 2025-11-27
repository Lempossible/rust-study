fn main() {
    println!("Hello, world!");

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    println!("{:?}", v);

    let mut v2 = vec![1, 2, 3];
    v2.push(4);
    println!("{:?}", v2);

    println!("{:?}", v2.get(2));
    for i in v2.iter() {
        println!("{}", i);
    }
    println!("{:?}", v2);

    // 遍历并修改元素
    for i in v2.iter_mut() {
        *i += 10;
    }
    println!("{:?}", v2);
    v2.pop();
    println!("{:?}", v2);
}
