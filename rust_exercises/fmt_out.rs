fn main() {
    // 直接替代
    println!("{} days", 31);
    // 位置参数
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // 命名参数
    println!("{subject} {verb} {object}", object="the dog", subject="the fox", verb="jumps over");
  
    // 特殊格式
    println!("{} and {:b} binary", 1, 2);

    // 格式化参数：
    // 指定宽度
    println!("{:6}", 1);
    println!("{:1$}", 1, 6);

    // 指定宽度填充
    println!("{number:>width$}", number=1, width=6);
    // 指定宽度（对齐，加0）
    println!("{number:>0width$}", number=1, width=6);
    // 指定小数位数
    println!("Pi is roughly number")

    // {:?}，供调试，fmt::Debug
    // {},fmt::Display

}