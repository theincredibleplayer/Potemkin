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

pub static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x675A20;

#[skyline::hook(offset = NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe extern "C" fn notify_log_event_collision_hit_replace(fighter_manager: *mut smash::app::FighterManager, attacker_id: u32, defender_id: u32, move_type: f32, arg5: i32, move_type_again: bool) -> u64 {
    let attacker_boma = sv_battle_object::module_accessor(attacker_id);
    let attacker_kind = sv_battle_object::kind(attacker_id);
    if attacker_kind == *FIGHTER_KIND_KAMUI || attacker_kind == *WEAPON_KIND_KAMUI_WATERDRAGON {
        let owner_id = WorkModule::get_int(attacker_boma,*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let owner = sv_battle_object::module_accessor(owner_id);
        if WorkModule::is_flag(owner,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_CHECK_ENEMY) {
            WorkModule::on_flag(owner,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_GRABBED_ENEMY);
            WorkModule::off_flag(owner,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_CHECK_ENEMY);
            WorkModule::set_int(owner,defender_id as i32,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_GRABBED_ENEMY_ID);
        }
        if WorkModule::is_flag(owner,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_CHECK_ENEMY) {
            WorkModule::on_flag(owner,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_GOTTEM);
            WorkModule::off_flag(owner,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_CHECK_ENEMY);
            WorkModule::set_int(owner,defender_id as i32,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_HPB_GOTTEM_ID);
        }
    }
    original!()(fighter_manager, attacker_id, defender_id, move_type, arg5, move_type_again)
}

pub fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

pub static OFFSET_SEARCH_CODE: &[u8] = &[
    0xff, 0x03, 0x03, 0xd1, //.text:0000007100675A20                 SUB             SP, SP, #0xC0
    0xe8, 0x2b, 0x00, 0xfd, //.text:0000007100675A24                 STR             D8, [SP,#0xB0+var_60]
    0xfc, 0x6f, 0x06, 0xa9, //.text:0000007100675A28                 STP             X28, X27, [SP,#0xB0+var_50]
    0xfa, 0x67, 0x07, 0xa9, //.text:0000007100675A2C                 STP             X26, X25, [SP,#0xB0+var_40]
    0xf8, 0x5f, 0x08, 0xa9, //.text:0000007100675A30                 STP             X24, X23, [SP,#0xB0+var_30]
    0xf6, 0x57, 0x09, 0xa9, //.text:0000007100675A34                 STP             X22, X21, [SP,#0xB0+var_20]
    0xf4, 0x4f, 0x0a, 0xa9, //.text:0000007100675A38                 STP             X20, X19, [SP,#0xB0+var_10]
    0xfd, 0x7b, 0x0b, 0xa9, //.text:0000007100675A3C                 STP             X29, X30, [SP,#0xB0+var_s0]
    0xfd, 0xc3, 0x02, 0x91, //.text:0000007100675A40                 ADD             X29, SP, #0xB0
    0xfb, 0x03, 0x00, 0xaa  //.text:0000007100675A44                 MOV             X27, X0
];

/*#[skyline::hook(offset = 0x4a0f90)]
pub unsafe extern "C" fn motion_hook(unk: u64, unk2: u64) -> u64 {
    let filepath = (unk + 0x148) as *mut *mut u64;
    println!("filepath: {:#x}",**filepath);
    original!()(unk,unk2)
}*/