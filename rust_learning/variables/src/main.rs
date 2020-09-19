fn main() {
    // 设置可变
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 常量
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    
    // 隐藏
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // 隐藏 vs mut
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The length of spaces is: {}", spaces);
    // let spaces = "   ";
    // spaces = spaces.len(); // error

    // float
    let x = 2.0;
    let y: f32 = 3.0; 

    // calculation
    let sum = 5 + 19;
    let diff = 95.5 - 4.3;
    let prod = 4 * 30;
    let quot = 56.7 / 23.2;
    let remainder = 42%6;

    // bool
    let t = true;
    let f: bool = false; // 显式
    
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    println!("The value of y is: {}", y);
    let a = tup.0; // indice
    let b = tup.1;
    let c = tup.2;
    println!("The value of a, b, c are: {},  {},  {}", a, b, c);

    // array
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    let first = a[0];
    let second = a[1];
    
}
