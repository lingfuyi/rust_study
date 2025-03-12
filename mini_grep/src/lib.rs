use clap::Parser;
use std::{error::Error, fs};
#[derive(Parser, Debug)]
#[command(author="Mineral", version="v1.0", about="Searches for a string in a file", long_about = Some("这个程序是在一个文件里面搜索字符串,并统计出现次数. \
                     Usage: ./program query filepath [-c] \"case sensitive search\"")
		)]

pub struct Config {
    #[arg(required = true, help = "The string to search for")]
    pub query: String,
    #[arg(required = true, help = "The filepath to search for")]
    pub filepath: String,
    #[arg(short, long, default_value = "false", help = "case sensitive search")]
    pub case_sensitive: bool,
}
impl Config {
    pub fn from_clap_app() -> Result<Config, &'static str> {
        let args = Config::parse();
        Ok(args)
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let fd = fs::read_to_string(config.filepath)?;
    let ret = if config.case_sensitive {
        search(&config.query, &fd)
    } else {
        search_case_insensitive(&config.query, &fd)
    };

    println!("{}出现了{}次", config.query, ret);

    Ok(())
}
pub fn search<'a>(query: &str, fd: &'a str) -> usize {
    fd.lines()
        .map(|line| line.to_lowercase().matches(query).count())
        .sum()
}
pub fn search_case_insensitive<'a>(query: &str, fd: &'a str) -> usize {
    let query = query.to_lowercase();
    fd.lines()
        .map(|line| line.to_lowercase().matches(&query).count())
        .sum()
}
