#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter(state) => {
            println!("state quarter from {:#?}", state);
            10
        }
    }
}
pub fn match_base_mod() {
    println!("----- match -----");
    let c: Coin = Coin::Quarter(UsState::Alaska);
    let a: Coin = Coin::Quarter(UsState::Alabama);
    let p = Coin::Penny;
    let n = Coin::Nickel;
    let d = Coin::Dime;
    println!("c is {}", value_in_coin(c));
    println!("a is {}", value_in_coin(a));
    println!("p is {}", value_in_coin(p));
    println!("n is {}", value_in_coin(n));
    println!("d is {}", value_in_coin(d));
    match_test();
    if_let();
    println!();
}
// match匹配必须穷尽枚举
fn match_test() {
    let v = 0u8;
    match v {
        1 => println!("One"),
        2 => println!("Two"),
        _ => (),
    }
}

fn if_let() {
    let v = 0u8;
    match v {
        1 => println!("One"),
        _ => println!("Other"),
    }
    // 等价于上面的match，如果不需要else，也可以身略else
    if let 1 = v {
        println!("one");
    } else {
        println!("other");
    }
}

// workspace
// 一个workspace可以包含一个到多个crates，当代吗改变的时候，只有涉及的crates才需要重新编译
