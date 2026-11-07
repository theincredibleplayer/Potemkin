use super::*;
use std::sync::LazyLock;


unsafe extern "C" fn waterball_on_generate(vtable:u64, weapon: *mut smash::app::Weapon) -> usize {
    let module_accessor = (*weapon).battle_object.module_accessor;
    PostureModule::set_scale(module_accessor, 0.45, false);
    PostureModule::set_pos(module_accessor,&Vector3f{x:20.0,y:17.0,z:0.0});
    return 0;
}
unsafe extern "C" fn waterball_on_hit(vtable:u64, weapon: *mut smash::app::Weapon, log: CollisionLog) -> usize {
    *(weapon as *mut bool).add(0x90) = true; //Lets the weapon persist on hit
    if (AttackModule::is_infliction((*weapon).battle_object.module_accessor, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction((*weapon).battle_object.module_accessor, *COLLISION_KIND_MASK_SHIELD) || AttackModule::is_infliction((*weapon).battle_object.module_accessor, *COLLISION_KIND_MASK_ATTACK) )
    {
        if WorkModule::is_flag((*weapon).battle_object.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_DID_IT_HIT){
            WorkModule::off_flag((*weapon).battle_object.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_DID_IT_HIT);
            StatusModule::change_status_request_from_script((*weapon).battle_object.module_accessor, WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM, false);
        }
    }
    return 0;
}


pub fn install() {
    unsafe {
        replace_weapon_vtable(
            WEAPON_KIND_KAMUI_FDB, // weapon kind
            WeaponVtableSlot::WeaponOnAttackMask, // vtable slot
            waterball_on_hit as *const () as usize // replacement vtable function
        );
        replace_weapon_vtable(
            WEAPON_KIND_KAMUI_FDB, // weapon kind
            WeaponVtableSlot::WeaponGenerate1, // vtable slot
            waterball_on_generate as *const () as usize // replacement vtable function
        );
    }
}