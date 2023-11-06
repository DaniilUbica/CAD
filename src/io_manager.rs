use std::collections::vec_deque;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{BufRead, BufReader};

use mematrica::{CMatrix, Matrix};

pub fn count_parts(file_name: &str) -> usize {
    let file = OpenOptions::new().read(true).open(format!("files/{}.cn", file_name)).expect("Can't open file");
    let lines_amount = BufReader::new(&file).lines().count();
    lines_amount
}

pub fn save_rects(rects: &[(usize, usize)], e: &[usize], k: &[usize], file_name: &str) {
    let mut file = OpenOptions::new().write(true).create(true).open(format!("files/{}.cn", file_name)).expect("Can't open file");

    for i in 0..rects.len() {
        write!(file, "{} {} {} {}\n", rects[i].0, rects[i].1, e[i], k[i]).unwrap();
    }
}

pub fn save_loads(point: &[(i32, i32)], distributed: &[(i32, i32)], file_name: &str) {
    let mut file = OpenOptions::new().write(true).create(true).open(format!("files/{}.ld", file_name)).expect("Can't open file");

    for i in 0..point.len() {
        write!(file, "{} {}\n", point[i].0, point[i].1).unwrap();
    }

    write!(file, "*\n").unwrap();

    for i in 0..distributed.len() {
        write!(file, "{} {}\n", distributed[i].0, distributed[i].1).unwrap();
    }
}

pub fn save_results(deltas: Vec<f64>, forces: Vec<(f64, f64)>, file_name: &str) {
    let mut file = OpenOptions::new().write(true).create(true).open(format!("files/{}.md", file_name)).expect("Can't open file");

    for i in 0..deltas.len() {
        write!(file, "{}\n", deltas[i]).unwrap();
    }

    write!(file, "*\n").unwrap();

    for i in 0..forces.len() {
        write!(file, "{} {}\n", forces[i].0, forces[i].1).unwrap();
    }
}

pub fn read_rects(file_name: &str) -> (Vec<(usize, usize)>, Vec<usize>, Vec<usize>) {
    let mut rects = vec![];
    let mut e = vec![];
    let mut k = vec![];

    let file = OpenOptions::new().read(true).open(format!("files/{}.cn", file_name)).expect("Can't open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut numbers_iter = line.split_whitespace();

        let number1: usize = numbers_iter.next().expect("Missing number").parse().expect("Invalid number");
        let number2: usize = numbers_iter.next().expect("Missing number").parse().expect("Invalid number");
        let value1: usize = numbers_iter.next().expect("Missing number").parse().expect("Invalid number");
        let value2: usize = numbers_iter.next().expect("Missing number").parse().expect("Invalid number");

        rects.push((number1, number2));
        e.push(value1);
        k.push(value2);
    }

    (rects, e, k)
}

pub fn read_loads(file_name: &str) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let mut dist = vec![];
    let mut point = vec![];

    let file = OpenOptions::new().read(true).open(format!("files/{}.ld", file_name)).expect("Can't open file");
    let reader = BufReader::new(file);
    let mut found_asterisk = false;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        if line == "*" {
            found_asterisk = true;
            continue;
        }

        let mut numbers_iter = line.split_whitespace();

        let number1: i32 = numbers_iter.next().expect("Missing number").parse().expect("Invalid number");
        let number2: i32 = numbers_iter.next().expect("Missing number").parse().expect("Invalid number");

        if found_asterisk {
            dist.push((number1, number2));
        } 
        else {
            point.push((number1, number2));
        }
    }

    (point, dist)
}