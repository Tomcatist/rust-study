use std::env;
use std::fs;

fn main () {
    // 命令行接受输入
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    let contents = fs::read_to_string(config.filename)
    .expect("Something went wrong reading the file");

    println!("With text \n{}", contents);
}

// 构造函数
struct Config {
    query: String,
    filename: String,
}

impl Config {
    // 转换配置函数
    fn new(args: &[String]) -> Config {
        // 获取输入参数的第一个和第二个
        let query = args[1].clone();
        let filename = args[2].clone();
        // 返回结构体
        Config { 
            query: query, 
            filename: filename 
        }
    }
}

