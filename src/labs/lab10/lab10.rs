#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]

use text_io::read;

pub fn main() {
    // question_2a();
    // question_2b();
    // question_3();
    question_4();
}

fn question_2a() {
    let mut name = String::new();
    println!("Enter your name: ");
    std::io::stdin().read_line(&mut name);
    println!("Select element: ");
    let element: usize = read!();

    println!("This element in the string contains the letter {:?}", name.chars().nth(element-1).unwrap());
}

fn question_2b() {
    let mut name = String::new();
    println!("Enter your name: ");
    std::io::stdin().read_line(&mut name);
    println!("Number of characters in your name is {}", name.len()-2); // len - 2 because it's counting the \n on both sides of the string
    println!("Select element: ");
    let element: usize = read!();
    
    println!("This element in the string contains the letter {:?}", name.chars().nth(element-1).unwrap());
}

fn question_3() {
    let mut name = String::new();
    println!("Enter your name: ");
    std::io::stdin().read_line(&mut name);
    println!("Number of characters in your name is {}", name.len()-2); // len - 2 because it's counting the \n on both sides of the string
    print!("Name reversed: {}", reverse(&name));
 
    fn reverse(input: &str) -> String {
        let mut string = String::new();
        for c in input.chars().rev() {
            string.push(c)
        }
        string
    }
}

fn question_4() {
    let months: [i8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    for n in 0..12 {
        println!("Month {} has {} days", (n+1), months[n]);
    }
}