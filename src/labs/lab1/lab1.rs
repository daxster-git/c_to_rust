#![allow(dead_code)]
#![allow(unused_variables)]


pub fn main() {
    // UART2_Initialize();

    print!("\tLAB 1\n=========================\n");
    print!("\tQUESTION 1A\n");
    print!("This is a test\n");
    print!("No this is not a test\n");
    print!("Goodbye\n");

    print!("\n\tQUESTION 1B\n");
    print!("Hello\n");
    print!("This is my first attempt ");
    print!("in Rust programming\n");
    print!("So give me your ");
    print!("valuable comments\n");

    print!("\n\tQUESTION 3A\n");
    // Declare friend names as strings //
    let friend_one: &str = "Devon";
    let friend_two: &str = "Wendi";

    // Declare friend address as string (both friends live in the same place) //
    let friend_address: &str = "2348 Selkirk Drive";
    print!("My first friend: {}, his address: {}\n\nMy second friend: {}, her address: {}", friend_one, friend_address, friend_two, friend_address);
}