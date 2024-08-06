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
    let p = Coin::Penny;
    let n = Coin::Nickel;
    let d = Coin::Dime;
    println!("c is {}", value_in_coin(c));
    println!("p is {}", value_in_coin(p));
    println!("n is {}", value_in_coin(n));
    println!("d is {}", value_in_coin(d));
    match_test();
    if_let();
    println!();
}

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
    // 等价于上面的match
    if let 1 = v {
        println!("one");
    } else {
        println!("other");
    }
}
