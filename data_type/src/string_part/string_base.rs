pub fn string_base_mod() {
    println!("----- string -----");
    // 字符串 Byte集合，能将byte解析为文本
    // rust核心语言层面，只有一个字符串类型：字符串切片str（&str）
    // 字符串切片：对存储在其他地方的UTF-8编码的字符串的引用
    // 字符串字面值存储在二进制文件中，也是字符串切片
    // Sting 类型
    // - 来自标准库而不是核心语言
    // - 可增长、可修改、可拥有

    // 创建
    // String::new()
    let mut s1 = String::new();
    s1.push('w');
    println!("s1 {}", s1);

    // 使用初始值来创建String
    // - to_string()方法，可用于实现了Display trait 的类型，包括字符串字面值
    let name = "wsn";
    let mut s_name = name.to_string();
    let she_name = "qhy".to_string();
    println!("s_name is {s_name}");
    println!("she_name is {she_name}");

    // 使用from函数 String::from("wsn")
    let he_name = String::from("wsz");
    println!("he_name is {he_name}");

    // push_str() 方法：把一个字符串切片附加到String
    s_name.push_str("帅");
    println!("s_name is {s_name}");
    // push() 方法 将单个字符附加到String中
    s_name.push('B');
    println!("s_name is {s_name}");

    // 拼接字符串
    // + 加号前是String 加号后是String引用，这个引用会被强制转化为&str切片类型
    // 只能把&str添加到String
    let s1 = String::from("wsn");
    let s2 = String::from("酷");
    let s3 = s1 + &s2; // 执行完后s1失去所有权，将s2的切片复制到s1最终将所有权给到s3
    println!("s3 is {s3}");

    let s4 = String::from("w");
    let s5 = String::from("s");
    let s6 = String::from("n");
    let s7 = s4 + "-" + &s5 + "-" + &s6;
    println!("s7 is {s7}");

    let s4 = String::from("w");
    let s5 = String::from("s");
    let s6 = String::from("n");
    let s8 = format!("{} - {} - {}", s4, s5, s6);
    println!("s8 is {s8}");

    // rust的String不支持索引类型的访问
    // String是对Vec<u8>的包装
    // len()

    let test_str1 = String::from("Wsn帅").len();
    println!("len is {test_str1}");

    // rust中看待字符串有三种方式
    // Byte - 字节
    // Scalar Values - 标量值
    // Grapheme Clusters 字形簇（最接近所谓的字母）
    let w = "王绍楠";

    for i in w.bytes() {
      println!("{i}");
    }

    for i in w.chars() {
      println!("{i}");
    }

    // 字形簇方式，rust没实现，相对复杂


    // 切割String
    let n = "我是王sn呀";

    let item = &n[0..6]; // 必须沿着字符的边界切割，不然会报错
    println!("item is {item}");



    println!();
}
