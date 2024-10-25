pub fn string_deep() {
  // 字符串的底层数据存储格式 [ u8 ]
  let hello = String::from("Hola");
  // Hola的长度是4个字节，因为每个字母在utf-8中的长度是1byte
  let hello = String::from("汉字");
  // 长度是6个字节，大部分汉字在utf-8中的长度是3个字节
  // 在这种情况下如果你访问 &hello[0] 是没有意义的，访问不了整个中字，这也是不用用索引访问的一个原因
  // 另一个原因是在rust中用索引访问字符串的复杂度也不一定是O(1)

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

  let item = &n[0..6]; // 必须沿着UTF-8字符的边界切割，不然程序会崩溃
  println!("item is {item}");
}