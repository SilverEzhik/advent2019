use std::io::{stdin, BufRead};
use std::str;

fn main() {
    let mut memory_orig: Vec<usize> = vec![];
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
                memory_orig.push(
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

    let mut index;
    for x1 in 0..=99 {
        for x2 in 0..=99 {
            let mut memory = memory_orig.clone();
            memory[1] = x1;
            memory[2] = x2;
            index = 0;

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

            if memory[0] == 19690720 {
                println!("{}{}", x1, x2);
                return;
            }
        }
    }
    println!("fail");
}
