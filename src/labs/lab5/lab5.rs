#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use text_io::read;

pub fn main() {
    // question_3b();
    question_5();
}

fn question_3b() {
    let mut sum: i32 = 0;
    let mut count: i32 = 1;
    // sum should be 2500
    for count in (1..100).step_by(2) {
        sum = sum + count;
    }
    print!("{}", sum);
}

fn question_5() {
    let mut grades: Vec<char> =  vec!['A'; 5];

    let mut grade_a: i8 = 0;
    let mut grade_b: i8 = 0;
    let mut grade_c: i8 = 0;
    let mut grade_d: i8 = 0;
    let mut grade_f: i8 = 0;

    for n in 1..6 {
        println!("Enter grade for subject {}", n);
        grades[n-1] = read!();
        match grades[n-1] {
            'A' => grade_a += 1,
            'B' => grade_b += 1,
            'C' => grade_c += 1,
            'D' => grade_d += 1,
            'F' => grade_f += 1,
            _ => println!("ERROR."),
        }
    }
    print!("Totals for each grade: \n");
    print!("A: {}", grade_a);
    print!("\nB: {}", grade_b);
    print!("\nC: {}", grade_c);
    print!("\nD: {}", grade_d);
    print!("\nF: {}", grade_f);
}