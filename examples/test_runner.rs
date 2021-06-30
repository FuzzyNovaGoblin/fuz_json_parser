use std::{env, fs, path::Path};

use args::{Args, ArgsError};
use getopts::Occur;

const PROGRAM_NAME: &'static str = "fuz_json_parser test file runner";
const PROGRAM_DESC: &'static str = "choose a test file and run it through the fuz_json_parser";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (file_path, use_pretty, use_debug) = match parse()? {
        ParseResults::Data {
            debug,
            path,
            pretty,
        } => (path, pretty, debug),
        ParseResults::Err => return Err("no valid test file given".into()),
        ParseResults::DontRun => return Ok(()),
    };

    let file_string = fs::read_to_string(file_path)?;
    let parsed_data = match fuz_json_parser::json_parser::parse(file_string) {
        Ok(v) => v,
        Err(e_str) => {
            eprintln!("failed to parse json with message: {}", e_str);
            return Ok(());
        }
    };

    println!("parsing complete\n",);

    if use_debug {
        println!("{:?}\n", parsed_data);
    }

    if use_pretty {
        println!("{}\n", parsed_data);
    }

    Ok(())
}

enum ParseResults {
    Data {
        path: String,
        pretty: bool,
        debug: bool,
    },
    DontRun,
    Err,
}

fn parse() -> Result<ParseResults, ArgsError> {
    let mut args = Args::new(PROGRAM_NAME, PROGRAM_DESC);
    args.flag("h", "help", "print this help menu");
    args.option(
        "f",
        "file",
        "file name or file path of test file",
        "[FILE | PATH]",
        Occur::Optional,
        Some(String::from("")),
    );
    args.flag("P", "no-pretty", "don't use display print output");
    args.flag("D", "no-debug", "don't use debug print output");

    args.parse_from_cli()?;

    if args.value_of("help")? {
        println!("{}", args.full_usage());
        return Ok(ParseResults::DontRun);
    }

    let file_path = get_valid_file_path(args.value_of("file")?);

    match file_path {
        Ok(path) => Ok(ParseResults::Data {
            path,
            debug: !args.value_of("no-debug")?,
            pretty: !args.value_of("no-pretty")?,
        }),
        Err(_) => Ok(ParseResults::Err),
    }
}

fn get_valid_file_path(path_str: String) -> Result<String, ()> {
    if Path::new(&path_str).exists() {
        return Ok(path_str);
    }

    for an_path in match env::current_exe() {
        Ok(v) => v,
        Err(_) => return Err(()),
    }
    .ancestors()
    .skip(1)
    {
        if an_path.ends_with("fuz_json_parser") {
            let mut p_buf = an_path.to_path_buf();
            p_buf.push("test_files");
            p_buf.push(path_str);
            if p_buf.exists() {
                return Ok(String::from(p_buf.to_string_lossy()));
            }
            break;
        }
    }

    Err(())
}
