use super::*;



mod hooks_vtable;
use hooks_vtable::*;
mod hooks_vtable_weapon;
use hooks_vtable_weapon::*;

mod hooks;
use hooks::*;

//vtable patching stuff
pub static mut KAMUI_VTABLE_ENTRIES: [usize; 146] = [0; 146];
pub static mut KAMUI_WEAPON_VTABLE_ENTRIES: [usize; 102] = [0; 102];




pub fn install() {
    hooks::install();
    hooks_vtable::install();
    hooks_vtable_weapon::install();
}