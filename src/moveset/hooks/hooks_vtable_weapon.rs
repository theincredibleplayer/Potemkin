use super::*;
use std::sync::LazyLock;

static WATERBALL_ON_HIT                     : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*WEAPON_KIND_KAMUI_RYUSENSYA, WeaponVtableEntry::ON_ATTACK             , true, false));
static OFFSET_WATERBALL_ON_HIT              : LazyLock<usize> = LazyLock::new(|| get_vtable_function_offset(*WEAPON_KIND_KAMUI_RYUSENSYA, WeaponVtableEntry::ON_ATTACK             , true, true));

#[skyline::from_offset(*WATERBALL_ON_HIT)]
unsafe extern "C" fn waterball_on_hit_og(vtable:u64, weapon: *mut smash::app::Weapon, log: CollisionLog) -> u64 ;

unsafe extern "C" fn waterball_on_hit(vtable:u64, weapon: *mut smash::app::Weapon, log: CollisionLog) -> u64 {
    let owner_id = WorkModule::get_int((*weapon).battle_object.module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
    let owner_accessor = sv_battle_object::module_accessor(owner_id as u32);
    if is_potemkin(owner_accessor){
        *(weapon as *mut bool).add(0x90) = true; //Lets the weapon persist on hit
        if (AttackModule::is_infliction((*weapon).battle_object.module_accessor, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction((*weapon).battle_object.module_accessor, *COLLISION_KIND_MASK_SHIELD) || AttackModule::is_infliction((*weapon).battle_object.module_accessor, *COLLISION_KIND_MASK_ATTACK) )
        {
            if WorkModule::is_flag((*weapon).battle_object.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_DID_IT_HIT){
                WorkModule::off_flag((*weapon).battle_object.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_DID_IT_HIT);
                StatusModule::change_status_request_from_script((*weapon).battle_object.module_accessor, WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM, false);
            }
        }
        return waterball_on_hit_og(vtable, weapon, log);
    }else{
        return waterball_on_hit_og(vtable, weapon, log);
    }
}


pub fn install() {
    unsafe {
        fn patch_vtable_function(offset: usize, function: u64) {
            // println!("function ptr: {:#x}", function as u64);
            let low = (function as u64 & 0xFFFFFFFF) as u32;
            // println!("Low to Big Endian: {:#x} > {:#?}", low, low.to_be_bytes());
            let high = (function as u64 >> 32) as u32;
            // println!("High to Big Endian: {:#x} > {:#?}", high, high.to_be_bytes());
            let _ = skyline::patching::Patch::in_text(offset).data(low);
            let _ = skyline::patching::Patch::in_text(offset + 0x4).data(high);
        }

        LazyLock::force(&WATERBALL_ON_HIT);

        patch_vtable_function(*OFFSET_WATERBALL_ON_HIT             , waterball_on_hit as u64);

    }
}