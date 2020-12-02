use std::{fs::File, io::Read};

mod day01;
mod day02;

pub type Error = Box<dyn std::error::Error + 'static>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn read_input(input_file: &str) -> Result<String> {
    let mut f = File::open(format!("inputs/{}", input_file))?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}

fn main() -> Result<()> {
    match std::env::args().nth(1).map(|s| s.parse::<usize>()) {
        Some(Ok(1)) => day01::solve()?,
        Some(Ok(2)) => day02::solve()?,
        Some(Ok(n)) => println!("Day {} is not implemented yet.", n),
        Some(Err(err)) => println!("Invalid day argument: {}.", err),
        None => println!("Day argument must be provided."),
    }

    Ok(())
}
