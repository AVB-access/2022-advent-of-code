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
    let filename = "./data/example.txt";
    let lines  = match read_lines(filename) {
        Err(why) => panic!("Error trying to read lines on {} : {}", filename, why),
        Ok(lines) => lines.peekable()
    };

    //Goal: find subsets of given ranges
    //let mut sum: u32 = 0;
    for line in lines {
        match line {
            Err(why) => panic!("Error on reading the line: {}", why),
            Ok(line) => {
                ranges_from_line(&line);
            }
        };
        //let range a be the left range and b be the right
        //if start of a < start of b and end of b < end of a
            //sum += 1
        //or vise versa
    }

}

/*
* Given a line from the file return the start and end points
* of the two given ranges
*/
fn ranges_from_line(line: &String) -> [u32; 4] {
    let mut num_ranges: [u32;4] = [0,0,0,0];

    let ranges= line.split(&['-', ','][..]);

    for (i, x) in ranges.enumerate(){
        num_ranges[i] = x.parse::<u32>().unwrap();
    }
    
    return num_ranges;
}
