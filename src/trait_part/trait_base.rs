use std::fmt::{Debug, Display};

use hello_rust::{Summary, Tweet};

pub fn trait_base_mod() {
    println!("----- trait base -----");
    // trait 告诉Rust编译器 某种类型具有哪些可以与其他类型共享的功能
    // 抽象的定义共享行为
    // trait bounds （约束）：泛型类型参数指定为实现了特定行为的类型

    // 和其他语言的interface类似，但有区别
    // 定义：把方法签名放在一起，来定义实现某种目的所必需的一组行为
    // - 关键字：trait
    // - 只有方法签名，没有具体实现
    // - trait可以有多个方法，每个方法签名占一行，以；结尾
    // - 实现了该trait的类型必须提供具体的方法实现

    // 与类型方法类似，不同之处：
    // impl Xxxx for Summary{}

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // 实现trait的约束
    // 可以在某个类型上实现某个trait的前提条件是：
    // - 这个类型 或 这个trait 是在本地crate里面定义的

    // trait参数 - impl Trait  fn notify (item: impl Summary) {}
    // trait参数 - impl Trait  fn notify (item: impl Summary + Display) {}

    // Trait bound:用于复杂的情况 fn notify<T: Summary> (item: T) {}

    println!();
}

fn notify<T: Summary + Display, U: Debug + Clone>(a: T, b: U) -> String {
    String::from("a")
}

// 传入trait等同于上面的写法
// 返回trait
fn notify1<T, U>(a: T, b: U) -> impl Summary
where
    T: Summary + Display,
    U: Debug + Clone,
{
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
