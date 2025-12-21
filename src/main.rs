mod configurator;
use configurator::Configurator;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let cfg = Configurator::from(&a)?;
    println!("{}", cfg.to_json());

    Ok(())
}
