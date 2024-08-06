// use std::fs::File;

pub fn unwrap_mod() {
  println!("----- unwrap -----");

  // let file = File::open("hello.txt");
  // let file = match file {
  //   Ok(f) => f,
  //   Err(error) => panic!("Error open file {:#?}", error)
  // };

  // 下面这段代码相当于上面的代码
  // 如果结果是Ok 则返回Ok里面的值
  // 如果是error，则调用panic!
  // 不可以指定错误信息
  // let f1 = File::open("hello.txt").unwrap();
  // expect用法和unwrap相同，但是可以指定错误信息
  // let f2 = File::open("hello.txt").expect("Error open file");

  // println!("{:?}", f1);
  // println!("{:?}", f2);


  println!();
}