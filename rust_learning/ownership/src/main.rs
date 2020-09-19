fn main() {
    // string
    let mut s = String::from("hello");
    s.push_str(", world!");  // 追加字面值
    println!("{}", s);

    // move
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1);  // error
    println!("{}, world!", s2);

    // clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // data in stack: copy
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // 可变引用
    let mut s = String::from("hello");
    change(&mut s);

    //dangling references
    //let reference_to_nothing = dangle();

    // slice
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];
    

}
fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}

fn first_word(s: &String) -> usize { // 不同步
    let bytes = s.as_bytes();  // 转化为字节数组
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
