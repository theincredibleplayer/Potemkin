use crate::moveset::*;



pub unsafe extern "C" fn special_air_hi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("bust"), 10.0, 70, 80, 0, 70, 12.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("handr"), 10.0, 70, 80, 0, 70, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 0, Hash40::new("handl"), 10.0, 70, 80, 0, 70, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
}
pub unsafe extern "C" fn sound_special_air_hi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_heat_tackle_X".to_string(),3,false,false,0);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_16"));
    }
}
pub unsafe extern "C" fn effect_special_air_hi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ARMOR_ON);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0); 

        for _ in 0..5 {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_jetpack"), Hash40::new("arml"), 0, 0, 0, -90, 90, 0, 0.7, true);
                macros::LAST_EFFECT_SET_RATE(agent, 1.0);
                macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_jetpack"), Hash40::new("armr"), 0, 0, 0, -90, 90, 0, 0.7, true);
                macros::LAST_EFFECT_SET_RATE(agent, 1.0);
              }
            wait(agent.lua_state_agent, 5.0);
        }
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ARMOR_ON);
    }
}
pub unsafe extern "C" fn expression_special_air_hi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
    }
}
