use::std::env;
use::std::process;

fn main() {
    let args: Vec<String> = env::args().collect(); // 命令行参数读取
    println!("args: {:?}", args); // 打印参数
    if args.len() != 2 {
        println!("ERROR: you need an argument.");
        process::exit(1);  // 退出程序
    }
    // method 1:
    let letters = args[1].bytes(); // 遍历字符串，返回原始字节
    println!("args[1]: {:?}", letters);
    for (i, item) in letters.enumerate() { 
        match item {                          // match模式匹配
            b'a' | b'A' => println!("{}: 'A'", i),
            b'e' | b'E' => println!("{}: 'E'", i),
            b'i' | b'I' => println!("{}: 'I'", i),
            b'o' | b'O' => println!("{}: 'O'", i),
            b'u' | b'U' => println!("{}: 'U'", i),
            b'y' | b'Y' => if i > 2 {
                println!("{}: 'Y'\n", i);
            },
            _ => println!("{}: {} is not a vowel", i, item),
        }
    }
    // method 2:
    let chars = args[1].chars(); // 遍历字符串，返回char类型值
    println!("args[1]: {:?}", chars);
    for (i, item) in chars.enumerate() { 
        match item {                          // match模式匹配
            'a' | 'A' => println!("{}: 'A'", i),
            'e' | 'E' => println!("{}: 'E'", i),
            'i' | 'I' => println!("{}: 'I'", i),
            'o' | 'O' => println!("{}: 'O'", i),
            'u' | 'U' => println!("{}: 'U'", i),
            'y' | 'Y' => if i > 2 {
                println!("{}: 'Y'\n", i);
            },
            _ => println!("{}: {} is not a vowel", i, item),
        }
    }
    println!("args: {:?}", args);  // args所有权还在
}
// String是一个Vec<u8>的封装