fn main() {
    println!("----- code organize -----");
    // crate是组织代码的基本构建块
    // 1、binary crate：可执行的，需要有main函数
    // 2、library crate：没有main函数，无法执行。定义一些功能，可共享使用。

    // crate root：编译crate的入口点
    // 1、binary crate： src/main.rs
    // 2、library crate：src/lib.rs
    //
    // package: 由1个或多个crates组成
    // 包含cargo.toml文件（描述乳鸽构建这些crates）
    // package规则
    // 1、可有多个 binary crates
    // 2、最多只能有1个library crate
    // 3、但至少的有一个crate
    // 创建binary crate：cargo new project_name
    // 创建library crate：cargo new project_lib --lib
    // Module 模块
    // 使用mod声明： mod garden
    // 1、可有子模块
    // 2、路径 path
    // 3、public vs private
    // 4、引用 use
    println!("----- code prganize end ------");
    println!();
}
