struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

#[derive(Debug)]
struct Run;

pub fn struct_category_mod() {
    println!("------ struct_category_mod ------");
    println!("----- tuple struct -----");
    let black = Color(0, 0, 0);
    println!("The value of black is {} {} {}", black.0, black.1, black.2);
    let point = Point(0, 0, 0);
    println!("The value of point is {} {} {}", point.0, point.1, point.2);
    println!();

    println!("------ unit-like(无字段)struct ------");
    // 主要作用可以定义trite的一些实现
    let run = Run;
    println!("The value of run is {:#?}", run);

    println!("----- struct_category_mod end ------");
    println!()
}
