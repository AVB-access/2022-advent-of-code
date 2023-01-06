use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::iter::FromIterator;

//From rust docs:
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

//utility func from https://stackoverflow.com/questions/39803237/build-hashset-from-a-vector-in-rust
fn vec_to_set(vec: Vec<char>) -> HashSet<char> {
    HashSet::from_iter(vec)
}

//named left as it will become the left side after split_off
fn find_matches(mut left: Vec<char>) -> Vec<char> {
    let right = left.split_off(left.len()/2);
    let left_set: HashSet<char> = vec_to_set(left);
    let right_set: HashSet<char> = vec_to_set(right);
    let mut matches = Vec::new();

    for item in left_set.iter() {
        if right_set.contains(&item) {
            matches.push(item.clone());
        }
    }

    return matches;
}

fn main() {
    let filename = "./data/example.txt";
    let lines  = match read_lines(filename) {
        Err(why) => panic!("Error trying to read lines on {} : {}", filename, why),
        Ok(lines) => lines
    };

    for line in lines {
        match line {
            Err(why) => panic!("Error on reading the line: {}", why),
            Ok(line) => {
                let matches = find_matches(line.chars().collect());
                println!("{:?}", matches);
            }
        }
    }
}
