pub fn floating_point_mod() {
    println!("----- floating point numbers - 浮点型！ -----");
    // rust中有两种浮点型 i32和i64
    // 默认是 i64
    // 所有浮点型都是有符号的
    let x = 2.0;
    let y: f32 = 3.0;
    println!("x's data type is: {:.1}", x);
    println!("y's data type is: {:.1}", y);

    // addition
    let sum = 5 + 10;
    println!("addition is: {}", sum);

    // subtraction
    let sub = 95.5 - 4.3;
    println!("subtraction is: {}", sub);

    // multiplication
    let mul = 5 * 60;
    println!("multiplication is: {}", mul);

    // division
    let quotient = 56.7 / 32.2;
    println!("division quotient is: {}", quotient);
    let truncated = -5 / 3;
    println!("division truncated is: {}", truncated);

    // remainder
    let remainder = 43 % 5;
    println!("remainder is: {}", remainder);

    println!("---- end -----");
}
