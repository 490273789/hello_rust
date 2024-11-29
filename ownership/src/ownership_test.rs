use std::rc::Rc;

pub fn ownership_test() {
    println!("----- ownership test ------");
    let hello = return_a_string1();
    println!("The value of hello1 is {hello}");
    let hello = return_a_string2();
    println!("The value of hello2 is {hello}");
    let hello = return_a_string3();
    println!("The value of hello3 is {hello}");

    let mut name = (String::from("Ethan"), String::from("Wang"));
    let first = &name.0;
    // let first = get_first(&name); // 这种写法不可以，rust只关注函数的签名不关注内容
    name.1.push_str(",cool");
    println!("{first} {} ", name.1)
}

// 直接返回引用会报错（造成空指针），直接将所有权返回
// fn return_a_string() -> &String {
//     let s = String::from("Hello");
//     &s
// }

// 解决方案1
fn return_a_string1() -> String {
    String::from("Hello1")
}

// 解决方案2
fn return_a_string2() -> &'static str {
    "Hello2"
}

// 解决方案3
// Rc - 里面会记录指针的数量，当指针数量为0的时候就回释放数据
fn return_a_string3() -> Rc<String> {
    let s = Rc::new(String::from("Hello3"));
    Rc::clone(&s)
}

// rust值关注函数的前面，他会默认你引用是&name
// fn get_first(name: &(String, String)) -> &String {
//     &name.0
// }
