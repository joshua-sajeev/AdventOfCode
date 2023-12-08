use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut lbl: Vec<String> = vec![];

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                lbl.push(ip);
            }
        }
    }

    let mut total = 0;

    for i in lbl {
        let mut first_digit = None;
        let mut last_digit = None;

        for c in i.chars() {
            if c.is_digit(10) {
                // If it's a numeric character, update last_digit
                last_digit = Some(c.to_digit(10).unwrap() as i32);
                if first_digit.is_none() {
                    // If first_digit is None, update it as well
                    first_digit = last_digit;
                }
            } else if c.is_alphabetic() {
                // If it's an alphabetic character, check if it's a spelled-out digit
                match c.to_string().as_str() {
                    "o" => last_digit = Some(1),
                    "t" => last_digit = Some(2),
                    "th" => last_digit = Some(3),
                    "f" => last_digit = Some(4),
                    "fi" => last_digit = Some(5),
                    "s" => last_digit = Some(6),
                    "se" => last_digit = Some(7),
                    "e" => last_digit = Some(8),
                    "n" => last_digit = Some(9),
                    _ => {}
                }
                if first_digit.is_none() {
                    first_digit = last_digit;
                }
            }
            println!("{first_digit:?}");
            println!("{last_digit:?}");
        }

        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            total += first * 10 + last;
        }
    }

    println!("Total: {}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
