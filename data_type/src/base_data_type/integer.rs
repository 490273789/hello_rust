pub fn integer_mod() {
  println!("----- integer - 整型！ -----");
  // i 表示有符号整型，u表示无符号整型
  // 8bit  i8: -2⁷ ~ 2⁷-1 (-167 ~ 156) u8: 2⁸-1 (0 ~ 255)
  // 16bit: i16: -2¹⁵ ~ 2¹⁵ -1 << 15 ~ 1 << 15 - 1;  u16: 1 << 16
  // i32 u32 32bit
  // i64 u64 64bit
  // i128 u128 128bit
  // arch isize usize 跟操作系统的架构有关：如果是32位操作系统，size就是32，64位操作系统，size就是64
  // 数字字面值	                    例子
  // Decimal (十进制)	            98_222
  // Hex (十六进制)	                0xff
  // Octal (八进制)	                0o77
  // Binary (二进制)	                0b1111_0000
  // Byte (单字节字符)(仅限于u8)       b'A'
  let i8_max = i8::MAX;
  println!("i8 max value is: {i8_max}"); // 127
  let i8_min = i8::MIN;
  println!("i8 min value is: {i8_min}"); // -128
  let byte = b'A';
  println!("The value of byte is {byte}");

  let price: i32 = 999;
  println!("price is {price}");

  println!("---- end -----");
  println!()
}