// 命名规则： PascalCase
#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    active: bool,
    email: String,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn struct_mod() {
    println!("------ struct ------");
    // struct 自定义类型，其中可包含多种数据类型
    // 如果想要改变结构体中的数据，let 后面要添加mut关键字
    let user = User {
        name: String::from("wsn"),
        age: 18,
        active: true,
        email: String::from("wansn666@you.com"),
    };

    println!(
        "name is {}, age is {}, active is {}, email is {}",
        user.name, user.age, user.active, user.email
    );

    let user2 = build_user(String::from("wansn888@you.com"), String::from("Ethan"));
    println!("The value of user2 is {:#?}", user2);

    // 复用user2中的部分属性
    let user3 = User {
        name: String::from("Wang"),
        ..user2
    };
    println!("The value of user3 is {:#?}", user3);

    let rectangle = Rectangle {
        width: 10,
        height: 5,
    };
    let area = rectangle_area(&rectangle);

    println!("Rectangle area is {}", area);

    println!("{:?}", rectangle);
    println!("{rectangle:#?}");

    let mut rectangle1 = Rectangle {
        width: 20,
        height: 10,
    };

    let x = &mut rectangle1.width;

    // println!("The value of p is {:#?}", &rectangle1); // 有一个可变引用了，不能再有不可变引用
    let h = &rectangle1.height;
    println!("The value of h is {h}");
    *x += 1;

    println!();
}

fn build_user(email: String, name: String) -> User {
    // 变量名简写
    User {
        name,
        age: 19,
        active: false,
        email,
    }
}

fn rectangle_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
