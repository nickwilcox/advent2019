use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn fuel_for_mass(mass: u32) -> u32 {
    if mass < 8 {
        0
    } else {
        let fuel = mass / 3 - 2;
        fuel_for_mass(fuel) + fuel
    }
}

pub fn main() {
    let input_path = Path::new("input/1.txt");
    let mut input_file = File::open(&input_path).unwrap();
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();
    let result: u32 = input
        .lines()
        .map(|line| fuel_for_mass(line.parse::<u32>().unwrap()))
        .sum();
    println!("result = {}", result);
}
