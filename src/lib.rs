#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![allow(warnings)]

pub static mut FIGHTER_MANAGER: usize = 0;
pub static mut FIGHTER_CUTIN_MANAGER_ADDR: usize = 0;
pub static mut ITEM_MANAGER : usize = 0;


mod moveset;

#[skyline::main(name = "hotpotwithudon")]
pub fn main() {
    moveset::install();
}