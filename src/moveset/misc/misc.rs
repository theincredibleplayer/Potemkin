use super::*;


pub trait BomaExt {
    unsafe fn set_front_cliff_hangdata(&mut self, x: f32, y: f32);
    unsafe fn set_back_cliff_hangdata(&mut self, x: f32, y: f32);
    unsafe fn set_center_cliff_hangdata(&mut self, x: f32, y: f32);
}

impl BomaExt for BattleObjectModuleAccessor {
    unsafe fn set_front_cliff_hangdata(&mut self, x: f32, y: f32) {
        let ground_module = *(self as *mut BattleObjectModuleAccessor as *const u64).add(0x58 / 8);
        let ground_data = *((ground_module + 0x28) as *mut *mut f32);
        *ground_data.add(0x530 / 4) = x;
        *ground_data.add(0x534 / 4) = y;
    }

    /// Sets the position of the back/blue ledge-grab box (see [`set_center_cliff_hangdata`](BomaExt::set_center_cliff_hangdata) for more information)
    /// 
    /// # Arguments
    /// * `x` - The x coordinate, relative to the [position](smash::app::lua_bind::PostureModule::pos) of the fighter
    /// * `y` - The y coordinate, relative to the [position](smash::app::lua_bind::PostureModule::pos) of the fighter
    unsafe fn set_back_cliff_hangdata(&mut self, x: f32, y: f32) {
        let ground_module = *(self as *mut BattleObjectModuleAccessor as *const u64).add(0x58 / 8);
        let ground_data = *((ground_module + 0x28) as *mut *mut f32);
        *ground_data.add(0x540 / 4) = x;
        *ground_data.add(0x544 / 4) = y;
    }

    /// Sets the center position of the two ledge-grab boxes
    /// 
    /// # Information about hang data
    /// There are two rectangles which represent the ledge-grab data for a fighter. One of them is usually
    /// placed behind the fighter and the other in front. For the purposes of explanation, I will refer to
    /// the one in front as "red" and the one in the back as "blue", as those are the colors chosen
    /// for the visualizer. 
    /// 
    /// The center position for ledge-grab boxes is a point which both the red and blue boxes have as a corner.
    /// Both boxes meet at this location. This is usually located near the center of the fighter on the x-axis. The
    /// location on the y-axis of the fighter depends on the fighter.
    /// 
    /// The front and back positions (set by [`BomaExt::set_front_cliff_hangdata`] and [`BomaExt::set_back_cliff_hangdata`] respectively)
    /// are the corners that oppose this center.
    /// 
    /// # Arguments
    /// * `x` - The x coordinate, relative to the [position](smash::app::lua_bind::PostureModule::pos) of the fighter
    /// * `y` - The y coordinate, relative to the [position](smash::app::lua_bind::PostureModule::pos) of the fighter
    unsafe fn set_center_cliff_hangdata(&mut self, x: f32, y: f32) {
        let ground_module = *(self as *mut BattleObjectModuleAccessor as *const u64).add(0x58 / 8);
        let ground_data = *((ground_module + 0x28) as *mut *mut f32);
        *ground_data.add(0x520 / 4) = x;
        *ground_data.add(0x524 / 4) = y;
    }
}

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


    #[skyline::from_offset(GET_BATTLE_OBJECT_FROM_ID_OFFSET)]
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


pub fn get_battle_object_from_entry_id(entry_id: u32) -> Option<*mut BattleObject> {
    unsafe {
        let entry = FighterManager::get_fighter_entry(singletons::FighterManager(), FighterEntryID(entry_id as i32)) as *mut u8;
        if entry.is_null() {
            None
        } else {
            Some(*(entry.add(0x4160) as *mut *mut BattleObject))
        }
    }
}