pub fn tuple_mod() {
  println!("----- tuple 元组 ------");
  // 1、固定长度
  // 2、元素类型可以不同

  // 创建tup
  let tup = (0, "wsn", 2.2);
  println!("tup elements is {} {} {}", tup.0, tup.1, tup.2);
  let (ele1, ele2, ele3) = tup;
  println!("ele1, ele2, ele3 {ele1} {ele2} {ele3}");
  // Empty Tuple() // 是函数默认返回值
  // 元组获取元素 tup.index

  println!("----- end ------");
  println!();
}