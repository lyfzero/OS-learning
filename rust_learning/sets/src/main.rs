fn main() { 
    sets();
    strings();
    hash_map();
}
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn sets() {
    // 新建
    let mut v: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];

    // 更新
    v.push(5);
    println!("{:?}", v);
    v2.push(4);
    println!("{:?}", v2);

    // 访问元素
    let third: &i32 = &v2[2];  // 返回一个引用
    println!("The third element is {}", third);
    
    match v2.get(2) { // 返回Option<&T>
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 访问一个不存在的元素
    let v = vec![1, 2, 3];
    // let does_not_exist = &v[100]; // panic
    let does_not_exist = v.get(100);
    match does_not_exist {
        Some(does_not_exist) => println!("The element is {}", does_not_exist),
        None => println!("There is no element"),
    }

    // 拥有vector中项的引用的同时向其增加一个元素
    let v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6); // 有不可变引用时禁止更改vector
    // v[1] = 2; // 也禁止更改内容
    println!("The first element is: {}", first);

    // 遍历
    let v = vec![1, 2, 3];
    for i in &v {   // 遍历元素
        println!("{}", i);
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {  // 遍历可变引用
        *i += 50;
    }

    // 用enum创建可存不同值的vector
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.2),
    ];
    
}
fn strings() {
    // 创建
    let _s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);
    let s = "initial contents".to_string();
    println!("{}", s);
    let s = String::from("initial contents");
    println!("{}", s);

    // 更新
    // push_str
    let mut s = String::from("foo");
    s.push_str("bar");
    
    let mut s1 = String::from("foo");
    let mut s2 = String::from("bar");
    s1.push_str(&s2);
    println!("s2 is {}", s2);
    s2.push_str("this");
    println!("s1 is {}, s2 is {}", s1, s2);
    //push
    s1.push('l');
    println!("s1 is {}", s1);
    // + & format!
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // println!("s1 is {}", s1); // error
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);
    println!("s1 is {}", s1); 
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);
    // 读取

    // 索引
    let s1 = String::from("hello");
    let h = &s1[0..3];
    println!("h is {}", h);
    // 遍历
    // chars
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    // bytes
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
fn hash_map() {
    // 创建
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    println!("{:?}", scores);
    // collect
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:#?}", scores);
    // 访问
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    // 覆盖
    scores.insert(String::from("Blue"), 25);
    println!("{:#?}", scores);
    // entry
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(30);
    println!("{:#?}", scores);
    // 更新旧值
    let text = "hello world wonder wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);
}
