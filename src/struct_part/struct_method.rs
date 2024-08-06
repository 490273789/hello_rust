#[derive(Debug)]
struct Rectangle {
 width: u32,
 height: u32
}

// 一个结构体可以有多个impl 块
impl Rectangle {
  // 结构体方法的定义
  fn rectangle_area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }

  // 关联函数
  // 不把self作为第一个参数的函数（不是方法）
  fn square (size: u32) -> Rectangle {
    Rectangle{
      width: size,
      height: size
    }
  }
  
}

pub fn struct_method_mod() {
  println!("----- struct method -----");
  // struct的方法和函数的不同：
  // 方法是在struct、enum、trait的上下文中定义
  // 第一个参数是self，表示方法被调用的struct实例
  let rectangle = Rectangle{
    width: 10,
    height: 50
  };
  let rectangle1 = Rectangle{
    width: 8,
    height: 45
  };
  let rectangle2 = Rectangle{
    width: 10,
    height: 5
  };

  println!("Rectangle area is {}", rectangle.rectangle_area());
  println!("rectangle can hold rectangle1 {}", rectangle.can_hold(&rectangle1));
  println!("rectangle1 can hold rectangle2 {}", rectangle1.can_hold(&rectangle2));

  // :: 符号
  // -1、用于调用关联函数
  // -2、模块创建的命名空间
  let square = Rectangle::square(10);
  println!("square is {:#?}", square);

  println!()
;}