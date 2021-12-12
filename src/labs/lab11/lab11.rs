#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use text_io::read;

#[derive(Debug)]
struct Video {
    title: String,
    length: i8,
    tape_cost: f32,
    tape_rent: f32,
}

pub fn main() {
    let mut tapes: Vec<Video> = Vec::new();
    println!("Enter title: ");
    let title = read!();
    println!("Enter length: ");
    let length = read!();
    println!("Enter tape cost: ");
    let tape_cost = read!();
    println!("Enter tape rent: ");
    let tape_rent = read!();
    
    
    let video = question_2a(title, length, tape_cost, tape_rent);
    tapes.push(video);
    println!("{:?}", video);
}

fn question_2a(title: String, length: i8, tape_cost: f32, tape_rent: f32) -> Video {
    let video = Video{
        title: String::from(title),
        length: length,
        tape_cost: tape_cost,
        tape_rent: tape_rent,
    };
    return video;
}