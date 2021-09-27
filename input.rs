use std::io;
use std::io::prelude::*;

pub fn int() -> u32{
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: u32 = n.trim().parse().expect("invalid input");
    n
}

pub fn matrix(size: u32) -> Vec<Vec<f64>>{
    let mut matrix: Vec<Vec<f64>>= Vec::new();
    let info:Vec<f64> = vec![0., size as f64, size as f64];
    matrix.push(info);

    let stdin = io::stdin();
    let mut i = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let words: Vec<&str> = line.split_whitespace().collect();

        let mut numbers: Vec<f64> = Vec::new();

        for i in  words.iter() {
            let num: f64 = i.parse().unwrap();
            numbers.push(num);
        }
        matrix.push(numbers);
        i += 1;
        if i == size{
            break;
        }
    }

    matrix
}