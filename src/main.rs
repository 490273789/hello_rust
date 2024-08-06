mod control_flow;
mod data_type;
mod enumeration;
mod function;
mod ownership;
mod struct_part;
mod vector_part;
mod string_part;
mod hash_map_part;
mod error_part;
mod trait_part;

// trait

const MAX_NUMBER: u32 = 32;

fn main() {
    // rust默认会导入prelude这个库，不在这个库内的就需要自己手动导入了
    // println! 注意这个!是调用宏(macro), 调用函数不需要加感叹号
    // &:str & 引用指针
    println!("Hello, world!");
    // 变量
    // 强制类型转换
    let num = 3.1;
    let _num1 = num as i32; // 未使用的变量可以加_,消除命令行的报错
                            // shallowing variables 影子变量
    let num = 4.1; // 可以使用相同的变量名覆盖上一个变量
    println!("num is: {num}");

    // 常量
    // 1、不能使用mut 关键字
    // 2、常量可以在任何作用域内进行声明
    // 3、常量只可以绑定常量表达式，无法绑定到函数的调用结果或只能在运行时才能计算出的值。
    // 在运行期间，常量在其声明的作用域内一直有效
    // 常量使用全大写字母，每个单词间使用下划线链接
    println!("常量 MAX_NUMBER: {MAX_NUMBER}");

    println!("----- 标量类型 - Scalar Types -----");
    data_type::integer_mod();
    data_type::floating_point_mod();
    data_type::boolean_mod();
    data_type::characters_mod();
    println!("----- 复合类型 - Compound Types -----");
    data_type::array_mod();
    data_type::tuple_mod();
    // 函数 - function
    function::function_mod();
    // 流程控制
    control_flow::control_flow_mod();
    // 所有权
    ownership::ownership_mod();
    // 结构体
    struct_part::struct_mod();
    struct_part::struct_method_mod();
    enumeration::enum_base_mod();
    enumeration::match_base_mod();

    // vector
    vector_part::vector_base_mod();
    vector_part::vector_loop_mod();

    // 字符串
    string_part::string_base_mod();

    // hash map
    hash_map_part::hash_map_mod();

    // 错误处理
    error_part::error_handle_mod();
    error_part::unwrap_mod();
    error_part::error_operation_mod();

    // trait
    trait_part::trait_base_mod();
}
