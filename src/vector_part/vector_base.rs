pub fn vector_base_mod() {
    println!("----- vector -----");
    // 由标准库提供
    // 可存储多个值
    // 只能存储相同类型的数据
    // 值在内存中连续存放
    // 离开作用域后将会被清理

    // 创建
    let v = vec![1, 2, 3];
    println!("v is {:?}", v);

    // 添加
    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("v1 is {:?}", v1);

    //读取
    // 如果index超出边界，会导致程序崩溃
    let first = &v1[0];
    println!("The first element is {}", first);
    
    // 如果index超出边界，不会导致程序崩溃
    match v1.get(2) {
        Some(second) => println!("The second element is {}.", second),
        None => println!("There is none second element."),
    }

    println!();
}
