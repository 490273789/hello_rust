// 命名：大驼峰
// 成员叫做“变体（Variants）”
#[derive(Debug)]
enum Message {
    Quit,                    // 不持有数据
    Move { x: u32, y: u32 }, // 持有结构体
    Write(String),           // 持有
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}
pub fn enum_base_mod() {
    println!("----- enum -----");
    let q = Message::Quit;
    let m = Message::Move { x: 12, y: 24 };
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangeColor(12, 32, 16);

    w.call();

    println!("q is {:#?}", q);
    println!("m is {:#?}", m);
    println!("w is {:#?}", w);
    println!("c is {:#?}", c);

    let res = match m {
        Message::ChangeColor(a, b, c) => {
            println!("{} {} {}", a, b, c);
            9
        }
        Message::Move { x, y } => {
            println!("{} {}", x, y);
            5
        }
        Message::Quit => 10,
        Message::Write(state) => {
            println!("{}", state);
            8
        }
    };
    println!("res is {}", res);

    // 常用的枚举： Option<T> 标识某个值可能存在或不存在
    // 来自标准库，Option机器Variants均在Prelude里
    // Some()
    // None
    // 这里面this_number 和 some_number是不同的类型，some_number是可能为 None
    // Option对比Null的优势
    // 1、表示出可能不存在
    // 2、Option<T>和T是不同类型
    // 3、强迫你处理这种情况
    let this_number = 5;
    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    println!("this_number:{this_number}, some_number: {some_number:#?}, absent_number: {absent_number:#?}");

    println!();
}
