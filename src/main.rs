use std::collections::HashMap;

fn collections_vector() {
    // 使用 push 更新需加上 mut 修饰
    // let mut v: Vec<u32> = Vec::new();
    // v.push(29);
    // v.push(71);

    // 使用宏 vec! 初始化集合，若后续 push 仍要声明 mut
    // let v: Vec<u32> = vec![29, 99, 11, 24, 71];
    // 有初始值，rust 会自动判断类型，所以不需要声明存储类型
    let v = vec![29, 99, 11, 24, 71];

    println!("{:?}", v);

    // 两种方式读取向量元素

    // 1. 使用索引操作符[]
    // 这里的&是取引用的意思，意味着two变量现在包含了向量v中第二个元素（索引是从0开始的）的引用
    let two: &i32 = &v[1];
    println!("two element is {}", two);

    // 2. 使用get方法
    // 与索引操作符不同，get方法返回一个Option枚举，这意味着结果要么是Some(&i32)（如果索引有效），要么是None（如果索引无效或越界）。Option枚举是Rust标准库中的一个类型，它用于编码可选值的概念：一个值可能存在（Some），也可能不存在（None）。
    // 使用get方法的好处是它不会导致程序因索引越界导致panic（运行时错误），如果尝试的索引无效，它会安全地返回None。这允许你在不确定索引是否合法时，更安全地处理这种情况。
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("third element is {third}"),
        None => println!("There is no third element."),
    }

    // 循环迭代
    for i in &v {
        println!("{i}");
    }

    // 循环迭代中对元素进行更改
    {
        let mut v = vec![29, 99, 11, 24, 71];
        for i in &mut v {
            *i += 50;
        }
        println!("Changed {:?}", v);
    }
}

fn collections_strings() {
    // 创建一个空的字符串
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    // 更新字符串

    let mut s = String::from("hello");

    // 使用 push_str 方法将字符串切片附加到 String
    s.push_str(" world");
    assert_eq!("hello world", s);

    // push 方法增加单个字符
    s.push('!');
    assert_eq!("hello world!", s);

    let a: String = String::from("hello");
    let b: String = String::from("world");
    let c = a + &b;
    assert_eq!("helloworld", c);

    let s1 = String::from("hello");
    let s2 = String::from("world");
    let d: String = format!("{s1}{s2}");
    assert_eq!("helloworld", d);

    for i in d.chars() {
        println!("{i}");
    }
}

fn collection_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 20);
    println!("{:?}", scores);

    let key = String::from("yellow");
    // get 方法返回一个 Option<&V>;如果哈希映射中该键没有值， get 将返回 None
    // 调用 copied 来处理 Option 来获取 Option<i32> 而不是 Option<&i32>
    // 使用 unwrap_or 来获取如果 scores 没有该密钥的条目，则将 score 设置为零
    let score = scores.get(&key).copied().unwrap_or(0);
    println!("{}", score);
}

fn main() {
    // collections_vector();
    // collections_strings();
    collection_hashmap();
}
