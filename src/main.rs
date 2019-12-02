use std::io::{stdin, BufRead};
use std::str;

fn main() {
    let mut memory: Vec<usize> = vec![];
    let mut buf: Vec<u8> = vec![];

    let stdin = stdin();
    let mut stdin = stdin.lock();

    loop {
        match stdin.read_until(b',', &mut buf) {
            Ok(0) => break,
            Ok(_) => {
                if buf.last() != Some(&b',') {
                    buf.push(b',');
                }
                memory.push(
                    str::from_utf8(&buf[..buf.len() - 1])
                        .unwrap()
                        .trim()
                        .trim_matches(',')
                        .parse()
                        .unwrap(), // oof forced unwraps
                );
                buf.clear();
            }
            _ => panic!("it broke"),
        }
    }
    // 1202 challenge thing
    memory[1] = 12;
    memory[2] = 2;

    let mut index = 0;
    while index < memory.len() {
        match memory[index] {
            1 => {
                let m1 = memory[index + 1];
                let m2 = memory[index + 2];
                let m3 = memory[index + 3];
                memory[m3] = memory[m1] + memory[m2];
                index += 4;
            }
            2 => {
                let m1 = memory[index + 1];
                let m2 = memory[index + 2];
                let m3 = memory[index + 3];
                memory[m3] = memory[m1] * memory[m2];
                index += 4;
            }
            99 => break,
            _ => unimplemented!(),
        }
    }

    println!("{}", memory[0]);
}
