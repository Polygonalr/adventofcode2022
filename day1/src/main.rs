use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn p1(line_vec: Vec<String>) -> i32 {
    let mut current = 0;
    let mut highest = 0;
    for l in line_vec {
        match l.as_str() {
            "" => {
                if current > highest {
                    highest = current;
                }
                current = 0;
            }
            &_ => {
                current += l.parse::<i32>().unwrap();
            }
        }
    }
    if current > highest {
        highest = current;
    }
    highest
}

fn p2(line_vec: Vec<String>) -> i32 {
    const QUEUE_SIZE: usize = 3;
    let mut current = 0;
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    for l in line_vec {
        match l.as_str() {
            "" => {
                if heap.len() < QUEUE_SIZE {
                    heap.push(Reverse(current));
                } else if heap.peek().unwrap().0 < current {
                    heap.pop();
                    heap.push(Reverse(current));
                }
                current = 0;
            }
            &_ => {
                current += l.parse::<i32>().unwrap();
            }
        }
    }
    if heap.peek().unwrap().0 < current {
        heap.pop();
        heap.push(Reverse(current));
    }
    let mut sum = 0;
    while let Some(c) = heap.pop() {
        sum += c.0;
    }
    sum
}

fn main() {
    let filepath = "./input.txt";
    // let mut str_buf = "".to_owned();
    let mut line_vec: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines.flatten() {
            // Process each line...
            line_vec.push(line);
        }
    }
    let line_vec2 = line_vec.to_vec();
    println!("Part 1: {}\nPart 2: {}", p1(line_vec), p2(line_vec2));
}

// Reusable function to read files
// From: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
