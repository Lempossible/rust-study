fn main() {
    println!("Hello, world!");

    // 变量绑定 int 类型
    let x = 5;
    let x = x + 1;
    println!("x = {}", x);
    let x = x * 2;
    println!("x = {}", x);

    // 变量绑定 char 类型
    // let x = 'a';

    // 变量绑定 bool 类型
    // let x = true;

    // 变量绑定字符串类型
    let x = "hello";
    println!("x = {}", x);

    // 变量绑定 元组类型
    let x = (1, 2);
    println!("x = {:?}, {:?}", x.0, x.1);

    // 变量绑定 数组类型, 数组是固定长度
    let x = [1, 2, 3];
    unsafe {
        println!(
            "x = {:?}, len = {}, capacity = {:?}",
            x,
            x.len(),
            *x.as_ptr().offset(1) as i32
        )
    }

    // function
    funca();
    let x = return_val();
    println!("x = {}", x);

    visit_array();

    ownership();

    let s = String::from("hello");
    string_ownership(s);
    // println!("s = {}", s); 所有权转移了，不能再使用 s

    // 引用类型可以在函数中使用, 但是不能修改引用的内容
    // 引用属于借用
    let s = String::from("hello");
    string_ownership_ref(&s);
    println!("s = {}", s);

    // 可变引用可以修改引用的内容
    let mut s = String::from("hello");
    mutate_string(&mut s);
    println!("s = {}", s);

    let s = String::from("hello world");
    let word = search_first_word(&s);
    println!("the first word = {}", word);

    // s.clear(); 同一个作用域只能有一个可变引用
    println!("the first word = {}", word);


    // 结构体初始化
    struct_init();


    // 结构体方法
    let rect = Rectangle::new(10, 20);
    println!("rect = {:?}", rect);
    println!("rect.area() = {}", rect.area());
}

fn funca() {
    println!("funca");
}

fn return_val() -> i32 {
    100
}

#[derive(Debug)]
struct MyStruct {
    x: i32,
    y: u32,
}

fn visit_array() {
    let x = [1,2,3];
    for i in x.iter() {
        println!("i = {}", i);
    }

    println!("x = {:?}", x);
    let my_struct = vec![
        MyStruct {
            x: 100,
            y: 200,
        },
    ];

    for item in my_struct.iter() {
        println!("item = {:?}, x = {}, y = {}", item, item.x, item.y);
    }
    println!("my_struct = {:?}", my_struct);
}

fn ownership() {
    // integer 类型是 Copy 类型, 所以 y 是 x 的副本, 而不是引用
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // String 类型不是 Copy 类型, 所以 s2 是 s1 浅拷贝 
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("s1 = {}, s2 = {}", s1, s2);

    let ms = MyStruct {
        x: 100,
        y: 200,
    };

    let ms2 = &ms;
    println!("ms2 = {:?}", ms2);
    println!("ms.x = {}", ms.x);
}

fn string_ownership(s: String) {
    println!("s = {}", s);
}

fn string_ownership_ref(s: &String) {
    println!("s = {}", s);
}

fn mutate_string(s: &mut String) {
    s.push_str(", world");
}

fn search_first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return "";
}

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: u32,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

trait Area {
    fn area(&self) -> u32;
}
impl Rectangle {
    // new 方法
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }
}
impl Area for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn struct_init() {
    let user = User {
        name: String::from("lene"),
        email: String::from("lene@example.com"),
        age: 30,
    };

    let user2 = User {
        name: String::from("hello"),
        .. user
    };

    // println!("user = {:?}", user); 结构体初始化后，user 就不能再使用了, 所有权转移了
    println!("user2 = {:?} name = {}, email = {}, age = {}", user2, user2.name, user2.email, user2.age);
}
