fn main() {
    println!("Hello, Algorithm!");
    let for_num = for_loop(5);
    println!("The value of for_num is {for_num}.");

    let while_num = while_loop(5);
    println!("The value of while_num is {while_num}");

    let recur_num = recur(5);
    println!("The value of recur_num is {recur_num}");

    let tail_recur_num = tail_recur(5, 0);
    println!("The value of tail_recur_num is {tail_recur_num}");

    let fib_num = fib(5);
    println!("The value of fib_num is {fib_num}");

    let for_loop_recur_num = for_loop_recur(5);
    println!("The value of for_loop_recur_num is {for_loop_recur_num}");

}

fn for_loop (n: i32) -> i32 {
    let mut res = 0;
    for i in 1..=n {
        res += i
    }
    res
}

fn while_loop (n: i32) -> i32 {
    let mut res = 0;
    let mut i = 1;

    while i <= n {
        res += i;
        i += 1;
    }
    res
}

fn recur (n: i32) -> i32 {

    // 递归的终止条件，开始“归”
    if n == 1 {
        return 1;
    }

    // 调用自身， 开始 “递”
    let res = recur(n - 1);

    // 返回结果
    n + res
}

fn tail_recur(n: i32, res:i32) -> i32 {
    if n == 0 {
        return res;
    }
    tail_recur(n - 1, res + n)
}

fn fib(n: i32) -> i32 {
    if n == 1 || n == 2 {
        return n - 1;
    }

    let res = fib(n - 1) + fib(n - 2);

    res
}

fn for_loop_recur(n: i32) -> i32 {
    // 模拟调用栈
    let mut stack = Vec::new();

    let mut res = 0;

    // “递”的过程
    for i in (1..=n).rev() {
        stack.push(i);
    }

    // “归”的过程
    while !stack.is_empty() {
        res += stack.pop().unwrap();
    }

    res
}