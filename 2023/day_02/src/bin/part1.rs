use std::fs;

fn main() {
    let mut input = fs::read_to_string("input1.txt")
        .expect("Should have been able to read the file");

    input = input.replace("Game","").replace(",","").replace(":","");
    let mut lines : Vec<_> = input.split('\n').collect();
    lines.remove(lines.len()-1);
    let mut numbers: Vec<u32>  = vec![];
    for i in 0..lines.len(){
        let line: Vec<_> = lines[i].split(';').collect();
        let mut flag = true;
        for j in 0..line.len(){
            let words : Vec<_> = line[j].split(' ').collect();
            let mut b_v = 0;
            let mut g_v = 0;
            let mut r_v = 0;
            for k in 1..words.len(){
                let prev = k-1;
                if words[k].contains("blue"){
                    b_v = words[prev].parse::<i32>().unwrap();
                }

                if words[k].contains("green"){
                    g_v = words[prev].parse::<i32>().unwrap();
                }

                if words[k].contains("red"){
                    r_v = words[prev].parse::<i32>().unwrap();
                }
            }

            if r_v > 12 || g_v > 13 || b_v > 14 {
                flag = false;
                break;
            }
        }
        if flag{
            let num = i+1;
            numbers.push(num.try_into().unwrap());
        }
    }
    let sum: u32 = numbers.iter().sum();
    println!("{sum}");
}
