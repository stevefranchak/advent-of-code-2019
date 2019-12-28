use std::io;
use std::io::prelude::*;

fn convert_to_fuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel <= 0 {
        return 0;
    }
    fuel + convert_to_fuel(fuel)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let fuel_iter = stdin.lock().lines()
        .take_while(|l| l.as_ref().expect("could not read line").len() > 0)
        .map(|l| convert_to_fuel(
            l.unwrap().parse::<i32>().expect("could not parse input line as i32")
        ));
    println!("Sum: {}", fuel_iter.sum::<i32>());
    Ok(())
}
