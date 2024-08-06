pub fn characters_mod() {
  println!("----- characters module ------");
  // 字符类型
  // 一个字符形 是4byte(32bit) 大小 - 一个unicode的标量值
  // unicode 范围
  // U+0000 ～ U+D7FF
  // U+E000 ~ U+10FFFF
  // 注意用单引号表示字符类型
  let c: char = 'c';
  let z = 'ℤ'; 
 println!("c is: {}", c);
 println!("z is: {}", z);
 println!();
}