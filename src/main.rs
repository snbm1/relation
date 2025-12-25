mod configurator;
use configurator::Configurator;
use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let cfg = Configurator::from(&a).unwrap();
    println!("{}", serde_json::to_string(&cfg).unwrap());
}
