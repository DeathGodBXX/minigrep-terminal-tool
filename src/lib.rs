use std::env;
use std::error::Error;
use std::fs;
pub struct Config {
    pub query: String,
    pub filename: String,
    pub is_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Don't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Don't get a filename string"),
        };

        //命令行的优先级高于环境变量,设置环境变量-is_insensitive 是bool类型;默认情况下是大小写敏感的
        let is_sensitive = match args.next() {
            Some(_) => match args.next() {
                Some(t) => t.parse::<bool>().unwrap_or_else(|err| {
                    eprintln!("{},default is sensitive!!!", err);
                    true
                }),
                None => {
                    eprintln!(" you forgot to set -is_sensitive variable, default is sensitive!!!");
                    true
                }
            },
            None => env::var("IS_INSENSITIVE").is_err(),
        };

        Ok(Config {
            query,
            filename,
            is_sensitive,
        })
    }
    // pub fn new(args: &[String]) -> Result<Config, &'static str> {
    //     if args.len() < 3 {
    //         return Err("not enough arguments!!!");
    //     }
    //     let query = args[1].clone();
    //     let filename = args[2].clone();
    //     let is_sensitive;
    //     //命令行的优先级高于环境变量,设置环境变量-is_insensitive 是bool类型;默认情况下是大小写敏感的
    //     if args.len() >= 5 && args[3] == "-is_sensitive" {
    //         is_sensitive = args[4].parse::<bool>().unwrap_or_else(|err| {
    //             eprintln!(" {}, default is sensitive!!!", err);
    //             true
    //         });
    //     } else {
    //         is_sensitive = env::var("IS_INSENSITIVE").is_err();
    //     }

    //     Ok(Config {
    //         query,
    //         filename,
    //         is_sensitive,
    //     })
    // }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.is_sensitive {
        search(&config.query, &contents)
    } else {
        search_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

// pub fn search<'a>(_query: &str, _contents: &'a str) -> Vec<&'a str> {
//     let mut result = Vec::new();
//     for line in _contents.lines() {
//         if line.contains(_query) {
//             result.push(line);
//         }
//     }
//     result
// }

//重构引入更少的变量
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let mut result = Vec::new();
//     let query = query.to_lowercase();
//     for line in contents.lines() {
//         if line.to_lowercase().contains(&query) {
//             result.push(line);
//         }
//     }
//     result
// }

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duck here.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_insensitive(query, contents)
        );
    }
}
