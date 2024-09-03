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
        //macros::CATCH(agent, 0, Hash40::new("top"), 6.0, 0.0, 5.5, 9.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        //macros::CATCH(agent, 1, Hash40::new("top"), 7.0, 0.0, 4.0, 13.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::ATTACK(agent,0,0,Hash40::new("handr"),15.0,75,100,0,40,9.0,3.0,0.0,0.0,None,None,None,1.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        //grab!(agent, MA_MSC_CMD_GRAB_CLEAR_ALL);
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
        //let vec2 = Vector2f{x: 6.0, y: 6.0}; /*density?*/
        //let vec3 = Vector3f{x: 0.0, y: 60.0, z: -5.0}; /*x???, y is duration, z???? */
        //EffectModule::request_post_effect_line_circle(agent.module_accessor,Hash40::new("genesis_get"),Hash40::new("top"),vec2,vec3,true,1.0/*radius*/,0.0);
        let vec1 = Vector2f{x: 1.0, y: 15.0}; //idk seemingly does nothing
        let vec2 = Vector2f{x: 0.0, y: 10.0}; // y is the distance which doesnt have the lines (relative to top?) x seems to be the same?
        let vec3 = Vector2f{x: 30.0 /*duration*/ , y: 10.0 /*fade out*/ }; //duration and fade out
        let vec4 = Vector2f{x: 0.0, y: 1.0}; //idk seemingly does nothing
        let speedlines = EffectModule::request_post_effect_line_parallel_2d(agent.module_accessor,Hash40::new("dead_area_out"),vec1,vec2,vec3,vec4,false,50.0 /*idfk*/,1.0 /*idfk too*/);
        //EffectModule::set_rot(agent.module_accessor, speedlines as u32, &Vector3f{x:50.0,y:0.0,z:30.0});
        //EffectModule::set_scale(agent.module_accessor, speedlines as u32, &Vector3f{x:0.5,y:0.5,z:0.5});
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



