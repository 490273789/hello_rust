mod base_data_type;
mod string_part;

fn main() {
  // "base", "integer", "float", "string", "array", "slice", "struct"
  let types = ["string"];
  if types.contains(&&"base"[..]) {
    // “::” 访问标准库中公开可用的API
    base_data_type::variable();
  }
  if types.contains(&"integer") {
    base_data_type::integer_mod();
    base_data_type::floating_point_mod();
  }
  if types.contains(&"char") {
    base_data_type::characters_mod();
  }
  if types.contains(&"boolean") {
    base_data_type::boolean_mod();
  }
  if types.contains(&"array") {
    base_data_type::array_mod();
    base_data_type::tuple_mod();
  }
  if(types.contains(&"string")) {
    string_part::string_base_mod()
  }


  // // 结构体
  // struct_part::struct_mod();
  // struct_part::struct_method_mod();
  //
  // // 切片
  // slice::slice_mod();
  //
  // // 字符串
  // string_part::string_base_mod();
}
