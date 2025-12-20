mod parser;
use parser::vless2json;
use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    vless2json(a);
}
