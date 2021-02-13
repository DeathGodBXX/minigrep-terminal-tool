use minigrep::Config;
use std::env;
use std::process;

fn main() {
    //1.获取命令行参数
    let args = env::args();

    //2.实例化Config结构体
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments:{}", err);
        process::exit(1);
    });

    //3.从指定文件查找内容的逻辑
    if let Err(e) = minigrep::run(config) {
        eprintln!("Error happened:{}\n", e);
        process::exit(1);
    };
}
