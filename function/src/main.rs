fn main() {
  function_mod();
}
pub fn function_mod() {
  println!("----- 函数 function -----");
  // 函数
  // 1、名称
  // snake case 所有字母小写 单词之间使用“_” 连接

  // 2、参数
  // parameters 形参 定义函数的参数
  // arguments 实参 调用函数实际传入的参数

  // 3、返回类型
  // rust是一个基于表达式的语言， 表达式会计算产生一个值
  // 表达式可以是语句的一部分 let a = 10， 10 就是一个表达式
  // 语句是执行一些动作的指令
  // 函数的定义也是语句
  // 语句不返回值，所以不可以使用let将一个语句赋给一个变量（和javascript不同）

  // 4、函数体

  // 1、语句（statements）：执行某些操作的指令，不返回值，里面会包含表达式
  // 2、表达式（Expression）：计算并返回一个结果，可以成为语句的一部分

  // 默认的返回值空tuple
  let no_return = function_params(10, 20); // 实参
  println!("The value of noReturn is {:?}", no_return);
  let five = five();
  println!("five: {five}");
  let num = plus_five(10);
  println!("The value of num is: {num}");

  println!("----- end -----");
  println!();
}
/*
 parameters
 x 形参，y 形参
 函数的定义是一个语句， 函数的调用是一个表达式
 函数体是由一系列语句组成，可由表达式结尾
 在函数中如果最后一行不写 “;” 则最后一行为表达式返回的值，如果写则会变成语句。
*/
fn function_params(x: u32, y: u32) {
  println!("x, y: {} {}", x, y);
  // 注意：rust中这个花括号也是表达式，会返回 a+2的值
  let y = {
    let a = 1;
    a + 2
  };
  println!("The value of y is {y}");
}

// 函数的返回值
fn five() -> i32 {
  // 函数的返回值：在 -> 符号后面声明函数的类型
  // 返回值：
  // 返回值就是函数体里面“最后一个表达式”
  // 若提前返回，需使用return关键字，并指定一个值
  // 大多数函数都是默认使用最后一个表达式作为返回值
  5
}

fn plus_five(x: u32) -> u32 {
  x + 5
}

// 发散函数 - 永远不返回值    -> !
// 会导致程序崩溃
