
fn main() {
    p2_2();
    p2_6();
    p2_8();
    p2_10();
    p2_13();
    p2_14();
    p2_20();
    p2_30();
    p2_44();
    p2_45();
    p2_46();
}

fn p2_2() {
    pub fn answer() -> () {
        let a = 40;
        let b = 2;
        assert_eq!(sum(a, b), 42);
    }
    pub fn sum(a: i32, b: i32) -> i32 {
        a + b
    }
    answer();
}
fn p2_6() {
    let a = [1, 2, 3];
    let b = &a; // 引用
    println!("{:p}", b); 
    let mut c = vec![1, 2, 3];
    let d = &mut c;
    d.push(4);
    println!("{:?}", c);
    let e = &42;
    assert_eq!(42, *e);
    println!("{}", e);
}
fn p2_8() { // 函数定义
    println!("{}", fizz_buzz(23));
}
fn fizz_buzz(num: i32) -> String {
    if num % 15 == 0 {
        String::from("fizzbuzz")
    } else if num % 3 == 0 {
        String::from("fizz")
    } else if num % 5 == 0 {
        String::from("buzz")
    } else {
        num.to_string()
    }

}
fn p2_10() { // 函数作为参数
    pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
        op(a, b)
    }
    fn sum(a: i32, b: i32) -> i32 { a + b }
    fn product(a: i32, b: i32) -> i32 { a * b }
    let a = 2;
    let b = 3;
    assert_eq!(math(sum, a, b), 5);
    assert_eq!(math(product, a, b), 6);
}
fn p2_13() { // 闭包
    let out = 42;
    let i = 1;
    let j = 2;
    fn add(i: i32, j: i32) -> i32 { i + j }
    // fn add(i: i32, j: i32) -> i32 { i + j + out }  // error
    let closure_annotated = |i: i32, j: i32| -> i32 { i + j + out };
    let closure_inferred = |i, j| i + j + out;
    assert_eq!(3, add(i, j));
    assert_eq!(45, closure_annotated(i, j));
    assert_eq!(45, closure_inferred(i, j));
}
fn p2_14() { // 闭包作为返回值
    fn two_times_impl() -> impl Fn(i32) -> i32 {
        let i = 2;
        move |j| j * i
    }
    let result = two_times_impl();
    assert_eq!(result(2), 4);
}
fn p2_20() { // match
    let number = 42;
    match number {
        0 => println!("Origin"),
        1..=3 => println!("All"),
        | 5 | 7 | 13 => println!("Bad Luck"),
        n @ 42 => println!("Answer is {}", n),
        _ => println!("Common"),
    }
}
fn p2_30() { // str字符串
    let truth: &'static str = "Rust ";
    let ptr = truth.as_ptr();
    let len = truth.len();
    let s = unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);
        std::str::from_utf8(slice)
    };
    assert_eq!(s, Ok(truth));
}
fn p2_44() { // vec
    let mut v1 = vec![];
    v1.push(1);
    v1.push(2);
    v1.push(3);
    assert_eq!(v1, [1, 2, 3]);
    assert_eq!(v1[1], 2);
    let mut v2 = vec![0; 10];
    v2[0] = 1;
    let mut v3 = Vec::new();
    v3.push(4);
    v3.push(5);
    v3.push(6);
    println!("v1: {:#?}", v1);
    println!("v2: {:#?}", v2);
    println!("v3: {:#?}", v3);
}
use std::collections::VecDeque;
fn p2_45() { // deque
    let mut buf = VecDeque::new();
    buf.push_front(1);
    buf.push_front(2);
    assert_eq!(buf.get(0), Some(&2));
    assert_eq!(buf.get(1), Some(&1));
    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);
    assert_eq!(buf.get(2), Some(&3));
    assert_eq!(buf.get(3), Some(&4));
    assert_eq!(buf.get(4), Some(&5));
}
use std::collections::LinkedList;
fn p2_46() {
    let mut list1 = LinkedList::new();
    list1.push_back('a');
    let mut list2 = LinkedList::new();
    list2.push_back('b');
    list2.push_back('c');
    list1.append(&mut list2);
    println!("{:?}", list1);
    println!("{:?}", list2);
    list1.pop_front();
    println!("{:?}", list1);
    list1.push_front('e');
    println!("{:?}", list1);
    list2.push_front('f');
    println!("{:?}", list2);
}