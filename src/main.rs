mod token;
use std::fs;

fn main() {
    let source_code = fs::read_to_string("sc.txt").expect("Haven't loaded the source code");
    println!("{source_code}");
}
