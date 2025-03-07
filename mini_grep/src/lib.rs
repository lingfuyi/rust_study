use clap::{Arg, Command};

use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let fd = fs::read_to_string(config.filepath)?;
    let ret = if config.case_sensitive {
        search(&config.query, &fd)
    } else {
        search_case_insensitive(&config.query, &fd)
    };
    for line in ret {
        println!("{}", line);
    }
    Ok(())
}
pub fn search<'a>(query: &str, fd: &'a str) -> Vec<&'a str> {
    fd.lines().filter(|line| line.contains(query)).collect()
}
pub fn search_case_insensitive<'a>(query: &str, fd: &'a str) -> Vec<&'a str> {
    fd.lines()
        .filter(|line| line.to_lowercase().contains(query.to_lowercase().as_str()))
        .collect()
}
pub struct Config {
    pub query: String,
    pub filepath: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn from_clap_app() -> Result<Config, &'static str> {
        let matches = Command::new("mini_grep")
            .version("1.0")
            .author("<lingfuyi> <<lingfuyi@126.com>>")
            .about("Searches for a string in a file")
            .arg(
                Arg::new("case_sensitive")
                    .short('s')
                    .long("case-sensitive")
                    .help("Case sensitive search"),
            )
            .arg(
                Arg::new("query")
                    .required(true)
                    .index(1)
                    .help("The string to search for"),
            )
            .arg(
                Arg::new("filepath")
                    .required(true)
                    .index(2)
                    .help("The file to search in"),
            )
            .get_matches();
        let query = matches
            .get_one::<String>("query")
            .ok_or("query is required")?;
        let filepath = matches
            .get_one::<String>("filepath")
            .ok_or("file path is required");
        //通过 copied().unwrap_or(true) 将 Option<&bool> 转换为 bool，并设置默认值为 true
        let case_sensitive = matches
            .get_one::<bool>("case_sensitive")
            .copied()
            .unwrap_or(true);

        Ok(Config {
            query: String::from(query),
            filepath: String::from(filepath),
            case_sensitive,
        })
    }
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
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
            vec!["Rust:", "Trust me.",],
            search_case_insensitive(query, contents)
        );
    }
}
