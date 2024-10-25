pub fn quote_borrow() {
  println!("---- 引用与借用 -----");
  // & 引用符号，允许你引用某些值，而不是取得其所有权
  let s1 = String::from("hello");
  let len1 = calculate_length1(&s1); // 将引用值传给函数，而不是所有权，好处是，函数用完后就不需要再将所有权返回了
  println!("The length of {s1} is {len1}");

  // 可变引用
  let mut s2 = String::from("hello");
  let len2 = calculate_length2(&mut s2); // 如果需要对引用的只修改，需要传入可变引用
  println!("The length of {s2} is {len2}");

  // 解引用 *
  let x = 5;
  let y = &x;
  assert_eq!(5, x);
  assert_eq!(5, *y);
  // 在特定作用域内，对某一块数据，只能有一个可变的引用，多个会报错
  // 好处是防止编译时数据竞争
  // let s3 = &mut s2;
  // let s4 = &mut s2;
  // println!("s3 is {s3}, s4 is {s4}");

  // 新版编译器中（>= 1.31）引用作用域的结束位置从花括号变成最后一次使用的位置

  // 会发生数据竞争的三种行为
  // 1、两个或多个指针同时访问一个数据
  // 2、至少有一个指针用于写入数据
  // 3、没有任何机制来同步的对数据的访问

  // 不可以拥有一个可变引用和一个不可变引用

  // 悬空引用
  // - 悬空指针 - Danging Pointer：一个指针引用了内存中的某个地址，而这个内存可能已经释放并分配给其他人使用了
  // - rust的这些规则就可以保证永远不会有悬空指针的问题
  println!();
}

// 把引用作为函数参数的这个行为叫做借用
fn calculate_length1(s: &String) -> usize {
  // 在这里s的ptr属性指向 s1的引用地址， s1的引用地址指向堆中的值
  s.len()
}

// 可变引用
fn calculate_length2(s: &mut String) -> usize {
  s.push_str(", world");
  s.len()
}
