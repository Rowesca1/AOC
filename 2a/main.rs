use std::fs;

fn main() {
    println!("Day 2 : Dive!");
    
    let mut d = 0;
    let mut h = 0;
    let content = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    
    let lines = content.lines();
    for line in lines {
        let args : Vec<&str> = line.split(" ").collect();
        match args[0] {
            "up"    => {
                let steps : u32 = args[1].parse().unwrap();
                d = d - steps;
            },
            "down"  => {
                let steps : u32 = args[1].parse().unwrap();
                d = d + steps;
            },
            "forward" => {
                let steps : u32 = args[1].parse().unwrap();
                h = h + steps;
            }
            &_ => {
                println!("Invalid args found in input");
            }
        }
    }
    println!("h = {}, d = {}, h*d = {}", h, d, h*d);
}
