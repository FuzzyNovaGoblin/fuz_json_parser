use std::fs;
use fuz_json_parser::json_parser::parse;


fn main() {
    let file_data =
        fs::read_to_string("test_files/tst5").unwrap_or_else(|_| { panic!("{}", "failed to read tst5".to_string()) });
    let data = parse(file_data).expect("failed to parse json");

    println!("{:?}", data);
    println!("\n{:?}\n", data["thang"]);
    println!("\n{:?}\n", data["sub-obj"]);
    println!("\n{:?}\n", data["sub-obj"]["sub-obj"]["val2"]);
    println!("\n{:?}\n", data["sub-obj"]["sub-obj"]["val2"].unwrap_int());
}
