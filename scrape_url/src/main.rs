use std::fs;
use std::env;

fn main() {
    // rust中除了let/static/const/fn等少数是语句，绝大多数代码都是表达式
    // 表达式的特点是有返回值，如果活没有显式的给返回值吗，默认是返回unit
    // 接口编程和泛型编程
    // 变量的类型一般情况下可以智能推导，但是const和static变量必须声明类型
    // 默认变量是不可变的，加mut未可变
    // rust中函数是一等公民，可以作为参数或返回值

    // rust中的数据结构
    // 1、使用struct定义结构体
    // 2、使用enum定义标签联合体（tagged union）
    // 使用宏：#[derive(Debug, Copy, Clone)]
    // const 右值 rvalue，被编译后放在可执行文件的数据段，全局可访问
    // static 静态变量和常量一样全局可访问，他也被编入可执行文件数据段中。但是可被声明为可变

    // 1、空结构体不占用任何空间
    // 2、元组结构体，每个域都是匿名的，可以通过索引访问；
    // 3、普通结构体，每个域都有名字，可通过名称访问；
    let params: Vec<String> = env::args().collect();
    println!("&params[1], {}", &params[0]);
    let url = &params[1];
    let output = &params[2];
    for arg in env::args() {
        println!("{arg}");
    }
    // let url = "https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html";
    // let output = "rust.md";

    // unwrap(),在错误的时候会直接panic
    // ?在错误的时候会return Err
    println!("Fetching url: {url}");
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {output}");
    fib_loop(5);
    fib_while(5);
    fib_for(5);
}

fn common(mut a: i32, mut b: i32) -> [i32; 2] {
    let c = a + b;
    a = b;
    b = c;

    println!("value is {b}");
    [a, b]
}

// for循环实际上是loo的一个语法汤
/** 斐波那契额数列 */
fn fib_loop(n: u8) {
    println!("fib_loop");
    if n == 0 {
        println!("value is 0");
    }
    if n == 1 {
        println!("value is 1");
    }
    let mut a = 1;
    let mut b = 1;
    // let mut i: u8 = 2;
    let mut i = 2u8;
    loop {
        i += 1;
        println!("i is {i}");
        [a, b] = common(a, b);

        if i >= n {
            break;
        }
    }
    println!();
}

fn fib_while(n: u8) {
    println!("fib_while");
    let (mut a, mut b, mut i) = (1, 1, 2);
    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        println!("value is {b}");
    }
    println!();
}

fn fib_for(n: u8) {
    println!("fib_for");
    let (mut a, mut b) = (1, 1);
    for _i in 2..n {
        let c = a + b;
        a = b;
        b = c;
        println!("value is {b}");
    }
    println!();
}
