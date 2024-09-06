// Carson Quigley -- CS 575 Algo

use std::env;
use std::fs;
use rand::prelude::*;

fn main() {
    // I'm a type wizard. I cast all day every day.
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
    // range is kept for output later
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

    // good lord this took way too much time to write
    // print_len = corrected range representing num nums in range
    let mut print_len = end_range - start_range;
    if start_range == 0 { print_len = print_len + 1; }

    println!("len: {}\nrange: {} - {}", print_len, start_range, end_range);
    for num in start_range..end_range+1 {
        println!("{}", result[num]);
    }
}

fn partition(a: &mut Vec<i32>, lo: usize, hi: usize) -> usize {
    // saves precious cycles
    if lo == hi { return lo; }

    let mut pivot = rand::thread_rng().gen_range(lo..hi);
    let mut i = lo as isize;
    let mut j = hi as isize;

    // two pointers, move towards pivot
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

        // should only pass when i == pivot and j == pivot
        if i >= j { break; }

        // update pivot position when swapping
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

    return pivot as usize;
}

fn quicksort(mut a: &mut Vec<i32>, lo: usize, hi: usize) -> &mut Vec<i32> {
    // should recurse log N times
    if hi <= lo { return a; }

    let pivot = partition(a, lo, hi);
    a = quicksort(a, lo, pivot);
    a = quicksort(a, pivot+1, hi);
    return a;
}
