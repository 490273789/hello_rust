# String
## &str 字符串切片
- 字符串切片的结构
```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```
![img.png](./img/img.png)