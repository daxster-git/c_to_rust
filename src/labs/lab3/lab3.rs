#![allow(dead_code)]
#![allow(unused_variables)]

use text_io::read;

pub fn main() {
    println!("Enter ccode1: ");
    let ccode_1: char = read!();
    println!("Enter ccode2: ");
    let ccode_2: char = read!();
    println!("Enter iqty1: ");
    let iqty_1: i16 = read!();
    println!("Enter iqty2: ");
    let iqty_2: i16 = read!();
    println!("Enter fprice1: ");
    let fprice_1: f32 = read!();
    println!("Enter fprice2: ");
    let fprice_2: f32 = read!();
    
    print!("CODE\tQUANTITY\tU_PRICE\n");
    print!("   {}\t    {}\t {}\n", ccode_1, iqty_1, fprice_1);
    print!("   {}\t    {}\t {}", ccode_2, iqty_2, fprice_2);
}