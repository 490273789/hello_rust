pub fn characters_mod() {
  println!("----- characters module ------");
  // 字符类型
  // 4 byte(32bit) 大小
  // 表示一个Unicode标量值（Unicode Scalar Value）
  // unicode 范围
  // U+0000 ～ U+D7FF
  // U+E000 ~ U+10FFFF
  // 注意用单引号表示字符类型
  let a: char = 'a';
  let z = 'ℤ';
  let c = '国';
  println!("c is: {c}");
  println!("z is: {z}");
  println!("z is: {c}");

  println!("---- end -----");
  println!()
}