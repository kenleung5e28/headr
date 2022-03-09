use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, BufRead, BufReader};

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
            Arg::with_name("files")
            .value_name("FILE")
            .help("Input file")
            .multiple(true)
            .default_value("-")
            .min_values(1)
        )
        .arg(
            Arg::with_name("lines")
            .value_name("LINES")
            .short("n")
            .long("lines")
            .help("Number of lines")
            .takes_value(true)
            .default_value("10")
        )
        .arg(
            Arg::with_name("bytes")
            .value_name("BYTES")
            .short("c")
            .long("bytes")
            .help("Number of bytes")
            .takes_value(true)
            .conflicts_with("lines")
        )
        .get_matches();
    let lines = matches.value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|err| format!("illegal line count -- {}", err))?;
    let bytes = matches.value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|err| format!("illegal byte count -- {}", err))?;
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes,
    })
}

pub fn run(config: Config) ->  MyResult<()> {
    let count = config.files.len();
    for (i, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(e) => eprintln!("head: {}: {}", filename, e),
            Ok(mut buf) => {
                if i > 0 {
                    println!();
                }
                if count > 1 {
                    println!("==> {} <==", filename);
                }
                if let Some(n) = config.bytes {
                    let mut read_buffer = vec![0; n];
                    let size_read = buf.read(&mut read_buffer)?;
                    print!("{}", String::from_utf8_lossy(&read_buffer[..size_read]));
                    continue;
                }
                for _ in 0..config.lines {
                    let mut line_read = String::new();
                    if buf.read_line(&mut line_read)? == 0 {
                        break;
                    }
                    print!("{}", line_read);
                }
            }
        }
    }
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

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}