use std::{fs::File, io::ErrorKind};

pub fn error_handle_mod() {
    println!("----- error handle ------");
    //  错误分类
    // 1、可恢复错误：例如文件未找到，可再次尝试
    // -- Result<T,E>
    // 2、不可恢复错误：bug，例如访问索引超出范围
    // -- panic! 宏
    // -- panic! 执行后
    // 1 会打印一个错误信息
    // 2 会展开（unwind）和清理调用栈
    // 3  退出程序
    // 默认情况下：展开调用栈
    // -- 沿着调用栈往回走
    // -- 清理每个遇到函数中的数据
    // 或者立即清理调用栈
    // -- 不进行清理，立即终止程序
    // -- 内存需要os清理
    // 想让二进制文件更小
    // 可以将展开设置为终止
    // 在Cargo.toml 文件中设置profile部分，将panic = "abort"

    // panic!("crash"); // 执行这段代码程序将会崩溃
    // 设置环境变量 RUST_BACKTRACE=1 可以在控制台看到详细的回朔信息

    // 可恢复错误
    // enum Result<T, E> {
    //   OK(T),
    //   Err(E)
    // }
    // let file = File::open("hello.txt");

    // let file = match file {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //       ErrorKind::NotFound => match File::create("hello.txt") {
    //         Ok(fc) => fc,
    //         Err(e) =>  panic!("Error create the file {:#?}", e)
    //       },
    //       _other_error =>  panic!("Error open the file {:#?}", _other_error)
    //     }
    // };

    let file = File::open("hello1.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello1.txt").unwrap_or_else(|e| panic!("Error create the file {:#?}", e))
        } else {
            panic!("Error open the file {:#?}", error);
        }
    });
    println!("{:?}", file);

    // panic!使用场景
    // 在定义一个函数可能失败的时候，优先考虑返回Result
    // 否则就panic
    // - 演示某些概念 unwrap
    // - 原型代码：unwrap expect
    // - 测试：unwrap expect
    println!()
}
