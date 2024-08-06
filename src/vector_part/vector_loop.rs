#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn vector_loop_mod() {
    println!("----- vector loop -----");

    let mut v = vec![100, 200, 300];
    // * 接引用符号
    for i in &mut v {
        *i += 50;
    }

    for i in v {
        println!("{i}");
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Hello")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("row is {:#?}", row);
    println!();
}
