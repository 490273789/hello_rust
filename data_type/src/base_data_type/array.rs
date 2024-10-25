pub fn array_mod() {
    println!("----- array module -----");
    // 1、固定长度
    // 2、每个元素的类型相同
    // 3、stack上分配的单个块的内存

    // 创建方式 [1,b,c]
    println!("创建方式1: 字面量 - [1,b,c] ");
    let arr1 = [1, 2, 3];
    for ele in arr1 {
        println!("arr1's ele is: {ele}");
    }

    // 创建方式2
    println!("创建方式2: [value; size]");
    let arr2 = [6; 3];
    // 获取索引元素 arr[index]
    for ele in arr2 {
        println!("arr2's ele is: {ele}");
    }

    // 获取长度 arr.len()
    // 数组用处
    // 如果想让数据存放在stack（栈）上而不是heap（堆）上，或者想保证固定的元素数量。

    // 访问数组
    println!("使用索引访问");
    let ele1 = arr1[1];
    println!("arr1's ele1 is: {ele1}");

    println!("使用get方法访问，避免索引越界");
    match arr1.get(1){
        Some(value) => println!("arr1's ele2 is: {value}"),
        _node => println!("value not found")
    }


    println!("----- end -----");
    println!();
}
