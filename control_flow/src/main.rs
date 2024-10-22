use rand::Rng;

fn main() {
  if_use();
  loop_use();
  while_use();
  for_use();
}
fn if_use() {
  // 在rust中 if else 是表达式，有返回值
  // 如果是用来多于1个else if ，那么最好使用match来重构代码
  // if else 是个表达式
  println!("----- 流程控制 control flow -----");
  println!("--- if 表达式");
  // 允许你根据条件来执行不同的代码分支， 这个条件必须是bool类型
  // 与条件相关联的代码块就叫做分支（arm）
  let count = rand::thread_rng().gen_range(1..=100);
  let result = if count % 2 == 0 { 2 } else { 3 };
  println!("The value of result is {:?}", result);

  if count % 2 == 0 {
    println!("count is divisible by 2");
  } else if count % 3 == 0 {
    println!("count is divisible by 3");
  } else {
    println!("count is not divisible by 2,3");
  }
}

// break 停止loop循环
// continue 跳出背刺迭代
fn loop_use() {
  println!("--- loop 循环 ---");
  let mut counter = 0;
  let result = loop {
    counter += 1;
    if counter == 10 {
      // break 可以给loop 返回值
      break counter * 2;
    }
  };

  println!("loop result is {result}");

  // loop标签
  let mut count = 0;
  'counting_up: loop {
    println!("count = {}", count);
    let mut remaining = 10;
    loop {
      println!("remaining = {}", remaining);
      if remaining == 9 {
        break;
      }
      if count == 2 {
        break 'counting_up;
      }
      remaining -= 1;
    }
    count += 1;
  }
  println!("End count = {}", count);
  println!();


}

fn while_use() {
  println!("--- while 循环 ---");
  let mut i = 3;
  while i != 0 {
    println!("i is {i}");
    i -= 1;
  }
  println!();
}


fn for_use() {
  println!("--- for循环遍历集合 ---");
  // while和loop也可以用来循环集合，大师易错且效率低
  let arr = [10, 20, 30, 40];
  for element in arr {
    println!("arr element is {element}");
  }

  // for element in arr.iter() {
  //   println!("arr element is {element}");
  // }

  println!("-- Range (1..4)");
  // range 包前不包后
  for number in (1..4).rev() {
    println!("{number}!");
  }

  // 将温度在华氏度和摄氏度之间转换
  // 生成n 个斐波那契数列
  // 打印圣诞颂歌《十二天的圣诞节》的歌词，并利用歌曲中的重复部分

  println!("------ end ------");
  println!();
}