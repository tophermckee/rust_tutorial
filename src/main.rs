#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn say_hello() {
    println!("What is your name?");
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("Failed to read line");
    println!("Hello {}! {}", name.trim_end(), greeting);
}

fn shadowing_and_stuff() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592653589793;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}

fn data_types() {
    println!("Max u32: {}", std::u32::MAX);
    println!("Max u64: {}", std::u64::MAX);
    println!("Max usize: {}", std::usize::MAX);
    println!("Max u128: {}", std::u128::MAX);
    println!("Max f32: {}", std::f32::MAX);
    println!("Max f64: {}", std::f64::MAX);
    let _unused_variable: &str = "I'm unused AF";   // This variable is unused (underscore)
    let _my_grade: char = 'A';                      // 'A' is a char, not a string (single quotes)
    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.111111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f64: {}", num_2 + 0.111111111111111);
}

fn conditionals() {
    let age: i32 = 80;
    if (age >= 1) && (age <= 18) {
        println!("{} is a pretty important birthday!", age)
    } else if (age == 21) || (age == 50) {
        println!("{} is a pretty important birthday!", age)
    } else if (age >= 65) {
        println!("{} is a pretty important birthday!", age)
    } else {
        println!("{} ain't nothing to call home about, get over it!", age)
    }
}

fn match_conditional() {
    let age: i32 = 25;
    match age {
        1..=18 => println!("Important Birthday!"),
        21 | 50 => println!("Important Birthday!"),
        65..=i32::MAX => println!("Important Birthday!"),
        _ => println!("stupid birthday, dawg")
    };
}

fn cooler_match() {
    let age = 22;
    let voting_age = 21;
    match age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote ❌"),
        Ordering::Greater => println!("Can Vote ✅"),
        Ordering::Equal => println!("You just gained the right to vote 🤏🏼"),
    };
}

fn array_time() {
    // elements in arrays must be of the same type!!!!

    let array_1: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    println!("1st: {}", array_1[0]);
    println!("Length: {}", array_1.len());
}

fn loop_array() {
    let array_1: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx: usize = 0;
    loop {
        if array_1[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if array_1[loop_idx] == 9 {
            break;
        }
        println!("Val: {}", array_1[loop_idx]);
        loop_idx += 1;
    }
}

fn while_loop_example() {
    let my_array: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut idx: usize = 0;

    while idx < my_array.len() {
        println!("Array : {}", my_array[idx]);
        idx += 1;
    }

}

fn for_loop_example() {
    let my_array: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut idx: usize = 0;

    for val in my_array.iter() {
        println!("Val: {}", val);
    }
}

fn main() {
    // println!("Hello, world!");
    // data_types()
    // conditionals()
    // match_conditional()
    // cooler_match()
    // loop_array()
    for_loop_example()
}