use std::fs;
fn main() {
    /* Since of every three-way measurement compare two of the three
     * numbers are always equal we can do a simple compare between the numbers 
     * which are not. That is the first number of measurement 1 and last numbert 
     * of measurement 2.
     *
     */

    //Use old input file
    let input = fs::read_to_string("../1a/input.txt").expect("Failed reading input.txt");
    let mut lines = input.lines();
    let mut lines2 = input.lines();
    lines2.nth(2);//TODO: hack to fix offset....
    let mut incr = 0;


   loop { 
        let first : u32 = match lines.nth(0) {
            Some(first) => first.parse().unwrap(),
            None => {
                break;
            },
        };

        let last : u32 = match lines2.nth(0) {
            Some(last) => last.parse().unwrap(),
            None =>  {
                break;
            },
        };

        if last > first {
            incr = incr + 1;
        }
    }

    println!("incr = {}", incr);
}
