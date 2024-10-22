
mod enumeration;
mod error_part;
mod hash_map_part;
mod lifecycle;
mod ownership;
mod trait_part;
mod vector_part;

// trait
// crate
// library crate (1)s
// binary crate (n)
fn main() {
  // rust默认会导入prelude这个库，不在这个库内的就需要自己手动导入了
  // println! 注意这个“!”是调用宏(macro), 调用函数不需要加感叹号
  // &:str - & 引用指针
  println!("Hello, Rust!");

  // 所有权
  // ownership::ownership_mod();
  // ownership::quote_borrow();

  // enumeration::enum_base_mod();
  // enumeration::match_base_mod();

  // vector
  // vector_part::vector_base_mod();
  // vector_part::vector_loop_mod();

  // hash map
  // hash_map_part::hash_map_mod();

  // 错误处理
  // error_part::error_handle_mod();
  // error_part::unwrap_mod();
  // error_part::error_operation_mod();

  // trait
  // trait_part::trait_base_mod();

  // 生命周期
  // lifecycle::lifecycle_mod();
  let a = 2 ^ 7;
}

//
