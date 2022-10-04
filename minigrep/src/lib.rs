use std::error::Error;
use std::fs;

// 业务逻辑
pub fn run(config: Config) -> Result<(),Box<dyn Error>> {
    println!("Query argument is {}", config.query);
    // 读取配置文件内容
    let contents = fs::read_to_string(config.filename)?;
    println!("With text \n{}", contents);
    Ok(())
}

// 构造函数
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // 转换配置函数
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
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
