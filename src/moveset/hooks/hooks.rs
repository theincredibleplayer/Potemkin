use super::*;




#[skyline::hook(offset = RESET_CLIFF_HANGDATA_HOOK_OFFSET.load(Ordering::Relaxed))]
unsafe extern "C" fn pot_select_cliff_hangdata_hook(ground_module: u64, hang_int: u32) -> f32 {
    let module_accessor = *((ground_module + 0x20) as *mut *mut BattleObjectModuleAccessor);
    //let battle_object = MiscModule::get_battle_object_from_id(boma.battle_object_id);
    //let module_accessor = (*battle_object).module_accessor;

    if is_potemkin(module_accessor) && (hang_int == 0){
        let agent = get_fighter_common_from_accessor((&mut*module_accessor));
        let something = *((ground_module+0xf0) as *mut f32);
        let scale = PostureModule::scale(module_accessor);
        let p1_x = 19.0;
        let p1_y = 32.0;
        let p2_x = -14.0;
        let p2_y = 12.0;
        let back_x   = (-p2_x)       * something * scale ;
        let back_y   = (p1_y - p2_y) * something * scale ;
        let front_x  = (p1_x)        * something * scale ;
        let front_y  = (p1_y - p2_y) * something * scale ;
        let center_x = 0.0;
        let center_y = p2_y * something * scale;

        agent.set_back_cliff_hangdata(back_x, back_y);
        agent.set_front_cliff_hangdata(front_x, front_y);
        agent.set_center_cliff_hangdata(center_x, center_y);
        return something;
    }else{
        original!()(ground_module, hang_int)
    }
}


#[skyline::hook(offset = AUTOTURN_SITUATION_CHECK_HOOK_OFFSET.load(Ordering::Relaxed), inline)]
unsafe extern "C" fn autoturn_situation_check_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = ctx.registers[21].x() as *mut BattleObjectModuleAccessor;
    let fighter_kind = utility::get_kind(&mut *(module_accessor));
    if fighter_kind == *FIGHTER_KIND_KAMUI {
        if is_potemkin(module_accessor){
            ctx.registers[0].set_x(0);
        }
    }
}

#[skyline::hook(offset = CAPTURE_2_HOOK_OFFSET.load(Ordering::Relaxed), inline)]
unsafe extern "C" fn potemkin_item_grab_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let grabber_battle_object = ctx.registers[0].x() as *mut BattleObject;
    let grabber_module_accessor = (*grabber_battle_object).module_accessor;
    let captured_original_status = ctx.registers[22].w() as i32;
    let status_kind = StatusModule::status_kind(grabber_module_accessor);
    //println!("original status = {:x}",captured_original_status);
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



#[skyline::hook(offset = WATERBALL_EFFECT_HANDELER_OFFSET.load(Ordering::Relaxed))]
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

#[skyline::hook(offset = KAMUI_SWORD_EFFECT_HANDELER_OFFSET.load(Ordering::Relaxed))]
unsafe extern "C" fn potemkin_noping_the_sword_eff(fighter: &mut Fighter) -> u64 {
    if is_potemkin(fighter.battle_object.module_accessor){
        return 0;
    }else{
        call_original!(fighter)
    }
}






pub fn install() {
    skyline::install_hooks!(
        noping_the_waterball_eff,
        potemkin_noping_the_sword_eff,
        autoturn_situation_check_hook,
        potemkin_item_grab_hook,
        pot_select_cliff_hangdata_hook
    );
}