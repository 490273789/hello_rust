use std::{fs::File, io::{self, Read}};


fn read_username_from_file() -> Result<String, io::Error> {
  // ? 运算符：传播错误的一种快捷方式
  // 如果Result 是Ok：Ok中的值就是表达式的结果，然后继续执行程序
  // 如果Result 是Err：err就是整个函数的返回值，就是使用了return
  let mut f = File::open("hello.txt")?;
  // 上面的写法是下面的简写方式
  // let mut f = match File::open("hello.txt") {
  //   Ok(file) => file,
  //   Err(e) => return Err(e)
  // };

  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)

}

// 进一步的简写
fn read_username_from_file1() -> Result<String, io::Error> {
  let mut s = String::new();
  File::open("hello.txt")?.read_to_string(&mut s)?;
  Ok(s)
}


pub fn error_operation_mod() {
  println!("----- error operation -----");
  let result = read_username_from_file();
  let result1 = read_username_from_file1();
  println!("result is {:?}", result);
  println!("result1 is {:?}", result1);

  // from函数：用于错误之间的转换
  // 被?所应用的错误，会隐士的被from函数处理
  // 当?调用from函数时：它所接收的错误类型会被转化为当前函数返回类型所定义的错误类型
  // 用于：针对不同错误原因，返回同一种错误类型
  // - 只要每个错误类型实现了转换为所返回的错误类型的from函数
  println!();
}