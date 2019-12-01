use std::io;
use std::io::BufRead;

fn main() {
    let mut sum = 0;

    for line in io::BufReader::new(io::stdin()).lines() {
        let num: i64 = line.unwrap().parse().unwrap();
        sum += (num / 3) - 2;
    }

    println!("{}", sum);
}
