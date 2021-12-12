#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use text_io::read;

pub fn main() {
    // question_3b();
    question_4();
}

fn question_3b() {
    let mut numbers: [f32; 4] = [1.0; 4];
    let mut max: f32 = numbers[0];
    println!("Enter number {}:", 1);
    numbers[0] = read!();
    println!("Enter number {}:", 2);
    numbers[1] = read!();
    println!("Enter number {}:", 3);
    numbers[2] = read!();
    println!("Enter number {}:", 4);
    numbers[3] = read!();
    
    if max <= numbers[1] {
        max = numbers[1];
    }
    if max <= numbers[2] {
        max = numbers[2];
    }
    if max <= numbers[3] {
        max = numbers[3];
    }
    print!("\nThe largest number is {}", max);
    
}

fn question_4() {
    let mut numbers: [f32; 4] = [1.0; 4];
    println!("Enter number {}:", 1);
    numbers[0] = read!();
    println!("Enter number {}:", 2);
    numbers[1] = read!();
    println!("Enter number {}:", 3);
    numbers[2] = read!();
    println!("Enter number {}:", 4);
    numbers[3] = read!();
    largest_number(numbers);
    fn largest_number(numbers:[f32; 4]) {
        let mut max: f32 = numbers[0]; 
        if max <= numbers[1] {
            max = numbers[1];
        }
        if max <= numbers[2] {
            max = numbers[2];
        }
        if max <= numbers[3] {
            max = numbers[3];
        }
        print!("\nThe largest number is {}", max);
    }
}