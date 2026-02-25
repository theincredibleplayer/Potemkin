use super::*;

//vtable patching stuff
pub static mut KAMUI_VTABLE_ENTRIES: [usize; 146] = [0; 146];
pub static mut KAMUI_WEAPON_VTABLE_ENTRIES: [usize; 102] = [0; 102];