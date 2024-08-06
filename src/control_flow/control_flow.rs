use rand::Rng;
pub fn control_flow_mod() {
    println!("----- 流程控制 control flow -----");
    println!("--- if 表达式");
    // 允许你根据条件来执行不同的代码分支， 这个条件必须是bool类型
    // 与条件相关联的代码块就叫做分支（arm）
    let number = rand::thread_rng().gen_range(1..=50);
    if number % 2 == 0 {
        println!("number is divisible by 2");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 2,3");
    }

    // 如果是用来多于1个else if ，那么最好使用match来重构代码
    // if else 是个表达式
    let x = rand::thread_rng().gen_range(1..50);
    let count = if x == 2 { 2 } else { 10 };
    println!("count is {count}");
    println!();
    println!("--- loop 循环");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
          break counter * 2;
        }
    };
    println!("loop result is {result}");
    println!();

    println!("--- while 循环");
    let mut i = 3;
    while i != 0 {
      println!("i is {i}");
      i -= 1;
    }
    println!();

    println!("--- for循环遍历集合");
    // while和loop也可以用来循环集合，大师易错且效率低
    let arr = [10,20,30,40];

    for element in arr.iter() {
      println!("arr element is {element}");
    }

    println!("-- Range (1..4)");
    for number in (1..4).rev() {
      println!("{number}!");
    }
    println!("LIFTOFF!");

    println!("------ end ------");
    println!();
}
