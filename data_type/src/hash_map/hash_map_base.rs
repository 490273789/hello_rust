use std::collections::HashMap;

pub fn hash_map_use() {
  let mut hash = HashMap::new();
  hash.insert("a", 1);
  println!("The value of hash is {hash:#?}");
}