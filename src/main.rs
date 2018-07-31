extern crate itertools;

use itertools::free::join;
use itertools::Itertools;
use std::env;

fn main() {
    let arg = env::args().nth(1);
    let iterations = match arg {
        Some(string) => match string.parse::<i32>() {
            Ok(num) => num,
            Err(_) => 5,
        },
        None => 5,
    };
    let mut vec_of_vecs = vec![];
    let mut prev = vec![1];
    for _ in 0..iterations {
        let next = pascal_expansion(&prev);
        vec_of_vecs.push(prev);
        prev = next;
    }
    boring_print(&vec_of_vecs);
    pretty_print(&vec_of_vecs);
}

fn pascal_expansion(vec: &Vec<u32>) -> Vec<u32> {
    let mut prev = 0;
    let mut iter = vec.iter();
    let mut new = vec![];
    while let Some(next) = iter.next() {
        let old_prev = prev;
        prev = *next;
        new.push(next + old_prev);
    };
    new.push(prev);
    new
}

fn boring_print(vec: &Vec<Vec<u32>>) -> () {
    let mut iter = vec.iter();
    while let Some(next) = iter.next() {
        println!("{}", join(next, " "));
    }
}

fn pretty_print(vec: &Vec<Vec<u32>>) -> () {
    let mut iter = vec.iter();
    let last_vec: &Vec<u32> = iter.clone().last().unwrap();
    let item_width = last_vec.iter().max().unwrap().to_string().len();
    let spacer = format!("{:^1$}", " ", item_width);
    let total_spaced = last_vec.iter().map(|i| i.to_string()).intersperse(spacer.clone()).collect::<Vec<String>>();
    let num_items = total_spaced.len();
    while let Some(next) = iter.next() {
        let spaced = next.iter().map(|i| format!("{:^1$}", i.to_string(), item_width)).intersperse(spacer.clone()).collect::<Vec<String>>();
        let current_num = spaced.len();
        let diff = num_items - current_num;
        let padding = join(vec![spacer.clone(); diff / 2].iter(), "");
        let spaced = join(spaced, "");
        println!("{}{}{}", padding, spaced, padding);
    }
}
