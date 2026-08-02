use super::*;


#[skyline::hook(offset = 0xb5f380)]
unsafe extern "C" fn kamui_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash_rs::app::LinkEvent) -> u64 {
    if is_potemkin(fighter.battle_object.module_accessor){
        if event.link_event_kind.0 == hash40("capture") {
            let capture_event : &mut smash_rs::app::LinkEventCapture = std::mem::transmute(event);
            let module_accessor = fighter.battle_object.module_accessor;
            //upair linkmodule
            let opponent_category = sv_battle_object::category(capture_event.sender_id);
            let opponent_boma = sv_battle_object::module_accessor(capture_event.sender_id as u32);
            // println!("what's up");
                if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_AIR {
                    StatusModule::change_status_request(module_accessor, FIGHTER_KAMUI_STATUS_KIND_AIR_THROW, false);
                    capture_event.result = true;
                    capture_event.node = smash_rs::phx::Hash40::new("throw");
                    return 0;
                }
                //potemkin buster linkmodule
                if StatusModule::status_kind(module_accessor) == FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER_WHIFF {
                    StatusModule::change_status_request(module_accessor, FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER, false);
                    capture_event.result = true;
                    capture_event.node = smash_rs::phx::Hash40::new("throw");

                    //capture_event.ignore_distance = true;
                    return 0;
                }
                //special_hi's link module
                if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
                    StatusModule::change_status_request(module_accessor, FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_START, false);
                    capture_event.result = true;
                    capture_event.node = smash_rs::phx::Hash40::new("throw");
                    //capture_event.is_motion_hi_lw = false;
                    return 0;
                }
                //final smash's link module
            if opponent_category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                if StatusModule::status_kind(module_accessor) == FIGHTER_KAMUI_STATUS_KIND_FINAL_JUMP
                || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_FINAL {
                    PostureModule::set_pos(module_accessor,&Vector3f{x: 0.0 , y: 0.0, z: 100.0});
                    WorkModule::off_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_JUMP);

                    WorkModule::off_flag(module_accessor ,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_START);
                    WorkModule::off_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_START);
                    WorkModule::off_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_ATTACK_START);
                    WorkModule::off_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_JUMP_START);

                    StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_FINAL, false);      

                    capture_event.result = true;
                    capture_event.node = smash_rs::phx::Hash40::new("throw");
                    return 0;
                }
            }
            return 1;
        }
    }
    original!()(vtable, fighter, event)
}



#[skyline::hook(offset = 0x69a728, inline)]
unsafe extern "C" fn autoturn_situation_check_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = ctx.registers[21].x() as *mut BattleObjectModuleAccessor;
    let fighter_kind = utility::get_kind(&mut *(module_accessor));
    if fighter_kind == *FIGHTER_KIND_KAMUI {
        if is_potemkin(module_accessor){
            ctx.registers[0].set_x(0);
        }
    }
}

#[skyline::hook(offset = 0x3ede68, inline)]
unsafe extern "C" fn potemkin_item_grab_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let grabber_battle_object = ctx.registers[0].x() as *mut BattleObject;
    let grabber_module_accessor = (*grabber_battle_object).module_accessor;
    let captured_original_status = ctx.registers[22].w() as i32;
    let status_kind = StatusModule::status_kind(grabber_module_accessor);
    println!("original status = {:x}",captured_original_status);
    if is_potemkin(grabber_module_accessor){
        if [*FIGHTER_STATUS_KIND_ATTACK_AIR
        ,FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER_WHIFF
        ,*FIGHTER_STATUS_KIND_SPECIAL_HI
        ,FIGHTER_KAMUI_STATUS_KIND_FINAL_JUMP
        ,*FIGHTER_STATUS_KIND_FINAL
        ].contains(&status_kind){
            if [0x9].contains(&captured_original_status) { // this check is basically to check if this is an item or not
                ctx.registers[22].set_w(*ITEM_STATUS_KIND_CAPTURE_DEMON_DIVED as u32);
            } 
            if [0x2].contains(&captured_original_status) { // this check is to check if enemy or not
                ctx.registers[22].set_w(0x22);
            }
        }
    }
}

// consider moving this to like init or something
#[skyline::hook(offset = 0x470b90, inline)]
unsafe extern "C" fn change_item_hold_hand_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let item_module = ctx.registers[24].x() as *const u64;
    let mut item_hash_ptr = (item_module as u64 + 0x20) as *mut *mut u64 ;
    let module_accessor = (item_module as u64 + 0x8) as *mut *mut BattleObjectModuleAccessor;

    if is_potemkin(*module_accessor){
        **item_hash_ptr = 0x051a07c0e7;
    }else{
        **item_hash_ptr = 0x05e008fd84;
    }
}

#[skyline::hook(offset = 0x3414310)]
unsafe extern "C" fn noping_the_waterball_eff(unk0:u64, weapon: *mut smash::app::Weapon, unk2: u64) -> u64 {
    let module_accessor = (*weapon).battle_object.module_accessor;
    let owner_id = WorkModule::get_int((*weapon).battle_object.module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
    let owner_accessor = sv_battle_object::module_accessor(owner_id as u32);
    if is_potemkin(owner_accessor){
        GroundModule::set_correct(module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        GroundModule::set_cliff_check(module_accessor,GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE));
        GroundModule::set_attach_ground(module_accessor, false);
        PostureModule::set_scale(module_accessor, 0.45, false);
        return 0;
    }else{
        call_original!(unk0, weapon, unk2)
    }
}

#[skyline::hook(offset = 0xb5cbf0)]
unsafe extern "C" fn noping_the_sword_eff(fighter: &mut Fighter) -> u64 {
    if is_potemkin(fighter.battle_object.module_accessor){
        return 0;
    }else{
        call_original!(fighter)
    }
}

#[skyline::hook(offset = 0x34146c0)]
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
        call_original!(vtable, weapon, log)
    }else{
        call_original!(vtable, weapon, log)
    }
}




pub fn install() {
    skyline::install_hooks!(
        kamui_link_event,
        waterball_on_hit,
        noping_the_waterball_eff,
        noping_the_sword_eff,
        autoturn_situation_check_hook,
        potemkin_item_grab_hook,
        change_item_hold_hand_hook
    );
}