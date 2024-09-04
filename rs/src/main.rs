// Carson Quigley -- CS 575 Algo

use std::env;
use std::fs;
use rand::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 && args.len() != 4{
        println!("arglen: {}", args.len());
        println!("usage: run <filepath> [<start_range> <end_range>]");
        return;
    }

    let fpath = &args[1];
    let mut num_list: Vec<i32> = fs::read_to_string(fpath)
        .expect("what")
        .lines()
        .filter_map(|w| w.parse().ok())
        .collect();
    let len = num_list.len();

    // sanity
    let start_range;
    let end_range;
    if args.len() == 4 {
        start_range = args[2].parse::<usize>().unwrap();
        end_range = args[3].parse::<usize>().unwrap();
    } else {
        start_range = 0;
        end_range = len-1;
    }
    if start_range > end_range || end_range >= len {
        println!("invalid range.");
        println!("usage: run <filepath> [<start_range> <end_range>]");
        return;
    }

    let result = quicksort(&mut num_list, 0, len-1 as usize);

	// good lord this took way too much time
    let mut print_len = end_range - start_range;
    if start_range == 0 { print_len = print_len + 1; }
    println!("len: {}\nrange: {} - {}", print_len, start_range, end_range);
    for num in start_range..end_range+1 {
        println!("{}", result[num]);
    }
}

fn partition(a: &mut Vec<i32>, lo: usize, hi: usize) -> usize {
    if lo == hi { return lo; }
    let mut pivot = rand::thread_rng().gen_range(lo..hi);
    let mut i = lo as isize;
    let mut j = hi as isize;

	// two pointers, I hope this is obvious as-is
    loop {
        while j > pivot as isize {
            if a[j as usize] < a[pivot] {
                break;
            }
            j = j-1;
        }
        while i < pivot as isize{
            if a[i as usize] > a[pivot] {
                break;
            }
            i = i+1;
        }
        if i >= j { break; }
        if i as usize == pivot {
            pivot = j as usize;
        }
        else if j as usize == pivot {
            pivot = i as usize;
        }
        let tmp = a[i as usize];
        a[i as usize] = a[j as usize];
        a[j as usize] = tmp;
    }

    let result = pivot as usize;
    return result;
}

fn quicksort(mut a: &mut Vec<i32>, lo: usize, hi: usize) -> &mut Vec<i32> {
    if hi <= lo { return a; }

    let pivot = partition(a, lo, hi);
    a = quicksort(a, lo, pivot);
    a = quicksort(a, pivot+1, hi);
    return a;
}
