use std::env::Args;

#[derive(Debug)]
pub enum ParseType {
    Json,
    None,
}
impl Default for ParseType {
    fn default() -> Self {
        ParseType::None
    }
}

impl<S> From<S> for ParseType
where
    S: AsRef<str>,
{
    fn from(s: S) -> Self {
        let s = s.as_ref().trim().to_owned().to_ascii_lowercase();
        match s.as_str() {
            "json" => ParseType::Json,
            _ => ParseType::default(),
        }
    }
}

#[derive(Default, Debug)]
pub struct ClArgs {
    pub filename: String,
    pub parse_type: ParseType,
}

impl ClArgs {
    pub fn pars_args(mut args: Args) -> ClArgs {
        let mut cl_args = ClArgs::default();

        args.next();
        loop {
            // for s in args {
            let s = match args.next() {
                Some(v) => v,
                None => break,
            };

            if let Some(_) = s.find("--file").or(s.find("-f")) {
                cl_args.filename = args.next().expect("flag \"--file\" expected a value");
            } else if let Some(_) = s.find("--type").or(s.find("-t")) {
                cl_args.parse_type =
                    ParseType::from(args.next().expect("flag \"--type\" expected a value"));
            } else {
                cl_args.filename = s;
            }
        }
        cl_args
    }
}
