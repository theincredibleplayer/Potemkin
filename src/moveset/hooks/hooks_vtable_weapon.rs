use super::*;
use std::sync::LazyLock;



unsafe extern "C" fn waterball_on_hit(vtable:u64, weapon: *mut smash::app::Weapon, log: CollisionLog) -> usize {
    *(weapon as *mut bool).add(0x90) = true; //Lets the weapon persist on hit
    println!("on hit activated");
    if (AttackModule::is_infliction((*weapon).battle_object.module_accessor, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction((*weapon).battle_object.module_accessor, *COLLISION_KIND_MASK_SHIELD) || AttackModule::is_infliction((*weapon).battle_object.module_accessor, *COLLISION_KIND_MASK_ATTACK) )
    {
        println!("is infliction");
        if WorkModule::is_flag((*weapon).battle_object.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_DID_IT_HIT){
            println!("did it hit");
            WorkModule::off_flag((*weapon).battle_object.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_DID_IT_HIT);
            StatusModule::change_status_request_from_script((*weapon).battle_object.module_accessor, WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM, false);
        }
    }
    return get_original_weapon_vtable(WEAPON_KIND_KAMUI_FDB,WeaponVtableSlot::WeaponOnAttackMask);
}


pub fn install() {
    unsafe {
        replace_weapon_vtable(
            WEAPON_KIND_KAMUI_FDB, // weapon kind
            WeaponVtableSlot::WeaponOnAttackMask, // vtable slot
            waterball_on_hit as *const () as usize // replacement vtable function
        );

    }
}