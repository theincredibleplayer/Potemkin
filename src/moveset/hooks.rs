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

pub static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x67a7b0;
// old implementation of grabs should be removed once full transition to real grabs has been done
#[skyline::hook(offset = NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe extern "C" fn notify_log_event_collision_hit_replace(fighter_manager: *mut smash::app::FighterManager, attacker_id: u32, defender_id: u32, move_type: f32, arg5: i32, move_type_again: bool) -> u64 {
    let attacker_boma = sv_battle_object::module_accessor(attacker_id);
    let attacker_kind = sv_battle_object::kind(attacker_id);
    if attacker_kind == *FIGHTER_KIND_KAMUI || attacker_kind == *WEAPON_KIND_KAMUI_WATERDRAGON {
        let owner_id = WorkModule::get_int(attacker_boma,*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let owner = sv_battle_object::module_accessor(owner_id);
        /*if WorkModule::is_flag(owner,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_CHECK_ENEMY) {
            println!("check enemy enabled");
            WorkModule::on_flag(owner,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_GRABBED_ENEMY);
            WorkModule::off_flag(owner,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_CHECK_ENEMY);
            WorkModule::set_int(owner,defender_id as i32,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_GRABBED_ENEMY_ID);
            println!("got the mf's id: {}",defender_id);
        }*/
        //if WorkModule::is_flag(owner,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_CHECK_ENEMY) {
        //    println!("check enemy enabled");
        //    WorkModule::on_flag(owner,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_GOTTEM);
        //    WorkModule::off_flag(owner,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_CHECK_ENEMY);
        //    WorkModule::set_int(owner,defender_id as i32,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_HPB_GOTTEM_ID);
        //    println!("got the mf's id: {}",defender_id);
        //}
    }
    original!()(fighter_manager, attacker_id, defender_id, move_type, arg5, move_type_again)
}


#[skyline::hook(offset = 0xb5f380)]
pub unsafe extern "C" fn kamui_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash_rs::app::LinkEvent) -> u64 {
    if event.link_event_kind.0 == hash40("capture") {
        let capture_event : &mut smash_rs::app::LinkEventCapture = std::mem::transmute(event);
        let module_accessor = fighter.battle_object.module_accessor;
        //upair linkmodule
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            capture_event.result = true;
            capture_event.node = smash_rs::phx::Hash40::new("throw");
            StatusModule::change_status_request(module_accessor, FIGHTER_KAMUI_STATUS_KIND_AIR_THROW, false);
            return 0;
        }
        //potemkin buster linkmodule
        if StatusModule::status_kind(module_accessor) == FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER_WHIFF {
            capture_event.result = true;
            capture_event.node = smash_rs::phx::Hash40::new("throw");
            capture_event.ignore_distance = true;
            StatusModule::change_status_request(module_accessor, FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER, false);
            return 0;
        }
        //special_hi's link module
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            capture_event.result = true;
            capture_event.node = smash_rs::phx::Hash40::new("throw");
            //capture_event.is_motion_hi_lw = false;
            StatusModule::change_status_request(module_accessor, FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_START, false);
            return 0;
        }
        //final smash's link module
        if StatusModule::status_kind(module_accessor) == FIGHTER_KAMUI_STATUS_KIND_FINAL_JUMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_FINAL {
            capture_event.result = true;
            //capture_event.constraint_offset.y = 20.0;
            capture_event.node = smash_rs::phx::Hash40::new("throw");
            

            WorkModule::off_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_JUMP);

            WorkModule::off_flag(module_accessor ,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_START);
            WorkModule::off_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_START);
            WorkModule::off_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_ATTACK_START);
            WorkModule::off_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_JUMP_START);

            PostureModule::set_pos(module_accessor,&Vector3f{x: 0.0 , y: 0.0, z: 50.0});
            StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_FINAL, false);      
            return 0;
        }
        return 1;
    }
    original!()(vtable, fighter, event)
}

#[skyline::hook(offset = 0x3414930)]
pub unsafe extern "C" fn noping_the_eff(vtable:u64, weapon: *mut smash::app::Weapon, log: CollisionLog) -> u64 {
    *(weapon as *mut bool).add(0x90) = true; //Lets the weapon persist on hit
    let mut flag = true;
    let opponent_id = log.opponent_battle_object_id;
    let mut opponent_accessor = sv_battle_object::module_accessor(opponent_id);





    if (AttackModule::is_infliction((*weapon).battle_object.module_accessor, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction((*weapon).battle_object.module_accessor, *COLLISION_KIND_MASK_SHIELD) || AttackModule::is_infliction((*weapon).battle_object.module_accessor, *COLLISION_KIND_MASK_ATTACK) )
    {
        if WorkModule::is_flag((*weapon).battle_object.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_DID_IT_HIT){
            WorkModule::off_flag((*weapon).battle_object.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_DID_IT_HIT);
            StatusModule::change_status_request_from_script((*weapon).battle_object.module_accessor, WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM, false);
        }
    }
    call_original!(vtable, weapon, log)
}




#[skyline::hook(offset = 0x4E53E0)]
unsafe fn get_param_float_hook(x0:u64, arg2:u64, arg3:u64) -> f32 {
    let mut boma = *((x0 as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
    let boma_reference = &mut *boma;
    let fighter_kind = utility::get_kind(boma_reference);
    if fighter_kind == *FIGHTER_KIND_KAMUI{
        if (arg2 == 0xdf05c072b) && (arg3 == 0x19996b81b6){
            return 1200.0;
        }
    }
    original!()(x0, arg2, arg3)
}


#[skyline::hook(offset = 0x6df820, inline)]
unsafe fn change_motion_hook(ctx: &skyline::hooks::InlineCtx) {
    println!("motion register: {:#x}",*ctx.registers[1].x.as_ref() as u64);

    if *ctx.registers[1].x.as_ref() as u64 == hash40("win_1") 
    || *ctx.registers[1].x.as_ref() as u64 == hash40("win_2") 
    || *ctx.registers[1].x.as_ref() as u64 == hash40("win_3") {
        println!("return register: {:#x}", *ctx.registers[0].x.as_ref() as u64 - getRegionAddress(Region::Text) as u64);
        let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        let mut lr: *const u64;
        asm!("mov {}, x30", out(reg) lr);
    println!("Current LR:\t\t{:#x} ({:#x})", (lr as u64) - text, (lr as u64));
    }
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


