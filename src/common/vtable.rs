use super::*;

pub fn patch_vtable_function(offset: usize, function: u64) {
    //println!("function ptr: {:#x}", function as u64);
    let low = (function as u64 & 0xFFFFFFFF) as u32;
    //println!("Low to Big Endian: {:#x} > {:#?}", low, low.to_be_bytes());
    let high = (function as u64 >> 32) as u32;
    //println!("High to Big Endian: {:#x} > {:#?}", high, high.to_be_bytes());
    let _ = skyline::patching::Patch::in_text(offset).data(low);
    let _ = skyline::patching::Patch::in_text(offset + 0x4).data(high);
}