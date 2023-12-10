use std::fs;

fn main() {
    let mut input = fs::read_to_string("input2.txt")
        .expect("Should have been able to read the file");

    input = input.replace("Game","").replace(",","").replace(":","");
    let mut lines : Vec<_> = input.split('\n').collect();
    lines.remove(lines.len()-1);
    let mut total = 0;
    for i in 0..lines.len(){
        let line: Vec<_> = lines[i].split(';').collect();
        let mut mb_v = 0;
        let mut mg_v = 0;
        let mut mr_v = 0;
        let mut b_v = 0;
        let mut r_v = 0;
        let mut g_v = 0;
        let mut sum = 0;
        for j in 0..line.len(){
            let words : Vec<_> = line[j].split(' ').collect();
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

                if b_v > mb_v {
                    mb_v = b_v;
                } 

                if r_v > mr_v {
                    mr_v = r_v;
                } 
                
                if g_v > mg_v {
                    mg_v = g_v;
                } 
            }
        }
        sum = sum + (mb_v * mr_v * mg_v);
        total = total + sum;
    }
    println!("{total}");
}
