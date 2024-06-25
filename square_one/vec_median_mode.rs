use std::collections::HashMap;
use std::io;

fn main() {
    println!("enter a white-space separated list of integers:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input read failure");

    let mut input: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("couldn't parse input value"))
        .collect();

    if input.len() == 0 {
        println!("no input provided, exiting");
        std::process::exit(0);
    }

    input.sort_by(|a, b| a.cmp(b));
    println!("sorted input = {:?}", &input);

    print!("median = ");
    let middle_index = input.len() / 2;
    match input.len() % 2 {
        1 => println!("{}", input[middle_index]),
        0 => match input.len() == 1 {
            true => println!("{}", input[0]),
            false => println!("{}", (input[middle_index - 1] + input[middle_index]) / 2),
        },
        _ => unreachable!(),
    }
    print!("mode = ");
    let mut frequencies = HashMap::new();
    for number in input {
        let count = frequencies.entry(number).or_insert(0);
        *count += 1;
    }
    let mode = frequencies.iter().max_by_key(|&(_, &value)| value);
    match mode {
        Some((key, _)) => println!("{}", key),
        _ => println!("meh"),
    }
}
