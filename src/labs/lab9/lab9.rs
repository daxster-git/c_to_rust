#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_assignments)]

use text_io::read;

pub fn main() {
    // question_2();
    // question_3();
    // question_4a();
    question_4b();
}

fn question_2() {
    let mut numbers: [i32; 4] = [0; 4];
    let mut element: i8 = 0;
    println!("Enter number #1: ");
    numbers[0] = read!();
    println!("Enter number #2: ");
    numbers[1] = read!();
    println!("Enter number #3: ");
    numbers[2] = read!();
    println!("Enter number #4: ");
    numbers[3] = read!();

    println!("Select element (1-4) to display: ");
    element = read!();

    match element {
        1 => print!("The element in the array contains {}", numbers[0]),
        2 => print!("The element in the array contains {}", numbers[1]),
        3 => print!("The element in the array contains {}", numbers[2]),
        4 => print!("The element in the array contains {}", numbers[3]),
        _ => print!("Invalid element selection!"),
    }
}

fn question_3() {
    let mut numbers: [i32; 4] = [0; 4];
    let mut element: i8 = 0;
    println!("Enter number #1: ");
    numbers[0] = read!();
    println!("Enter number #2: ");
    numbers[1] = read!();
    println!("Enter number #3: ");
    numbers[2] = read!();
    println!("Enter number #4: ");
    numbers[3] = read!();

    println!("Select element (1-4) to display: ");
    element = read!();

    disp(element, numbers);

    fn disp(element: i8, numbers: [i32; 4]) {    
        match element {
        1 => print!("The element in the array contains {}", numbers[0]),
        2 => print!("The element in the array contains {}", numbers[1]),
        3 => print!("The element in the array contains {}", numbers[2]),
        4 => print!("The element in the array contains {}", numbers[3]),
        _ => print!("Invalid element selection!"),
        }
    }
}

fn question_4a() {
    let mut numbers: [i32; 4] = [0; 4];
    println!("Enter number #1: ");
    numbers[0] = read!();
    println!("Enter number #2: ");
    numbers[1] = read!();
    println!("Enter number #3: ");
    numbers[2] = read!();
    println!("Enter number #4: ");
    numbers[3] = read!();
    
    let mut min: i32 = numbers[0];
    if min >= numbers[1] {
        min = numbers[1];
    }
    if min >= numbers[2] {
        min = numbers[2];
    }
    if min >= numbers[3] {
        min = numbers[3];
    }
    print!("The smallest number in the array is {}", min);
}

fn question_4b() {
    let mut numbers: [i32; 4] = [0; 4];
    println!("Enter number #1: ");
    numbers[0] = read!();
    println!("Enter number #2: ");
    numbers[1] = read!();
    println!("Enter number #3: ");
    numbers[2] = read!();
    println!("Enter number #4: ");
    numbers[3] = read!();
    smallest_number(numbers);
    
    fn smallest_number(numbers: [i32; 4]) {
        let mut min: i32 = numbers[0];
        if min >= numbers[1] {
            min = numbers[1];
        }
        if min >= numbers[2] {
            min = numbers[2];
        }
        if min >= numbers[3] {
            min = numbers[3];
        }
        print!("The smallest number in the array is {}", min);
    }
}