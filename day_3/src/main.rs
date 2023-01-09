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

fn convert_to_prio(item: &char) -> u32 {
    match item {
        'a'..='z' => {
            return (*item as u32) - 96;
        },
        'A'..='Z' => {
            return (*item as u32) - 38;
        },
        _ => { return 0; },
    }
}
fn find_group(bags: &Vec<String>) -> char {
    let bag_a: HashSet<char> = vec_to_set(bags[0].chars().collect());
    let mut inter: Vec<&char> = bag_a.iter().filter(|&x| bags[1].contains(*x) && bags[2].contains(*x)).collect();

    return inter[0].clone();
}

fn main() {
    let filename = "./data/real.txt";
    let mut lines  = match read_lines(filename) {
        Err(why) => panic!("Error trying to read lines on {} : {}", filename, why),
        Ok(lines) => lines.peekable()
    };

    /* Part 1 solution
    let mut matches: Vec<Vec<char>> = Vec::new();
    for line in lines {
        match line {
            Err(why) => panic!("Error on reading the line: {}", why),
            Ok(line) => {
                matches.push(find_matches(line.chars().collect()));
            }
        }
    }
    //println!("{:?}", matches);
    let mut sum: u32 = 0;
    for submatch in matches {
        for item in submatch {
            sum += convert_to_prio(item);
        }
    }*/

    //Part 2 assuming always divisible by 3
    let mut i = 1;
    let mut group_bags: Vec<String> = Vec::new();
    let mut groups: Vec<char> = Vec::new();
    while !lines.peek().is_none() {
        group_bags.push(lines.next().unwrap().unwrap());
        if (i % 3) == 0 {
            groups.push(find_group(&group_bags));
            group_bags.clear();
        }
        i += 1;
    }
    println!("{:?}", groups);

    let mut sum: u32 = 0;
    for group in groups {
        sum += convert_to_prio(&group);
    }
    println!("{}", sum);
}
