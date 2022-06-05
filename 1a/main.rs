use std::fs;

fn  main () {
    let input =  fs::read_to_string("input.txt").expect("Failed reading input.txt");    
    let mut incr = -1;
    let mut prev = 0;

    for line in input.lines() {
        let depth: u32 = line.parse().unwrap();
        if prev < depth { 
            incr = incr + 1;
        }
        prev = depth;
    }

    println!("Depth increased {} times", incr);
}
