use core::panic;
use std::env;
use std::fs;
use std::process;

fn main () {
    // 命令行接受输入
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process:: exit(1);
    });
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
    fn new(args: &[String]) -> Result<Config, &'static str> {
        // 入参判断
        if args.len() < 3 {
            //panic!("not enough arguments");
            return Err("not enough arguments");
        }
        // 获取输入参数的第一个和第二个
        let query = args[1].clone();
        let filename = args[2].clone();
        // 返回结构体
        Ok(Config { query, filename })
    }
}

