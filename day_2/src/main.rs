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

//A/X -> rock = 1 //B/Y -> paper = 2
//C/Z -> Scissors = 3 // 0, 3, 6 = lose, draw, win
fn _calc_line_score_p1(hands: Vec<&str>) -> u32 {
    let mut inner_sum: u32 = 0;
    let win = 6;
    let draw = 3;

    match hands[1] {
        "X" => {
            inner_sum += 1;
            match hands[0] {
                "C" => {inner_sum += win;},
                "A" => {inner_sum += draw;},
                _ => {},
            };
        },
        "Y" => {
            inner_sum += 2;
            match hands[0] {
                "A" => {inner_sum += win;},
                "B" => {inner_sum += draw;},
                _ => {},
            }
        },
        "Z" => {
            inner_sum += 3;
            match hands[0] {
                "B" => { inner_sum += win; },
                "C" => { inner_sum += draw; },
                _ => {},
            }
        }
        _ => println!("Unknown value: {}", hands[1]),
    };

    return inner_sum;

}

//X means you need to lose,
//Y means you need to end the round in a draw,
//and Z means you need to win.
fn calc_line_score_p2(hands: Vec<&str>) -> u32 {
    let mut inner_sum: u32 = 0;

    match hands[1] {
        "X" => {
            match hands[0] {
                "A" => {inner_sum += 3;}, // play Scissors
                "B" => {inner_sum += 1;}, //play rock
                "C" => {inner_sum += 2;}, //play paper
                _ => {},
            };
        },
        "Y" => {
            inner_sum += 3;
            match hands[0] {
                "A" => {inner_sum += 1;},
                "B" => {inner_sum += 2;},
                "C" => {inner_sum += 3;}
                _ => {},
            }
        },
        "Z" => {
            inner_sum += 6;
            match hands[0] {
                "A" => { inner_sum += 2; }, //play paper
                "B" => { inner_sum += 3; }, //play Scissors
                "C" => { inner_sum += 1; }, //play rock
                _ => {},
            }
        }
        _ => println!("Unknown value: {}", hands[1]),
    };

    return inner_sum;

}

fn main() {
    let mut total_sum: u32 = 0;
    let filename = "./data/real.txt";
    let lines  = match read_lines(filename) {
        Err(why) => panic!("Error trying to read lines on {} : {}", filename, why),
        Ok(lines) => lines
    };

    for line in lines {
        match line {
            Err(why) => panic!("Error on reading the line: {}", why),
            Ok(line) => {
                //do stuff

                total_sum += calc_line_score_p2(line.split_whitespace().collect());

            }
        }
    }
    println!("{}", total_sum);
}
