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
        let mut array: Vec<i32> = Vec::new(); // Moved array initialization outside the inner loop
        for c in i.chars() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap() as i32; // Parse digit correctly
                array.push(digit);
            }
        }

        let len = array.len(); // Calculate length after digits are added to the array
        let digit;
        if len != 0 {
            digit = array[0] * 10 + array[len - 1];
        } else {
            digit = array[0] * 10 + array[0];
        }
        total = total + digit;
    }
    println!("{total}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
