use super::*;



pub fn install() {
    install_hooks!(
        kamui_link_event,
        noping_the_eff,
        handle_draw_potemkin,
        autoturn_situation_check_hook,
    );
    Patch::in_text(0xb616d0).nop(); //Nops the set pos at the end of final smash
    Patch::in_text(0xb5ccc0).nop().expect("Failed to patch 0xb5ccc0"); //Nops the sword effect
    Patch::in_text(0x3414534).nop(); //Nops the waterball effect
    unsafe {
        KAMUI_VTABLE_ENTRIES[0] = 0x4FC8B50;
        for i in 1..146 {
            KAMUI_VTABLE_ENTRIES[i] = KAMUI_VTABLE_ENTRIES[0] + (0x8 * (i as usize));
        }
        KAMUI_WEAPON_VTABLE_ENTRIES[0] = 0x4fc9b50; //INSERT WEAPON VTABLE START OFFSET
        for i in 1..102 {
            KAMUI_WEAPON_VTABLE_ENTRIES[i] = KAMUI_WEAPON_VTABLE_ENTRIES[0] + (0x8 * (i as usize));
        }
        patch_vtable_function(KAMUI_VTABLE_ENTRIES[VtableEntry::ON_FRAME_3],kamui_on_frame as u64);
        patch_vtable_function(KAMUI_VTABLE_ENTRIES[VtableEntry::ON_SEARCH],kamui_on_search as u64);
        patch_vtable_function(KAMUI_VTABLE_ENTRIES[VtableEntry::RESET_1], kamui_reset as u64);
        patch_vtable_function(KAMUI_VTABLE_ENTRIES[VtableEntry::INITIALIZE], kamui_init as u64);
        patch_vtable_function(KAMUI_VTABLE_ENTRIES[VtableEntry::ON_SHIELD_COLLISION], kamui_on_shield as u64);
        patch_vtable_function(KAMUI_VTABLE_ENTRIES[VtableEntry::ON_DAMAGE], kamui_on_damage as u64);
        patch_vtable_function(KAMUI_VTABLE_ENTRIES[VtableEntry::ON_ATTACK], kamui_on_attack as u64);
        patch_vtable_function(KAMUI_VTABLE_ENTRIES[VtableEntry::ON_CHANGE_MOTION], kamui_on_change_motion as u64);
        param_config_ig();
    }
}