use std::io;
use std::io::prelude::*;

fn convert_to_fuel(mass: u32) -> u32 {
    mass / 3 - 2
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let fuel_iter = stdin.lock().lines()
        .take_while(|l| l.as_ref().expect("could not read line").len() > 0)
        .map(|l| convert_to_fuel(
            l.unwrap().parse::<u32>().expect("could not parse input line as u32")
        ));
    println!("Sum: {:?}", fuel_iter.sum::<u32>());
    Ok(())
}
