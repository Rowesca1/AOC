use std::fs;

fn main() {
    println!("Day 2: Dive! Part Two");

    let mut a : u64 = 0;   
    let mut h : u64 = 0;
    let mut d : u64 = 0;
    let content = fs::read_to_string("../2a/input.txt").expect("Failed to read input.txt");
    
    let lines = content.lines();
    for line in lines {
        let args : Vec<&str> = line.split(" ").collect();
        match args[0] {
            "up"    => {
                let steps : u64 = args[1].parse().unwrap();
                a = a - steps;
            },
            "down"  => {
                let steps : u64 = args[1].parse().unwrap();
                a = a + steps;
            },
            "forward" => {
                let steps : u64 = args[1].parse().unwrap();
                h = h + steps;
                d = d + (steps * a);
            },
            &_ => {
                println!("Invalid args found in input");
            }
        }
    }
    println!("h = {}, d = {}, h*d = {}", h, d, h*d);
}
