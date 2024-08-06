pub fn integer_mod() {
  // i 表示有符号整型，u表示无符号整型
  // 8-bit	i8	u8 
  // 16-bit	i16	u16
  // 32-bit	i32	u32
  // 64-bit	i64	u64
  // 128-bit	i128	u128
  // arch	isize	usize - 依赖你运行的计算机架构，64位他就是i64， 32位的它就是i32
  // 数据范围
  // i8: -2**7 ～ 2**7 - 1 : -128 ~ 127
  // u8: 2 ** 8 - 1 : 0 ~ 255
  let i8_max = std::i8::MAX;
  println!("i8 max value is: {i8_max}"); // 127
  let i8_min = std::i8::MIN;
  println!("i8 min value is: {i8_min}"); // -128

  // 数字字面值	                    例子
  // Decimal (十进制)	            98_222
  // Hex (十六进制)	              0xff
  // Octal (八进制)	              0o77
  // Binary (二进制)	          0b1111_0000
  // Byte (单字节字符)(仅限于u8)    b'A'
}