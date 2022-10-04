use std::env;
use std::fs;

fn main () {
    // 命令行接受输入
    let args: Vec<String> = env::args().collect();
    // 获取输入参数的第一个和第二个
    let query = &args[1];
    let filename = &args[2];

    println!("Search for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    println!("With text \n{}", contents);
}