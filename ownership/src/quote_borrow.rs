use std::ops::Add;

pub fn quote_borrow() {
  println!("---- 引用与借用 -----");
  // & 引用符号，允许你引用某些值，而不是取得其所有权
  // 结论：引用就是没有所有权的指针
  let s1 = String::from("hello");
  println!("The address of s1 is {:p}", &s1);
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

  // 别名和可变性不可同时存在
  // 别名：通过不同的变量访问同一数据
  // 别名数据： 可悲多个变量访问的一块数据

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

  // 引用就是无所有权的指针

  // 不可以拥有一个可变引用和一个不可变引用

  // 指针安全原则：数据不应同时“被别名引用”和“具有可变性”

  // 悬空引用
  // - 悬空指针 - Danging Pointer：一个指针引用了内存中的某个地址，而这个内存可能已经释放并分配给其他人使用了
  // - rust的这些规则就可以保证永远不会有悬空指针的问题


  // rust通过“借用检查器”确保引用的安全性
  // 变量对数据有三种权限
  // 1、读（R）：数据可以被复制到另一个位置
  // 2、写（W）：数据可以被修改
  // 3、拥有（O）：数据可以被移动或释放
  // 这些权限在运行时不存在，尽在编译器内部存在
  // 默认情况下，变量读起数据具有读/拥有权限
  // 如果一个变量被注释为let mut ，那么他还具有写 的权限
  // 关键：引用可以临时移出这些权限

  let mut x = Box::new(5);
  println!("The x address is {:p}", x);
  let a: i32 = *x;
  *x += 1;
  let r1 = &x;
  println!("The r1 address is {:p}", r1);
  let b = **r1;

  let mut r2 = &*x; // 只读的引用（没有所有权）和r1一样
  // let mut r2 = &mut *x; // 所有权会转移
  println!("The r2 address is {:p}", r2);

  println!("The x is {x}");
  let c = *r2;
  // *r2 += 1;
  println!("The c is {c}");
  println!("The r2 is {r2}");
  println!();
}

// 把引用作为函数参数的这个行为叫做借用
fn calculate_length1(s: &String) -> usize {
  println!("The value of s is {}", s);
  let address_in_s = s as *const String;
  println!("s 存储的内容{:p}", address_in_s); // s存的内存地址，指向s1
  println!("s 的地址{:p}", &s); // s自己的内存地址
  // 在这里s的ptr属性指向 s1的引用地址， s1的引用地址指向堆中的值
  s.len()
}

// 可变引用
fn calculate_length2(s: &mut String) -> usize {
  s.push_str(", world");
  s.len()
}
