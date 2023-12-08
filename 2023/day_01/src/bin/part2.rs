use std::fs;

fn main(){

    let contents = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let  output = contents.lines().map(process_line).sum::<u32>();

    output.to_string();
    println!("{output}");
}

fn process_line(line: &str) -> u32{
    let mut it = (0..line.len()).filter_map(|index| {
        match &line[index..] {
            line if line.starts_with("one") => Some(1),
            line if line.starts_with("two") => Some(2),
            line if line.starts_with("three") => Some(3),
            line if line.starts_with("four") => Some(4),
            line if line.starts_with("five") => Some(5),
            line if line.starts_with("six") => Some(6),
            line if line.starts_with("seven") => Some(7),
            line if line.starts_with("eight") => Some(8),
            line if line.starts_with("nine") => Some(9),
            line => {
                line.chars().next().unwrap().to_digit(10)
            }
        }
    });
    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => first * 10 + num,
        None => first * 10 + first,
    }

}

