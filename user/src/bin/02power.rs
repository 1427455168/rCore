#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

const SIZE: usize = 10;
const P: u32 = 3;
const STEP: usize = 1_0000_0;
const MOD: u32 = 1_0007;


#[no_mangle]
fn main() -> i32 {
    let mut pow = [0_u32; SIZE];
    let mut index: usize = 0;
    pow[index] = 1;

    for i in 1..=STEP {
        let last = pow[index];
        index = (index + 1) % SIZE;
        pow[index] = last * P % MOD;

        if i % 1_0000 == 0 {
            println!("{}^{}={}(MOD {})", P, i, pow[index], MOD);
        }
    }

    println!("Test power Ok!");
    0
}