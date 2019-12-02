use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn set_previous_data(data: &mut [usize]) {
    data[1] = 12;
    data[2] = 2;
}

fn evaluate(pc: usize, data: &mut [usize]) -> bool {
    match data[pc + 0] {
        1 => {
            data[data[pc + 3]] = data[data[pc + 1]] + data[data[pc + 2]];
            false
        }
        2 => {
            data[data[pc + 3]] = data[data[pc + 1]] * data[data[pc + 2]];
            false
        }
        99 => true,
        _ => panic!("unknown opcode"),
    }
}

pub fn main() {
    let input_path = Path::new("input/2.txt");
    let mut input_file = File::open(&input_path).unwrap();
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();
    let mut data = input
        .trim()
        .split(',')
        .map(|text| text.parse::<usize>().expect("cannot parse text"))
        .collect::<Vec<_>>();
    set_previous_data(&mut data[..]);
    let mut pc = 0;

    loop {
        if evaluate(pc, &mut data[..]) {
            break;
        }
        pc += 4;
    }

    let result = data[0];
    println!("result = {}", result);
}
