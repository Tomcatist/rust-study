use std::env;

fn main () {
    // 命令行接受输入
    let args: Vec<String> = env::args().collect();
    // 获取输入参数的第一个和第二个
    let query = &args[1];
    let filename = &args[2];

    println!("Search for {}", query);
    println!("In file {}", filename);
}