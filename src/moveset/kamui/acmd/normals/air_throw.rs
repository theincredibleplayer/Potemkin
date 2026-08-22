
use super::*;







unsafe extern "C" fn game_throwair(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 270, 78, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 78, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 3.0, 80, 80, 0, 50,7.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, 10, 10);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 2.0);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 10.0, y: 10.0, z: 0.0});
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        let capture_id = LinkModule::get_node_object_id(agent.module_accessor, *LINK_NO_CAPTURE);
        if capture_id != 0x50000000 {
            println!("releasing attack");
            AttackModule::hit_absolute_joint(agent.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,capture_id as u32,Hash40::new("throw"), 0, 0);
        }
    }
}

unsafe extern "C" fn sound_throwair(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("vc_pot_busterwhiff_a"));
        macros::STOP_SE(agent, Hash40::new("vc_pot_busterwhiff_b"));
        macros::STOP_SE(agent, Hash40::new("vc_pot_busterwhiff_c"));
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_impact_catch_large"));
    }
}





pub fn install(agent: &mut Agent) {
    agent.game_acmd     ("game_throwair",       game_throwair,      Priority::Low);
    agent.sound_acmd    ("sound_throwair",      sound_throwair,     Priority::Low);
    //.effect_acmd        ("effect_throwair",             effect_throwair,                           Priority::Low)
    //.expression_acmd    ("expression_throwair",         expression_throwair,                        Priority::Low)
}