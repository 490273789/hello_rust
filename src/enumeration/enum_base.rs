#[derive(Debug)]
enum Message {
    Quit,
    Move { x: u32, y: u32 },
    Write(String),
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

  let res =  match m {
      Message::ChangeColor(a, b, c) => {
        println!("{} {} {}", a ,b,c);
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

    // Option<T> 
    // Some()
    // None
    println!();
}
