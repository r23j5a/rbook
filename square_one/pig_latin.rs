use std::io;

fn main() {
    println!("hey ig-pay atin-lay!");
    println!("enter some words:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input read failure");

    let mut input: Vec<String> = input
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    if input.len() == 0 {
        println!("no input provided, exiting");
        std::process::exit(0);
    }
    let mut pig_latin = Vec::new();
    for word in &mut input {
        let mut chars = word.chars();
        let mut char = chars.next();
        match char {
            Some(first_char) => {
                if matches!(first_char, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U') {
                    pig_latin.push(format!("{word}-hay"));
                    continue;
                }
            }
            None => unreachable!(),
        }
        let mut index = 0;
        let mut first_consonants_range = 0;
        while char.is_some() {
            let char_value = char.unwrap();
            if matches!(char_value, 
                'b' | 'c' | 'd' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' | 'm' | 'n' | 'p' | 'q' | 'r' | 's' | 't' | 'v' | 'w' | 'x' | 'y' | 'z' |
                'B' | 'C' | 'D' | 'F' | 'G' | 'H' | 'J' | 'K' | 'L' | 'M' | 'N' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'V' | 'W' | 'X' | 'Y' | 'Z') {
                    first_consonants_range += 1;
                }
            if first_consonants_range == index {
                break;
            }    
            char = chars.next();
            index += 1;
        }
        let first_consonants: String = word.drain(..first_consonants_range).collect();
        pig_latin.push(format!("{word}-{first_consonants}ay")); 
    }
    for word in pig_latin {
        print!("{} ", word);
    }
    println!("");
}
