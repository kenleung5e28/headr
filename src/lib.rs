use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Ken C. Y. Leung <kenleung5e28@gmail.com>")
        .about("Rust head")
        .arg(
            Arg::with_name("file")
            .value_name("FILE")
            .help("Input file")
            .multiple(true)
            .default_value("-")
            .min_values(1)
        )
        .arg(
            Arg::with_name("lines")
            .short("n")
            .long("lines")
            .help("Number of lines")
            .takes_value(true)
            .default_value("10")
            .conflicts_with("bytes")
        )
        .arg(
            Arg::with_name("bytes")
            .short("c")
            .long("bytes")
            .help("Number of bytes")
            .takes_value(true)
        )
        .get_matches();
    Ok(Config {
        files: vec![],
        lines: 0,
        bytes: None,
    })
}

pub fn run(config: Config) ->  MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    // 12 is a valid positive integer
    let res = parse_positive_int("12");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 12);

    // Any non-numeric string is error
    let res = parse_positive_int("E33or");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "E33or".to_string());

    // 0 is error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
