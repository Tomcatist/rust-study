use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;


fn main () {
    // 命令行接受输入
    let args: Vec<String> = env::args().collect();

    // 调用构造函数Config的new() 方法 并且处理错误信息（闭包）
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process:: exit(1);
    });

    // 调用run() 并处理错误信息
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };

}


