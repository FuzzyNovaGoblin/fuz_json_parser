use std::{env, error::Error, fs};

use crate::runner_mods::cl_args::ClArgs;

mod runner_mods {
    pub mod cl_args;
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = ClArgs::pars_args(env::args());

    if args.filename == "" {
        return Ok(());
    }

    let data = fs::read_to_string(args.filename.clone())
        .expect(format!("failed to read {}", args.filename).as_str());

    let parsed_data = fuz_json_parser::parser::parse(data.clone());

    match parsed_data {
        Ok(data) => println!("finnished parsing\n{}", data),
        Err(e) => println!("failed to finnish parse with error\n```\n{}\n```", e),
    }

    Ok(())
}
