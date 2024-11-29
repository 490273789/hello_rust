mod ownership;
mod ownership_test;
mod quote_borrow;

fn main() {
    println!("Hello, world!");
    ownership::ownership_mod();
    quote_borrow::quote_borrow();
    ownership_test::ownership_test();
}
