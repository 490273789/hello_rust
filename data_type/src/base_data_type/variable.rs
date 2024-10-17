pub fn variable() {
  println!("----- variable - 变量！ -----");
  // 什么是类型：类型是二进制数据的一种约束行为，类型比起直接使用二进制的优势
  // 1、减少开发者心智负担
  // 2、安全

  // 常见类型的分类
  // 1、静态类型 - 编译时进行类型检查
  // 2、动态类型 - 运行时进行检查
  // 3、强类型 - 不允许进行隐式转换
  // 4、弱类型 - 允许进行隐式转换

  println!("rust有4种基本标量（scalar）类型");

  // 变量
  // 变量 - 默认是不可变的
  let x = 5;
  println!("The value of x is {x}");

  // mut 可变变量
  let mut y = 6;
  println!("The value of y is {:?}", y);
  y = 7;
  println!("The value of y is {:?}", y);

  // rust是静态强类型语言
  // 基本标量（scalar）类型
  // 1、integer - 整型
  // 2、floating-point-numbers - 浮点类型
  // 3、boolean - 布尔类型
  // 4、characters - 字符类型
  // 复合类型（Compound）类型 - 可以讲多个值组合在一个类型中
  // 1、tuple - 元组
  // 2、array - 数组

  // 强制类型转换
  let num = 3.1;
  let _num1 = num as i32; // 未使用的变量可以加_,消除命令行的报错
  // shadowing variables 影子变量 相当于创建了一个新的变量
  let num = 4; // 可以使用相同的变量名覆盖上一个变量
  println!("num is: {num}");

  // 常量
  // 1、不能使用mut 关键字
  // 2、必须标注类型
  // 3、常量可以在任何作用域内进行声明
  // 4、常量只可以绑定常量表达式，无法绑定到函数的调用结果或只能在运行时才能计算出的值。
  // 在运行期间，常量在其声明的作用域内一直有效
  // 常量使用全大写字母，每个单词间使用下划线链接
  const MAX_NUMBER: u32 = 32;
  println!("常量 MAX_NUMBER: {MAX_NUMBER}");

  const THREE_AND_A_BIT: f32 = 3.4028236;
  println!("常量 THREE_AND_A_BIT: {THREE_AND_A_BIT}");

  println!("---- end -----");
  println!()
}