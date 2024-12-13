pub fn string_base_mod() {
    println!("----- string -----");
    // 引入 #![allow(unused_variables)] 属性标记，该标记会告诉编译器忽略未使用的变量，不要抛出 warning 警告
    // 字符串 Byte集合，能将byte解析为文本
    // rust核心语言层面，只有一个字符串类型：字符串切片str（&str） - UTF-8
    // 字符串切片：对存储在其他地方的UTF-8编码的字符串的引用
    // 字符串字面值存储在二进制文件中，也是字符串切片

    // 字符是unicode类型 4 byte
    // 字符串是utf-8编码 1-4 byte，这样有助于降低字符串的占用空间

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello: {}, world: {}", hello, world);

    // Sting 类型 动态的字符串类型
    // - 来自标准库而不是核心语言
    // - 可增长、可修改、拥有所有权的UTF-8编码字符串
    // String类型是由栈中的堆指针、字符串的长度、字符串的容量共同组成
    // 创建
    // String::new()
    let mut s1 = String::new();
    s1.push('w');
    println!("s1 {}", s1);

    // 使用初始值来创建String
    // - to_string()方法，可用于实现了Display trait 的类型，包括字符串字面值
    let name = "wsn"; // 字符串字面值是不可变的，因为被硬编码到程序代码中
    let mut s_name = name.to_string(); // 将&str类型转换为String
    let she_name = "qhy".to_string();
    println!("s_name is {s_name}");
    println!("she_name is {she_name}");

    // String 转 &str
    let w_name = String::from("hello");
    let n_name = &w_name[..];
    println!("n_name {}", n_name);

    // 使用from函数 String::from("wsn")
    let he_name = String::from("wsz");
    println!("he_name is {he_name}");

    // 操作字符串
    // push_str() 方法：把一个字符串切片（&str）附加到String
    s_name.push_str("帅");
    println!("s_name is {s_name}");
    // push() 方法 将单个字符（char）附加到String中
    s_name.push('B');
    println!("追加：s_name is {s_name}");

    // 插入 insert
    // insert() 插入 char
    // insert_str() 插入字符串字面量
    // 第一个参数是要插入位置的索引， 第二个参数要插入的内容
    s_name.insert(1, '6');
    s_name.insert_str(1, "7");
    println!("插入： s_name is {s_name}");
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

    // 如果不希望字符串被转义用 r"" 或 r#""# 套起来即可

    let str = String::from(r"汉字");
    println!(" The value of str is {str}");
    // ASCII
    let byte_escape = "I'm saying hello \x71";
    println!(r"The value of \x71 is {byte_escape}");

    // unicode
    let byte_escape = "I'm saying hello \u{0065}";
    println!(r"The value of \u{{0065}} is {byte_escape}");

    // 这个字面量必须使用r##这种形式，因为我们希望在字符串字面量里面保留""
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    //  很多时候，我们的字符串字面量中用不到 Unicode 字符，只需要 ASCII 字符集。
    // 对于这种情况，Rust 还有一种更紧凑的表示法：字节串。用 b 开头，双引号括起来，比如 b"this is a byte string"。
    // 这时候字符串的类型已不是字符串，而是字节的数组 [u8; N]，N 为字节数。

    // 字节串的类型是字节的数组，而不是字符串了
    let byte_string: &[u8; 21] = b"this is a byte string";
    println!("A byte string: {:?}", byte_string);
    // 可以看看下面这串打印出什么
    let escaped = b"\x52\x75\x73\x74 as bytes";
    println!("Some escaped bytes: {:?}", escaped);
    // 字节串与原始字面量结合使用
    let raw_byte_string = br"\u{211D} is not escaped here";
    println!("{:?}", raw_byte_string);

    println!();
}
