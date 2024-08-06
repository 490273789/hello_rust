pub fn function_mod() {
    println!("----- 函数 function -----");
    // 函数的参数
    // parameters 形参 定义函数的参数
    // arguments 实参 调用函数实际传入的参数
    // rust是一个机遇表达式的语言， 表达式会计算产生一个值
    // 表达式可以是语句的一部分 let a = 10， 10 就是一个表达式
    // 语句是执行一些动作的指令
    // 函数的定义也是语句
    // 语句不返回值，所以不可以使用let将一个语句赋给一个变量（和javascript不同）
    function_params(10, 20); // 实参
    five();
    plus_five(10);

    println!("----- end -----");
    println!();
}
/*
 parameters
 x 形参，
 y 形参
 函数的定义是一个语句
 函数的调用是一个表达式
*/
fn function_params(x: u32, y: u32) {
    println!("x, y: {} {}", x, y);
    let y = {
        let a = 1;
        a + 2
    };
    println!("y is {}", y);
}

// 函数的返回值
fn five() -> i32 {
    // 函数的返回值：在-> 符号后面声明函数的类型
    // 在rust里面，返回值就是函数体里面最后一个表达式
    // 若是想提前返回，需使用return关键字，并指定一个值
    // 大多数函数都是默认使用最后一个表达式作为返回值
    5
}
// 默认返回的空tuple
fn plus_five(x: u32) -> u32 {
    x + 5
}
