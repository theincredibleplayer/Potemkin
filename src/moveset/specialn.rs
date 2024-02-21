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


unsafe fn motion_setup(fighter: &mut L2CFighterCommon, situation_change: bool) {
    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if situation_change == false {
            MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_air_s"),0.0,1.0,false,0.0,false,false);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR,0.0,0.0,0.0,0.0,0.0);
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_air_s"),-1.0,1.0,0.0,false,false);
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if situation_change == false {
            MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_s"),0.0,1.0,false,0.0,false,false);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_GROUND,0.0,0.0,0.0,0.0,0.0);
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_s"),-1.0,1.0,0.0,false,false);
        }
    }
}