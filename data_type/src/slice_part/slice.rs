pub fn slice_mod() {
    println!("----- slice -----");
    // rust中另外一种不持有所有权的类型就是切片（slice）
    // 切片就是存储一个指针和切片的长度，指向对应数据的起始位置

    let str = String::from("Hello World");

    // 有等号就是包含这个索引（<=），没有等号就不包含(<)
    let slice1 = &str[5..=10];
    // 截取到最后可以省略最后一个参数
    let slice2 = &str[5..];
    // 从0开始截取可省略第一个参数
    let slice3 = &str[..5];
    // 截取整个字符串
    let slice4 = &str[..];

    println!("slice1 is {slice1}");
    println!("slice2 is {slice2}");
    println!("slice3 is {slice3}");
    println!("slice4 is {slice4}");

    // 如果尝试从多字节的字符中创建切片，就会报错

    let first = first_world(&str[..]);

    println!("First world is \"{first}\".");

    // 对数组也可以同样的切片
    println!("------ slice end ------");
    println!();
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
