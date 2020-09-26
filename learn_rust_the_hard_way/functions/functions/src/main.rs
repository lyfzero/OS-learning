use::std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    print_arguments(&args);
}

fn print_arguments(args: &Vec<String>) {
    for arg in args.iter() {
        print_letters(&arg);
    }

}
fn print_letters(arg: &str) {
    for c in arg.chars() {
        println!("{}", c);
    }
}