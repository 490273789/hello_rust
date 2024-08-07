use std::fmt::Display;

pub fn lifecycle_mod() {
    println!("----- lifecycle -----");
    // 每个引用都有自己的生命周期
    // 生命周期：引用保持有效的作用域
    // 大多数情况下生命周期是隐式的，可推断的
    // 当生命周期可能以不同的方式相互关联时：手动标注生命周期

    // 生命周期的标注，不会改变引用的生命周期长度
    // 当指定了泛型生命周期参数，函数可以接收任何带有生命周期的引用
    // 生命周期标注：描述多个生命周期间的关系，但不影响生命周期

    let string1: String = String::from("abcd");
    let string2: &str = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The longest string is {result}");

    // struct 的生命周期
    // 如果结构体中有引用的数据类型，则需要在引用上添加生命周期标注
    println!();
}

// 'a的生命周期是x和y中生命周期较小的一个
// 指定生命周期参数的方式依赖于函数所作的事情

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 从函数返回引用时，返回类型的生命周期参数需要与其中一个参数的生命周期匹配
// 如果返回的引用没有指向任何参数，那么它只能引用函数内创建的值：
// - 这就是悬垂引用：该值在函数结束就走出了作用域

// 返回了函数内部的值，但函数运行完后result的内存已经销毁，造成悬垂引用
// fn longest1<'a>(x: &'a str, y: &'a str) -> &'a str {
//   let result = String::from("test");
//   result.as_str()
// }

// 那么如果返回函数内部的变量呢？
// 不返回变量的引用，直接返回变量返回即可，讲变量的所有权移交出去
fn longest2<'a>(x: &'a str, y: &'a str) -> String {
    let result = String::from("test");
    result
}

// 可以省略生命周期的3个规则，如果编译器通过下面的三条规则后仍然不能推断出所有的生命周期，那么就会报错
// 规则1：每个引用类型的参数都有自己的生命周期
// 规则2：如果只有1个输入生命周期参数，那么该生命周期被赋予所有的输出生命周期参数
// 规则3：如果有多个输入生命周期参数，但其中一个是&self或&mut self(是方法)， 那么self的生命周期会被赋给所有的输出生命周期参数

// 方法定义中的生命周期标注
// impl<'a> Asa<'a> {}

// 静态生命周期
// 'static是一个特殊的生命周期：整个程序的持续时间
// 所有程序的字面值都有'static生命周期

// 生命周期、泛型、trait bound同时使用
fn longest3<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
