use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct NumTime {
    number: Vec<u32>,
    index: Vec<u32>,
}
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
        let mut array: Vec<i32> = Vec::new();
        let mut digit = 0;
        let mut ind = 0;
        if i.contains("0") {
            digit = 0;
            array.push(digit);
        }
        if i.contains("1") {
            digit = 1;
            array.push(digit);
        }
        if i.contains("2") {
            digit = 2;
            array.push(digit);
        }

        if i.contains("3") {
            digit = 3;
            array.push(digit);
        }

        if i.contains("4") {
            digit = 4;
            array.push(digit);
        }

        if i.contains("5") {
            digit = 5;
            array.push(digit);
        }

        if i.contains("6") {
            digit = 6;
            array.push(digit);
        }

        if i.contains("7") {
            digit = 7;
            array.push(digit);
        }

        if i.contains("8") {
            digit = 8;
            array.push(digit);
        }

        if i.contains("9") {
            digit = 9;
            array.push(digit);
        }

        if i.contains("zero") {
            digit = 0;
            array.push(digit);
        }
        if i.contains("one") {
            digit = 1;
            array.push(digit);
        }

        if i.contains("two") {
            digit = 2;
            array.push(digit);
        }

        if i.contains("three") {
            digit = 3;
            array.push(digit);
        }

        if i.contains("four") {
            digit = 4;
            array.push(digit);
        }
        if i.contains("five") {
            digit = 5;
            array.push(digit);
        }
        if i.contains("six") {
            digit = 6;
            array.push(digit);
        }
        if i.contains("seven") {
            digit = 7;
            array.push(digit);
        }
        if i.contains("eight") {
            digit = 8;
            array.push(digit);
        }
        if i.contains("nine") {
            digit = 9;
            array.push(digit);
        }
        println!("{i}");
        println!("{array:?}");
        let len = array.len();
        if len != 0 {
            digit = array[0] * 10 + array[len - 1];
        } else {
            digit = array[0] * 10 + array[0];
        }
        total = total + digit;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
