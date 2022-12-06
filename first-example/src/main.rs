use std::fs::File; //file services?
use std::io::{self, BufReader, BufRead};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    return BufReader::new(File::open(filename)?).lines().collect();
}

fn main() {
    let lines = lines_from_file("./data/real-input.txt").expect("Could not load data from file");
    let mut max_data: u32 = 0;
    let mut sec_max: u32 = 0;
    let mut thrd_max: u32 = 0;
    let mut sum: u32 = 0;

    for line in lines {
        if line.is_empty() {
            if max_data < sum { thrd_max = sec_max; sec_max = max_data; max_data = sum; }
            else if sec_max < sum { thrd_max = sec_max; sec_max = sum; }
            else if thrd_max < sum { thrd_max = sum; }
            sum = 0;
            continue;
        }
        match line.parse::<u32>() {
            Ok(n) => { sum += n; }
            Err(e) => { println!("{}", e); }
        }
    }

    //Final check
    if max_data < sum { thrd_max = sec_max; sec_max = max_data; max_data = sum; }
    else if sec_max < sum { thrd_max = sec_max; sec_max = sum; }
    else if thrd_max < sum { thrd_max = sum; }

    println!("{}, ", max_data);
    println!("{}, ", sec_max);
    println!("{}, ", thrd_max);
    println!("{}, ", max_data + sec_max + thrd_max);
}
