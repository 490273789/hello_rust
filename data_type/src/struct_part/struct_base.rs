struct User {
  name: String,
  age: u8,
  active: bool
}

#[derive(Debug)]
struct Rectangle {
 width: u32,
 height: u32
}
pub fn struct_mod() {
  println!("------ struct ------");
  let user = User {
    name: String::from("wsn"),
    age: 18,
    active: true
  };

  println!("name is {}, age is {}, active is {}", user.name, user.age, user.active);

  let rectangle = Rectangle{
    width: 10,
    height: 5
  };
  let area = rectangle_area(&rectangle);

  println!("Rectangle area is {}", area);

  println!("{:?}", rectangle);
  println!("{:#?}", rectangle);

  println!();
}

fn rectangle_area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}
