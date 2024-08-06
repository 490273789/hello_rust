use std::collections::HashMap;
pub fn hash_map_mod() {
    println!("------ hashMap -----");
    // 键值对的形式存储数据，一个键对应一个值
    // Hash函数：决定如何在内存中存放k和v
    // hashMap存在heap上
    // 同构，所有的key必须是同一个类型
    // 所有的value必须是同一个类型
    // 创建hashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("black"), 20);

    // 另一种创建HashMap的方式：collect
    // 在元素类型为Tuple的Vector上使用collect方法，可以创建一个HashMap
    // - 要求Tuple有两个值：i个作为k，一个作为v
    // collect 方法可以把数据整合成很多种集合类型，包括HashMap，返回的值需要显式指明类型
    let teams = vec![String::from("blue"), String::from("black")];
    let team_scores = vec![10, 50];

    let hash: HashMap<_, _> = teams.iter().zip(team_scores.iter()).collect();
    println!("hash is {:?}", hash);

    // 对于实现了Copy trait的类型，值会被复制到HashMap中
    // 对于拥有所有权的值，值会被移动，所有权会转移给HashMap
    // 如果将值的引用插入到HashMap，值本身不会移动
    // 在hashMap有效的期间，被引用的值必须保持有效

    // 访问用get方法
    let name = String::from("black");
    let score = hash.get(&name);

    match score {
        Some(s) => println!("score is {}", s),
        None => println!("team not exist"),
    }

    // 遍历
    for (k, v) in &hash {
        println!("key: {}, value: {}", k, v);
    }

    // 更新数据
    // - key已存在
    // 替换现有的value
    scores.insert(String::from("black"), 30);
    // 保留现有的值， 忽略新的value
    scores.entry(String::from("blue")).or_insert(100);
    // 保留现有的值和新的值
    // - or_insert
    // 如果k存在，返回对应 value的一个可变引用
    // 如果k不存在，将方法参数作为k的新值插进去，返回到这个值的可变引用

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // 返回值的可变引用
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);

    // 默认情况下，HashMap使用加密功能强大的Hash函数，可以抵抗拒绝服务器的DoS攻击
    // - 不是可用的最快的Hash算法
    // 但具有更好的安全性
    // 可以执定不同的hasher来切换到另一个函数
    // hasher是实现BuildHasher trait的类型

    println!();
}
