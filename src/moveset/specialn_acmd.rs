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


pub unsafe extern "C" fn game_regular(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let owner_id = WorkModule::get_int(agent.module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
        println!("owner id: {}",owner_id);
        let owner_accessor = sv_battle_object::module_accessor(owner_id as u32);
        let mut damage = WorkModule::get_float(owner_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_REAL_POWER_REFLECT);
        if damage < 10.0{
            damage = 10.0/1.4;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"),  damage * 1.4 ,
         55, 90, 0, 70, 25.0, 0.0, 0.0, 0.0, None, None, None,
         2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false,
         0, 0.0, 0, true, true
         , false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
         AttackModule::disable_tip(agent.module_accessor);

    }
    //frame(agent.lua_state_agent, 60.0);
    //if macros::is_excute(agent) {
    //    WorkModule::on_flag(agent.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_ATTACK);
    //}
}

pub unsafe extern "C" fn effect_regular(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_fdb_fire"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, true);
        //EffectModule::set_rate_last(agent.module_accessor, 0.5);
        let random_number = smash::app::sv_math::rand(hash40("fighter"), 3);
        let mut eff_string = "sys_bomb_a";
        if random_number == 0{
            let mut eff_string = "se_common_fire_s";
        } else if random_number == 1 {
            let mut eff_string = "se_common_fire_l";
        } else {
            let mut eff_string = "se_common_fire_m";
        }
        macros::EFFECT_FLW_POS(agent, Hash40::new("master_bow_arrow2_hit"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 180, 0, 1, true);
        //macros::EFFECT(agent, Hash40::new(eff_string), Hash40::new("top"),0, 0, 0, 0, 0, 0, 1.0 + (smash::app::sv_math::rand(hash40("fighter"), 3) as f32 * 0.5), 0, 0, 0, 0, 0, 0, false);
    }
    for _ in 0..99{
        sv_animcmd::wait(agent.lua_state_agent, 5.0/* + (smash::app::sv_math::rand(hash40("fighter"), 3) * 2 ) ) as f32*/);
        if macros::is_excute(agent) {
            let random_number = smash::app::sv_math::rand(hash40("fighter"), 3);
            let mut eff_string = "sys_bomb_a";
            if random_number == 0{
                let mut eff_string = "sys_bomb_a";
                macros::PLAY_SE(agent, Hash40::new("se_common_fire_s"));
            } else if random_number == 1 {
                let mut eff_string = "sys_bomb_b";
                macros::PLAY_SE(agent, Hash40::new("se_common_fire_l"));
            } else {
                let mut eff_string = "sys_bomb_c";
                macros::PLAY_SE(agent, Hash40::new("se_common_fire_m"));
            }
            macros::EFFECT(agent, Hash40::new("master_bow_arrow2_hit"), Hash40::new("top"),0, 5, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
            //macros::EFFECT_FLW_POS(agent, Hash40::new("master_bow_arrow2_hit"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 180, 0, 1, true);
            EffectModule::set_rate_last(agent.module_accessor, 1.0);
            EffectModule::set_scale_last(agent.module_accessor, &Vector3f{x:0.8,y:0.5,z:0.8} )
            //macros::EFFECT(agent, Hash40::new(eff_string), Hash40::new("top"),0, 0, 0, 0, 0, 0, 1.0 + (smash::app::sv_math::rand(hash40("fighter"), 3) as f32 * 0.5), 0, 0, 0, 0, 0, 0, false);
        }
    }
}

pub unsafe extern "C" fn game_shotmax(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d)); 
    }

}

pub unsafe extern "C" fn effect_shotmax(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bomb_d"), Hash40::new("top"),0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, false);
    }
}

pub unsafe extern "C" fn sound_shotmax(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_bomb"));
    }
}











pub unsafe extern "C" fn effect_specialnstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
    }
}

pub unsafe extern "C" fn sound_specialnstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_fdb_X".to_string(),3,false,false,0);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
    }
}

pub unsafe extern "C" fn expression_specialnstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }

}



pub unsafe extern "C" fn game_specialnhold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, 0, 2);
    }
}

pub unsafe extern "C" fn effect_specialnhold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
    }
    wait(agent.lua_state_agent, 5.0);
}

pub unsafe extern "C" fn sound_specialnhold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_kamui_special_n01"));
    }
}

pub unsafe extern "C" fn expression_specialnhold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        //ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}




pub unsafe extern "C" fn game_specialnfire(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED){
            shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, 0, 2);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_SHIELD, 0, 2);
    }
}

pub unsafe extern "C" fn effect_specialnfire(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
    }
    wait(agent.lua_state_agent, 2.0);
}

pub unsafe extern "C" fn sound_specialnfire(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_special_n01"));
    }
}

pub unsafe extern "C" fn expression_specialnfire(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        //ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        //ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 24, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        //WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_N_FLAG_MOTION_END2);
    }
}