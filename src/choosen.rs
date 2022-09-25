use std::env;
use std::collections::BinaryHeap;
use std::{io, io::prelude::*};
use rand::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("Need at least 1 argument")
    }

    let n = args[1].parse::<usize>().unwrap();
    // std::println!("{:?}", args);
    let mut rng = rand::thread_rng();
    let mut heap = BinaryHeap::new();
    let fd = io::stdin();
    let mut lines = 0;
    let mut errs = 0;
    for line in fd.lock().lines() {
        lines += 1;
        match line {
            Ok(uline) => {
                // let uline = line.unwrap();
                let r: u64 = rng.gen();
                // println!("{}", uline);
                heap.push((r,uline));
                while heap.len() > n {
                    heap.pop();
                }
            },
            Err(e) => {
                errs += 1;
                eprintln!("{}", e);
            }
        }
    }
    eprintln!("Lines:{} Errors:{}", lines, errs);
    while heap.len() > n {
        heap.pop();
    }
    while heap.len() > 0 {
        // println!("{}", heap.pop().unwrap()[0]);
        println!("{}", heap.pop().unwrap().1);
    }
}
