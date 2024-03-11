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

pub unsafe extern "C" fn attack_s3(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
    }
    sv_animcmd::frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent,0,0,Hash40::new("handr"),15.0,75,100,0,40,9.0,3.0,0.0,0.0,None,None,None,1.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    sv_animcmd::frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

pub unsafe extern "C" fn effect_attack_s3(_agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn attack_lw3(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent,0,0,Hash40::new("footl"),10.0,75,30,0,70,4.0,3.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_NONE);
        macros::ATTACK(agent,1,0,Hash40::new("kneel"),8.0,75,30,0,70,4.0,3.0,0.0,0.0,Some(0.0),Some(3.0),Some(0.0),1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_NONE);
        macros::ATTACK(agent,2,0,Hash40::new("legl"),6.0,80,30,0,70,5.0,3.0,0.0,0.0,Some(0.0),Some(3.0),Some(0.0),1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_NONE);
        macros::ATTACK(agent,3,0,Hash40::new("handl"),8.0,130,50,0,10,5.0,3.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_NONE);
        macros::ATTACK(agent,4,0,Hash40::new("handr"),8.0,130,50,0,10,5.0,3.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub unsafe extern "C" fn effect_attack_lw3(_agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn attack_dash(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent,0,0,Hash40::new("top"),15.0,50,50,0,80,9.0,0.0,7.0,10.0,Some(0.0),Some(25.0),Some(10.0),1.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_HEAVY,*ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub unsafe extern "C" fn effect_attack_dash(_agent: &mut L2CAgentBase) {
    macros::EFFECT_FOLLOW(_agent, Hash40::new("sys_togezoshell_speedline"), Hash40::new("top"), 0, 15, 8, -90, 0, 0, 1.5, true);
}

pub unsafe extern "C" fn catch(agent: &mut L2CAgentBase) {
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
    }
}

pub unsafe extern "C" fn catchdash(agent: &mut L2CAgentBase) {
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

pub unsafe extern "C" fn attack_hi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 15.0, 93, 91, 0, 50, 8.0, 0.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 10.0, 93, 91, 0, 50, 8.0, 0.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 13.0, 80, 91, 0, 50, 8.0, 0.0, 7.0, 8.0, Some(0.0), Some(12.0), Some(8.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub unsafe extern "C" fn sound_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    wait(agent.lua_state_agent, 1.0);
    WorkModule::get_float(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_HOLD_RATE);
    //if(0x14e230(1026626521, 1)){
        //if macros::is_excute(agent) {
          //  macros::PLAY_SEQUENCE(agent, Hash40::new("seq_kamui_rnd_attack_smash_h"));
        //}
      //  else{
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_kamui_attack06"));
        }
    //}
//}
if macros::is_excute(agent) {
    macros::PLAY_SE(agent, Hash40::new("se_kamui_smash_h01"));
}
wait(agent.lua_state_agent, 8.0);
if macros::is_excute(agent) {
    macros::PLAY_SE(agent, Hash40::new("se_kamui_smash_h02"));
}
}

pub unsafe extern "C" fn expression_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
}

pub unsafe extern "C" fn effect_attack_hi4(_agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn effect_attack_hi4_charge(_agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn attack_s4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    execute(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 20.0, 30, 90, 0, 60, 8.0, 0.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 18.0, 30, 90, 0, 60, 8.0, 0.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 20.0, 30, 90, 0, 60, 8.0, 0.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 3, 0, Hash40::new("shoulderr"), 18.0, 30, 90, 0, 60, 8.0, 0.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub unsafe extern "C" fn sound_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
    }
    wait(agent.lua_state_agent, 4.0);
    WorkModule::get_float(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_HOLD_RATE);
    //if(0x14e230(1026626521, 1)){
        //if macros::is_excute(agent) {
          //  macros::PLAY_SEQUENCE(agent, Hash40::new("seq_kamui_rnd_attack_smash_s"));
      //  }
        //else{
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_kamui_attack07"));
        }
    //}
//}
}

pub unsafe extern "C" fn sound_attacks4charge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
}

pub unsafe extern "C" fn expression_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
}

pub unsafe extern "C" fn spear_attack_s4(agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn effect_attack_s4(_agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn effect_attack_s4_charge(_agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn effect_attack_s4_charge_spear(_agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn game_final(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        CameraModule::reset_all(agent.module_accessor);
        macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
        macros::SLOW_OPPONENT(agent, 900.0, 60.0);
        ArticleModule::generate_article(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,false,-1);
        ArticleModule::change_motion(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("final"),false,-1.0);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) == false {
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::FT_SET_FINAL_FEAR_FACE(agent, 60);
            macros::REQ_FINAL_START_CAMERA_arg3(agent,Hash40::new("d04final.nuanmb"), false, false);
            macros::FT_START_CUTIN(agent);
        }
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        //SlowModule::set_whole(agent.module_accessor, 30, 0);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        SlowModule::clear_whole(agent.module_accessor);
    }
    else{
    if macros::is_excute(agent) {
        macros::FT_START_CUTIN(agent);
    }
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        //macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 80, 100, 180, 0, 10.0, 0.0, 5.0, 12.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        //macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 80, 100, 180, 0, 10.0, 0.0, 5.0, -12.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        //AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        //macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 80, 100, 180, 0, 8.0, 0.0, 1.0, 12.0, Some(0.0), Some(27.0), Some(12.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        //macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 80, 100, 180, 0, 8.0, 0.0, 1.0, -12.0, Some(0.0), Some(27.0), Some(-12.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        //AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        //macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 80, 100, 180, 0, 5.0, 0.0, 1.0, 12.0, Some(0.0), Some(45.0), Some(12.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        //macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 80, 100, 180, 0, 5.0, 0.0, 1.0, -12.0, Some(0.0), Some(45.0), Some(-12.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        //AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_CLEAR_ATTACK);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        SlowModule::clear_whole(agent.module_accessor);
        CameraModule::zoom_out(agent.module_accessor, 30);
    }
    frame(agent.lua_state_agent, 76.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_CHANGE_TO_FS_DASH);
    }
}

pub unsafe extern "C" fn effect_final(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::req_screen(agent.module_accessor, Hash40::new("bg_kamui_final"), false, false, false);
        macros::EFFECT_FLW_POS(agent, Hash40::new("kamui_transform_splash_start"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0  {
        if macros::is_excute(agent) {
            //macros::EFFECT(agent, Hash40::new("kamui_final_energy"), Hash40::new("top"), 0, 0.5, 12, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, true);
            //macros::EFFECT(agent, Hash40::new("kamui_final_energy"), Hash40::new("top"), 0, 0.5, -12, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, true);
            for boi in 1..12 {
                macros::EFFECT(agent, Hash40::new("sys_dragoon_a"), Hash40::new("top"), 0, 12, 0, 0, 90, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
            }
        }
    }
    else{
        if macros::is_excute(agent) {
            //macros::EFFECT(agent, Hash40::new("kamui_final_energy"), Hash40::new("top"), 0, 0.5, 12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            //macros::EFFECT(agent, Hash40::new("kamui_final_energy"), Hash40::new("top"), 0, 0.5, -12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            for boi in 1..12 {
              macros::EFFECT(agent, Hash40::new("sys_dragoon_a"), Hash40::new("top"), 0, 12, 0, 0, 90, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
            }
        }
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("kamui_final_start_flash"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
    }

    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        //macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), -12, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        //macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FLW_POS(agent, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 90.0);
    if macros::is_excute(agent) {
        EffectModule::remove_screen(agent.module_accessor, Hash40::new("bg_kamui_final"), -1);
    }
}

pub unsafe extern "C" fn sound_final(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_kamui_final01"));
        macros::PLAY_SE(agent, Hash40::new("se_kamui_final03"));
    }
}


pub unsafe extern "C" fn game_finalvisualscene01(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        CameraModule::zoom_out(agent.module_accessor, 0);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, -1);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40::new("final_visual_scene_01"), false, -1.0);
    }
}
pub unsafe extern "C" fn sound_finalvisualscene01(agent: &mut L2CAgentBase) {
}

pub unsafe extern "C" fn game_fakefinalvisualscene01(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 0.0, 80, 100, 180, 0, 30.0, 0.0, 1.0, 12.0, Some(0.0), Some(0.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, -1);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40::new("final_visual_scene_01"), false, -1.0);
    }
    frame(agent.lua_state_agent, 229.0);
    if macros::is_excute(agent) {
        for _ in 0..6 {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("robot_atk_lw_jet"), Hash40::new("hip"), -10, -2, -15, -90, 90, 0, 0.8, true);
                macros::LAST_EFFECT_SET_RATE(agent, 0.8);
                macros::EFFECT_FOLLOW(agent, Hash40::new("robot_atk_lw_jet"), Hash40::new("hip"), -10, -2, 15, -90, 90, 0, 0.8, true);
                macros::LAST_EFFECT_SET_RATE(agent, 1.0);
              }
            wait(agent.lua_state_agent, 7.0);
        }
    }
}

pub unsafe extern "C" fn sound_fakefinalvisualscene01(agent: &mut L2CAgentBase) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("heavenly_1_1"));
            macros::PLAY_SE(agent, Hash40::new("se_kamui_final01"));
        }
        frame(agent.lua_state_agent, 120.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("heavenly_1_2"));
        }
        frame(agent.lua_state_agent, 240.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("heavenly_1_3"));
        }
}


pub unsafe extern "C" fn game_finalvisualscene02(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //JostleModule::set_status(agent.module_accessor, false);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, -1);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40::new("final_visual_scene_02"), false, -1.0);
        ArticleModule::generate_article(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND,false,-1);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, Hash40::new("final_visual_scene_02"), false, -1.0);
        let plane = ArticleModule::get_article(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND);
        let plane_boma = *(plane as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
        LinkModule::remove_model_constraint(plane_boma,true);
        LinkModule::set_model_constraint_pos_ort(plane_boma,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("attach"),Hash40::new("top"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32 | *CONSTRAINT_FLAG_OFFSET_ROT as u32,true); 
        macros::FT_SET_FINAL_FEAR_FACE(agent, 150); 
    }
    frame(agent.lua_state_agent, 71.0);
    if macros::is_excute(agent) {
        //VisibilityModule::set_whole(agent.module_accessor, false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFFECT_SWORD_AURA);
    }
}

pub unsafe extern "C" fn sound_finalvisualscene02(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 70.0);
    if macros::is_excute(agent) {
    let module_accessor = agent.module_accessor;
    let enemy_id = WorkModule::get_int(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_HPB_GOTTEM_ID);
    let enemy = sv_battle_object::module_accessor(enemy_id as u32);
    let enemy_kind = sv_battle_object::kind(enemy_id as u32);
    let name = LOWERCASE_FIGHTER_NAMES.get(enemy_kind as usize).unwrap();
    let result = ["vc_",name,"_damage_twinkle"].join("");
    let scream_handle = SoundModule::play_se(enemy, Hash40::new(result.as_str()), true, false, false, false, enSEType(0));
    SoundModule::set_se_vol(enemy, scream_handle as i32, 2.0, 0);

    }
    frame(agent.lua_state_agent, 130.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("heavenly_1_4"));
    }
}
pub unsafe extern "C" fn effect_finalvisualscene02(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("hip"), 0, -10, 0, 0.0, 0.0, 0.0, 15, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
        EffectModule::set_alpha_last(agent.module_accessor, 0.5);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("hip"), 0, -10, 0, 0.0, 0.0, 0.0, 10, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.4);
        EffectModule::set_alpha_last(agent.module_accessor, 0.5);
    }
}

pub unsafe extern "C" fn game_finalvisualscene03(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_KAMUI_FINAL, 0, 1.5, 361, 80, 1, 0, 0.0, 1.0, *ATTACK_LR_CHECK_POS, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(agent.module_accessor, true, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
    }
    frame(agent.lua_state_agent, 52.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
    }
    frame(agent.lua_state_agent, 57.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
    }
    frame(agent.lua_state_agent, 62.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
    }
    frame(agent.lua_state_agent, 67.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
    }
    frame(agent.lua_state_agent, 74.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
    }
}

pub unsafe extern "C" fn sound_finalvisualscene03(agent: &mut L2CAgentBase) {
}

pub unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 35, 80, 0, 50, 28.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 0, 10, 0, 10, 100.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub unsafe extern "C" fn game_win1a(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,false,-1);
        ArticleModule::change_motion(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("win_1"),false,-1.0);
    }
}



