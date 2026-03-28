use super::*;




#[allow(non_snake_case)]
pub mod MiscModule {
    use super::*;
    
    pub unsafe fn add_shield_group(module_accessor: *mut BattleObjectModuleAccessor, resource: *mut ShieldGroupResource, group_id: i32) {
        // Get Vtable Function
        let ptr = get_module_vtable_func(module_accessor, 0x100, 0x58);
        let set_shield_group: extern "C" fn(*mut u64, *mut ShieldGroupResource, i32) = std::mem::transmute(ptr);

        // Redefine ShieldModule
        let shield_module = *(module_accessor as *mut *mut u64).add(0x100 / 8);

        set_shield_group(shield_module, resource, group_id);
        let count = (*resource).count as i32;
        if count > 0 {
            for x in 0..count {
                ShieldModule::set_status(module_accessor, x, ShieldStatus(*SHIELD_STATUS_NONE), group_id);
            }
        }
    }

    pub unsafe fn get_module_vtable_func(module_accessor: *mut BattleObjectModuleAccessor, module_offset: usize, func_offset: u64) -> u64 {
        let module = (module_accessor as *mut u64).add(module_offset / 0x8);
        let vtable = *module as *const u64;
        *((*vtable + func_offset) as *const u64)
    }


    #[skyline::from_offset(0x3ac540)]
    pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;
}




pub fn get_fighter_common_from_accessor<'a>(boma: &'a mut BattleObjectModuleAccessor) -> &'a mut L2CFighterCommon {
    unsafe {
        let lua_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CFighterCommon))
    }
}

pub fn is_valid_entry_id(entry_id: i32) -> bool {
    unsafe {
        let entry = smash::app::lua_bind::FighterManager::get_fighter_entry(singletons::FighterManager(), smash::app::FighterEntryID(entry_id)) as *mut smash::app::FighterEntry;
        if entry.is_null() {
            false
        } else {
            true
        }
    }
}

pub fn get_ui_index_from_entry_id(entry_id: usize) -> usize {
    // start at index 0
    let mut index = 0;

    // check all of the possible entry IDs less than or equal to this one,
    // and see how many "slots" should be filled, counting up from 0.
    for n in 0..entry_id {
        if is_valid_entry_id(n as i32) {
            // this is a valid fighter in this match, which means they will
            // be occupying a UI slot. Thus, we cannot take that slot.
            index += 1;
        }
    }
    return index;
}