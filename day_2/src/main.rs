use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

//From rust docs:
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut sum: u32 = 0;
    let filename = "./data/example.txt";
    let lines  = match read_lines(filename) {
        Err(why) => panic!("Error trying to read lines on {} : {}", filename, why),
        Ok(lines) => lines
    };

    for line in lines {
        match line {
            Err(why) => panic!("Error on reading the line: {}", why),
            Ok(line) => {
                //do stuff
                //A/X -> rock = 1
                //B/Y -> paper = 2
                //C/Z -> Scissors = 3
                // 0, 3, 6 = lose, draw, win
                println!("{}", line);
            }
        }
    }
}
