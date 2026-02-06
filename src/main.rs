mod configurator;
use configurator::Configurator;
use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut cfg = Configurator::new();
    cfg.default().set_outbound_from_url(&a);
    cfg.save_to_file();
    println!("{}", serde_json::to_string(&cfg).unwrap());
}
