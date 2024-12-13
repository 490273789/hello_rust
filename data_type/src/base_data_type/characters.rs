pub fn characters_mod() {
    println!("----- characters module ------");
    // 字符类型
    // 4 byte(32bit) 大小
    // 表示一个Unicode标量值（Unicode Scalar Value）
    // unicode
    //  码点表示：Unicode 为每个字符分配一个唯一的码点，这个码点是一个数字，用于在逻辑上标识字符。Unicode 的码点范围从 0 到 0x10FFFF（即 1,114,111）。
    // 字节序列：Unicode 本身不定义如何将码点转换为字节序列，这是由不同的 Unicode 编码形式完成的。

    // 注意用单引号表示字符类型

    // utf-8 是unicode的一种实现方式
    // 对于码点 U+0000 到 U+007F（即 ASCII 字符），UTF-8 使用一个字节表示，与 ASCII 兼容。
    // 对于码点 U+0080 到 U+07FF，UTF-8 使用两个字节表示。
    // 对于码点 U+0800 到 U+FFFF，UTF-8 使用三个字节表示。
    // 对于码点 U+10000 到 U+10FFFF，UTF-8 使用四个字节表示。

    let a: char = 'a';
    let z = 'ℤ';
    let c = '国';
    println!("c is: {c}");
    println!("z is: {z}");
    println!("a is: {a}");

    println!("---- end -----");
    println!()
}
