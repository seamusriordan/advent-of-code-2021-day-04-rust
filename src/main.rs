use std::fs;
use day_04_rust::driver::BingoDriver;

fn main() {
    let input_string = fs::read_to_string("input.txt").unwrap();
    let mut input = input_string.lines();

    let mut driver = BingoDriver::new(&mut input);

    print!("{:?}\n", driver.process())
}
