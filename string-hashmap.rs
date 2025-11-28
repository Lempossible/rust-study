use std::collections::HashMap;
fn main() {
    let tmp = String::from("temp");
    let s1 = String::from("hello "); // 从字符串字面值创建字符串
    let s2 = "hello"; // s2 是引用
    println!("{:p}", &s2);
    let s3 = " world";
    let s4 = s1 + s2; // s1 被移动了，不能再使用
    println!("{}", s4);

    // to_string() 是新分配内存，不会改变 s2 的引用
    let s5 = s2.to_string() + s3; // 引用和引用不能直接相加，需要先转换为 String 类型
    // String 和String 也不能直接相加
    let s6 = s5 + &tmp; //只能是 String + &str, tmp是&String, 但是编译器会强制转为&str
    println!("{}", s6);
    println!("{:p}", &s2);

    // format! 宏 不会夺取任何参数的所有权
    let s1 = String::from("hello ");
    let s2 = String::from("world");
    let s7 = format!("{}{}", s1, s2);
    println!("{}", s7);
    println!("{s1}");
    println!("{s2}");

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50, 30];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.entry(String::from("Red")).or_insert(20);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.insert(String::from("Red"), 30);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // get 方法返回一个 Option<&V> 类型，所以需要 unwrap() 方法获取值
    let red = scores.get(&String::from("Red")).unwrap();
    println!("{red}");
}
