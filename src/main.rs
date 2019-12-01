use std::io;
use std::io::BufRead;

fn main() {
    let mut sum = 0;

    for line in io::BufReader::new(io::stdin()).lines() {
        let mut num: i64 = line.unwrap().parse().unwrap();
        num = calc(num);

        while num >= 0 {
            sum += num;
            num = calc(num);
        }
    }

    println!("{}", sum);
}

fn calc(num: i64) -> i64 {
    (num / 3) - 2
}
