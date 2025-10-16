use skyline_smash::app::sv_fighter_util::get_kirifuda_position;
use smash::lib::{L2CValue,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon};
use smash::hash40;
use smash::app::lua_bind::*;
use smashline::*;
use smash::app::*;
use smash_script::macros::*;
use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::phx::{Vector2f,Vector3f};
use skyline::nn::ro::LookupSymbol;
use smash_script::lua_args;
use smash_script::macros;
use std::arch::asm;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use crate::FIGHTER_MANAGER;
use crate::moveset::*;



pub unsafe extern "C" fn game_turn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 0.3);
    }
}


pub unsafe extern "C" fn catch(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 0.6);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 6.1, 0.0, 9.0, 7.0, Some(0.0), Some(9.0), Some(12.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 4.55, 0.0, 9.0, 5.45, Some(0.0), Some(9.0), Some(15.25), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        smash_script::grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
        macros::FT_MOTION_RATE(agent, 1.0);
    }
}

pub unsafe extern "C" fn catchdash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 0.6);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 6.5, 0.0, 9.0, 7.0, Some(0.0), Some(9.0), Some(14.3), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 4.25, 0.0, 9.0, 5.75, Some(0.0), Some(9.0), Some(15.55), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        smash_script::grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
        macros::FT_MOTION_RATE(agent, 1.0);

    }
}

pub unsafe extern "C" fn catchturn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 6.1, 0.0, 6.0, -4.0, Some(0.0), Some(6.0), Some(-14.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 4.55, 0.0, 6.0, -2.45, Some(0.0), Some(6.0), Some(-16.15), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        smash_script::grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}
pub unsafe extern "C" fn game_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.8, 361, 100, 30, 0, 5.0, 0.0, 13.0, 13.5, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub unsafe extern "C" fn game_entryl(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40::new("actual_entry"), false, -1.0);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("g_head_root"), 0, -3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.6);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 100.0);
    if macros::is_excute(agent) {
        ArticleModule::remove(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub unsafe extern "C" fn effect_entryl(agent: &mut L2CAgentBase) {

}

pub unsafe extern "C" fn sound_entryl(agent: &mut L2CAgentBase) {

}


pub unsafe extern "C" fn game_cliffattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 45, 20, 0, 90, 5.5, 0.0, 5.5, 20.0, Some(0.0), Some(5.5), Some(-5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub unsafe extern "C" fn effect_cliffattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 17, -22, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}


pub unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 60, 80, 0, 25, 7.0, 0.0, 9.0, 15.0, Some(0.0), Some(20.0), Some(15.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }

}

pub unsafe extern "C" fn effect_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

pub unsafe extern "C" fn sound_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        attack_sound_randomizer(agent,"light");
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_14"));
    }
}




pub unsafe extern "C" fn effect_walkslow(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 4, 0, 6, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 57.0);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, false);
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

pub unsafe extern "C" fn sound_walkslow(agent: &mut L2CAgentBase) {
    loop{
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP_FLIPPABLE(agent, Hash40::new("se_btl_pot_09"), Hash40::new("se_btl_pot_09"));
        }
        frame(agent.lua_state_agent, 57.0);
        macros::PLAY_STEP_FLIPPABLE(agent, Hash40::new("se_btl_pot_09"), Hash40::new("se_btl_pot_09"));
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

pub unsafe extern "C" fn expression_walkslow(agent: &mut L2CAgentBase) {
    loop{
        if macros::is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
            macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S_HALF);
        }
        frame(agent.lua_state_agent, 58.0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S_HALF);
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}


pub unsafe extern "C" fn effect_walkslowb(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -4.5, 0, -4.5, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 65.0);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -6, 0, 3, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, false);
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

pub unsafe extern "C" fn sound_walkslowb(agent: &mut L2CAgentBase) {
    loop{
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP_FLIPPABLE(agent, Hash40::new("se_btl_pot_09"), Hash40::new("se_btl_pot_09"));
        }
        frame(agent.lua_state_agent, 65.0);
        macros::PLAY_STEP_FLIPPABLE(agent, Hash40::new("se_btl_pot_09"), Hash40::new("se_btl_pot_09"));
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

pub unsafe extern "C" fn expression_walkslowb(agent: &mut L2CAgentBase) {
    loop{
        if macros::is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
            macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S_HALF);
        }
        frame(agent.lua_state_agent, 66.0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S_HALF);
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}




pub unsafe extern "C" fn game_slipattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 50, 0, 60, 4.5, 0.0, 4.5, 16.0, Some(0.0), Some(4.5), Some(-16.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 361, 50, 0, 60, 6.0, 0.0, 9.0, 10.0, Some(0.0), Some(9.0), Some(-10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub unsafe extern "C" fn effect_slipattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.4);
    }
}

pub unsafe extern "C" fn sound_slipattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_14"));
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_09"));
    }
}

pub unsafe extern "C" fn expression_slipattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}



pub unsafe extern "C" fn sound_wait2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_08"));
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_08"));
    }
    frame(agent.lua_state_agent, 76.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_08"));
    }
}
