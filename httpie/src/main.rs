use clap::{AppSettings, Clap};


///
/// 此项目废弃 2022.10.18 22:36
///
#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "yyss")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

// 子命令分别对应不通的Http方法， 目前只支持 get / post
#[derive(Clap, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post)
}

// get子命令

#[derive(Clap, Debug)]
struct Post {
    url: String,
    body: Vec<String>,
}


fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
