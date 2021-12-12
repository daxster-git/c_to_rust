#![allow(dead_code)]
#![allow(unused_variables)]

use text_io::read;
const PI: f32 = 3.142;

pub fn main() {
    question_14();
    question_16();
}

fn question_14() {
    let radius: f32 = 5.0;
    let circumference: f32 = PI * (radius * radius);
    print!("Circumference = {}", circumference);
}

fn question_16() {
    println!("\nEnter your age: ");
    let age: i32 = read!();

    if age >= 18 {
        print!("You are eligible to vote, and are also able to obtain a learner's permit for driving.");
    } else if age <= 18 && age >= 16 {
        print!("You are ineligible to vote, but are able to obtain a learner's permit for driving.");
    } else {
        print!("You are ineligible to vote, and aren't able to obtain a learner's permit for driving.")
    }
}