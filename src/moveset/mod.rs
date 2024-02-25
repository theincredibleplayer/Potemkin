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
use skyline::hooks::{Region,getRegionAddress};
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use crate::FIGHTER_MANAGER;


pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_CHECK_ENEMY: i32 = 0x200000ec;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_GRABBED_ENEMY: i32 = 0x200000ed;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RELEASE_ENEMY: i32 = 0x200000ee;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_CHANGE_TO_FS_DASH: i32 = 0x200000ee;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_CHECK_ENEMY: i32 = 0x200000ef;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_GOTTEM: i32 = 0x200000f0;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SELECT_FINAL: i32 = 0x200000f1;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_RELEASE: i32 = 0x200000f2;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SELECT_ATTACK: i32 = 0x200000f1;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_START_NO_DEAD: i32 = 0x200000f3;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SWITCH_TO_FS_JUMP: i32 = 0x200000f4;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_JUMP: i32 = 0x200000f5;





pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_ACCEL: i32 = 0x60;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_TIMER: i32 = 0x61;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_GRABBED_ENEMY_ID: i32 = 0x100000c1;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_HPB_GOTTEM_ID: i32 = 0x100000c2;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LOOP_HITS_SPECIAL_HI: i32 = 0x100000c3;

pub const FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_START: i32 = 0x1f4;
pub const FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_LOOP: i32 = 0x1f5;
pub const FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_RELEASE: i32 = 0x1f6;
pub const FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_START: i32 = 0x1f7;
pub const FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_LOOP: i32 = 0x1f8;
pub const FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_ATTACK: i32 = 0x1f9;
pub const FIGHTER_KAMUI_STATUS_KIND_FINAL_JUMP: i32 = 0x1fa;
pub const FIGHTER_KAMUI_STATUS_KIND_FINAL_FAKE_VISSCENE: i32 = 0x1fb;






use std::{
    marker::PhantomData,
    sync::atomic::{AtomicUsize, Ordering},
};

pub trait FromArrayAddress {
    const STRIDE: usize;
    unsafe fn from_address(addr: usize) -> Self;
}

pub struct StaticArrayAccessor<T: FromArrayAddress> {
    text: AtomicUsize,
    offset: usize,
    count: usize,
    _phantom: PhantomData<T>,
}

impl<T: FromArrayAddress> StaticArrayAccessor<T> {
    pub const fn new(offset: usize, count: usize) -> Self {
        Self {
            text: AtomicUsize::new(0),
            offset,
            count,
            _phantom: PhantomData,
        }
    }

    pub fn get(&self, index: usize) -> Option<T> {
        let mut text = self.text.load(Ordering::Acquire);
        if text == 0 {
            let base =
                unsafe { skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as usize };
            self.text.store(base, Ordering::Release);
            text = base;
        }

        (index < self.count)
            .then(|| unsafe { T::from_address(text + self.offset + T::STRIDE * index) })
    }

    pub fn iter(&self) -> StaticArrayIterator<T> {
        StaticArrayIterator {
            accessor: self,
            current: 0
        }
    }
}

pub struct StaticArrayIterator<'a, T: FromArrayAddress> {
    accessor: &'a StaticArrayAccessor<T>,
    current: usize,
}

impl<'a, T: FromArrayAddress> Iterator for StaticArrayIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.accessor.get(self.current)?;
        self.current += 1;
        Some(item)
    }
}

impl FromArrayAddress for &'static str {
    const STRIDE: usize = 8;

    unsafe fn from_address(addr: usize) -> Self {
        let pointer = *(addr as *const *const u8);
        let mut len = 0;
        while *pointer.add(len) != 0 {
            len += 1;
        }

        std::str::from_utf8_unchecked(std::slice::from_raw_parts(pointer, len))
    }
}

macro_rules! primitives {
    ($($T:ty)*) => {
        $(
            impl FromArrayAddress for $T {
                const STRIDE: usize = std::mem::align_of::<$T>();

                unsafe fn from_address(addr: usize) -> Self {
                    *(addr as *const $T)
                }
            }
        )*
    }
}

primitives![i8 u8 i16 u16 i32 u32 i64 u64 f32 f64];


















pub const LOWERCASE_FIGHTER_NAMES: StaticArrayAccessor<&'static str> =
    StaticArrayAccessor::new(0x4f7fe20, 118);


pub const SITUATION_KIND: i32 = 0x16;
pub const PREV_SITUATION_KIND: i32 = 0x17;
pub const STICK_X: i32 = 0x1A;
pub const STICK_Y: i32 = 0x1B;

mod hooks;
use hooks::*;
mod throw;
use throw::*;
mod taunt;
use taunt::*;
mod speciallw;
use speciallw::*;
mod specialhi;
use specialhi::*;
mod normals;
use normals::*;
mod specials;
use specials::*;
mod specialn;
use specialn::*;
mod finalsmash; 
use finalsmash::*;

extern "C" {
    #[link_name = "\u{1}_ZN3app19FighterCutInManager13is_one_on_oneEv"]
    pub fn FighterCutInManager__is_one_on_one() -> bool;
}

//unsafe fn auto_turn(fighter: &mut L2CFighterCommon) -> L2CValue {
//    if FighterCutInManager__is_one_on_one()
//    && fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
//        let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
//        let entry_id = if utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_KAMUI {
//            1
//        }
//        else {
//            0
//        };
//        let entry = smash::app::lua_bind::FighterManager::get_fighter_entry(fighter_manager,FighterEntryID(entry_id));
//        let battle_object = *(entry as *mut u64).add(0x4160/8) as *mut u64;
//        let enemy = *battle_object.add(0x20/8) as *mut BattleObjectModuleAccessor;
//        if PostureModule::pos_x(enemy) > PostureModule::pos_x(fighter.module_accessor) {
//            PostureModule::set_lr(fighter.module_accessor,1.0);
//        }
//        else {
//            PostureModule::set_lr(fighter.module_accessor,-1.0);
//        }
//        PostureModule::update_rot_y_lr(fighter.module_accessor);
//    }
//    return L2CValue::I32(0)
//}

pub unsafe extern "C" fn kamui(fighter: &mut L2CFighterCommon) {
    unsafe {
        //WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
        //JostleModule::set_status(fighter.module_accessor, false);
        if ControlModule::check_button_trigger(fighter.module_accessor,*CONTROL_PAD_BUTTON_APPEAL_S_L) {
            println!("{}",WorkModule::get_float(fighter.module_accessor,*FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1));
        }
        if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_FINAL
        && StatusModule::status_kind(fighter.module_accessor) != FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_START
        && StatusModule::status_kind(fighter.module_accessor) != FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_LOOP
        && StatusModule::status_kind(fighter.module_accessor) != FIGHTER_KAMUI_STATUS_KIND_FINAL_FAKE_VISSCENE
        && StatusModule::status_kind(fighter.module_accessor) != FIGHTER_KAMUI_STATUS_KIND_FINAL_JUMP  
        && StatusModule::status_kind(fighter.module_accessor) != FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_ATTACK {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SELECT_FINAL);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SWITCH_TO_FS_JUMP);
        }
        /*if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK_WAIT
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ENTRY
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_KAMUI_STATUS_KIND_FINAL_READY
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_KAMUI_STATUS_KIND_FINAL_END {
            let module_accessor = fighter.module_accessor;
            let enemy_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_HPB_GOTTEM_ID);
            let enemy = sv_battle_object::module_accessor(enemy_id as u32);
            WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
            WorkModule::on_flag(enemy,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
            WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD_UP);
            WorkModule::on_flag(enemy,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD_UP);
        }*/
    }
}

//#[smashline::fighter_frame(agent = FIGHTER_KIND_RYU, main)]
//pub fn ryu(fighter: &mut L2CFighterCommon) {
//    unsafe {
//        if ControlModule::check_button_trigger(fighter.module_accessor,*CONTROL_PAD_BUTTON_APPEAL_S_L) {
//            println!("{}",WorkModule::get_float(fighter.module_accessor,*FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1));
//        }
//    }
//}

pub fn install() {
    unsafe {
        LookupSymbol(
            &mut FIGHTER_MANAGER,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
        );
        LookupSymbol(
            &mut FIGHTER_CUTIN_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E\u{0}"
                .as_bytes()
                .as_ptr(),
        );
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, OFFSET_SEARCH_CODE) {
            NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
        }
    }

    Agent::new("kamui")
    //agent frame
    .on_line(Main, kamui)

    //throw
    .game_acmd("game_throwlw", throw_lw)
    .effect_acmd("effect_throwlw", effect_throw_lw)
    .sound_acmd("sound_throwlw", sound_throw_lw)
    .expression_acmd("expression_throwlw", expression_throwlw)

    //taunt
    .game_acmd("game_appealsr", appeal_srl)
    .effect_acmd("effect_appealsr", effect_appeal_srl)
    .sound_acmd("sound_appealsr", sound_appeal_srl)
    .game_acmd("game_appealsl", appeal_srl)
    .effect_acmd("effect_appealsl", effect_appeal_srl)
    .sound_acmd("sound_appealsl", sound_appeal_srl)
    .status(End, *FIGHTER_STATUS_KIND_APPEAL, appeal_end)

    //special_s
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main)
    .status(Pre, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL, side_b_slide_pre)
    .status(Main, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL, side_b_slide)
    .status(Pre, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_END, side_b_end_pre)
    .status(Main, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_END, side_b_end)
    .status(Pre, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK, side_b_attack_pre)
    .status(Main, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK, side_b_attack)
    .status(Init, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL, side_b_slide_init)
    .status(Exit, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL, side_b_slide_exit)
    .status(Exec, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL, side_b_slide_exec)
    .game_acmd("game_specialsslide", game_specialsloop)
    .effect_acmd("effect_specialsslide", effect_specialsloop)
    .effect_acmd("effect_specials", effect_specials)
    .game_acmd("game_specialsattack", game_specialsattack)
    //.status(Pre, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B, megafist_f_pre)
    .status(Main, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B, megafist_f_main)
    .status(Main, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B_LANDING, megafist_f_land_main)
    //.status(Pre, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F, megafist_f_pre)
    .status(Main, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F, megafist_f_main)
    .game_acmd("game_specialsmff", game_specialsmff)
    .status(Main, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F_LANDING, megafist_f_land_main)


    //special_lw
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_end)
    .game_acmd("game_speciallw", special_lw)
    .effect_acmd("effect_speciallw", effect_special_lw)
    .sound_acmd("sound_speciallw", sound_special_lw)

    //special_hi
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main)
    .status(Main, FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_START, special_hi_start_main)
    .status(Main, FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_LOOP, special_hi_loop_main)
    .status(Main, FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_RELEASE, special_hi_release_main)
    .status(End, FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_RELEASE, special_hi_release_end)
    .game_acmd("game_specialhi", special_hi)
    .effect_acmd("effect_specialhi", effect_special_hi)
    .sound_acmd("sound_specialhi", sound_special_hi)
    .game_acmd("game_specialhistart", special_hi_start)
    .game_acmd("game_specialhiloop", special_hi_loop)
    .game_acmd("game_specialhirelease", special_hi_release)

    //normals
    .game_acmd("game_attacks3", attack_s3)
    .effect_acmd("effect_attacks3", effect_attack_s3)
    .game_acmd("game_attacklw3", attack_lw3)
    .effect_acmd("effect_attacklw3", effect_attack_lw3)
    .game_acmd("game_attackdash", attack_dash)
    .effect_acmd("effect_attackdash", effect_attack_dash)
    .game_acmd("game_catch", catch)
    .game_acmd("game_catchdash", catchdash)
    .game_acmd("game_catchturn", catchturn)
    .game_acmd("game_attackhi4", attack_hi4)
    .sound_acmd("sound_attackhi4", sound_attackhi4)
    .expression_acmd("expression_attackhi4", expression_attackhi4)
    .effect_acmd("effect_attackhi4", effect_attack_hi4)
    .effect_acmd("effect_attackshi4charge", effect_attack_hi4_charge)
    .game_acmd("game_attacks4", attack_s4)
    .sound_acmd("sound_attacks4", sound_attacks4)
    .sound_acmd("sound_attacks4charge", sound_attacks4charge)
    .expression_acmd("expression_attacks4", expression_attacks4)
    .effect_acmd("effect_attacks4", effect_attack_s4)
    .effect_acmd("effect_attacks4charge", effect_attack_s4_charge)
    .game_acmd("game_final", game_final)
    .sound_acmd("sound_final", sound_final)
    .effect_acmd("effect_final", effect_final)
    .game_acmd("game_finalvisualscene01", game_finalvisualscene01)
    .game_acmd("game_finalvisualscene02", game_finalvisualscene02)
    .game_acmd("game_finalvisualscene03", game_finalvisualscene03)
    .sound_acmd("sound_finalvisualscene01", sound_finalvisualscene01)
    .sound_acmd("sound_finalvisualscene02", sound_finalvisualscene02)
    .sound_acmd("sound_finalvisualscene03", sound_finalvisualscene03)
    .game_acmd("game_fakefinalvisualscene01", game_fakefinalvisualscene01)
    .sound_acmd("sound_fakefinalvisualscene01", sound_fakefinalvisualscene01)
    .effect_acmd("effect_finalvisualscene02", effect_finalvisualscene02)
    .game_acmd("game_attacklw4", game_attacklw4)
    .game_acmd("game_win1a", game_win1a)

    
    //special_n



    //final
    .status(Main, *FIGHTER_STATUS_KIND_FINAL, Final_main)
    //.status(Exit, *FIGHTER_STATUS_KIND_FINAL, Final_exit)
    .status(Pre, FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_LOOP, finaldashloop_pre)
    .status(Main, FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_LOOP, finaldashloop_main)
    .status(Exit, FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_LOOP, finaldashloop_exit)
    .status(Init, FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_LOOP, finaldashloop_init)
    .status(Exec, FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_LOOP, finaldashloop_exec)

    .status(Pre, FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_ATTACK, finaldashattack_pre)
    .status(Main, FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_ATTACK, finaldashattack_main)
    .status(Exit, FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_ATTACK, finaldashattack_exit)
    .status(Exec, FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_ATTACK, finaldashattack_exec)
    .status(Init, FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_ATTACK, finaldashattack_init)
    .status(End, FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_ATTACK, finaldashattack_end)

    .status(Main, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ENTRY, finalsceneenter_main)
    .status(Main, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK, finalsceneattack_main)
    .status(Main, *FIGHTER_KAMUI_STATUS_KIND_FINAL_END, finalsceneexit_main)

    .status(Pre, FIGHTER_KAMUI_STATUS_KIND_FINAL_JUMP, finaldashattack_pre)
    .status(Main, FIGHTER_KAMUI_STATUS_KIND_FINAL_JUMP, finaljump_main)
    .status(Exit, FIGHTER_KAMUI_STATUS_KIND_FINAL_JUMP, finaldashattack_exit)
    .status(Init, FIGHTER_KAMUI_STATUS_KIND_FINAL_JUMP, finaldashattack_init)
    .status(Exec, FIGHTER_KAMUI_STATUS_KIND_FINAL_JUMP, finaldashattack_exec)
    .status(Exec, FIGHTER_KAMUI_STATUS_KIND_FINAL_JUMP, finaldashattack_end)


    .status(Pre, FIGHTER_KAMUI_STATUS_KIND_FINAL_FAKE_VISSCENE, fakefinalsceneenter_pre)
    .status(Main, FIGHTER_KAMUI_STATUS_KIND_FINAL_FAKE_VISSCENE, fakefinalsceneenter_main)
    .status(Exit, FIGHTER_KAMUI_STATUS_KIND_FINAL_FAKE_VISSCENE, fakefinalsceneenter_exit)
    .status(Init, FIGHTER_KAMUI_STATUS_KIND_FINAL_FAKE_VISSCENE, fakefinalsceneenter_init)
    .status(Exec, FIGHTER_KAMUI_STATUS_KIND_FINAL_FAKE_VISSCENE, fakefinalsceneenter_exec)
    .status(End, FIGHTER_KAMUI_STATUS_KIND_FINAL_FAKE_VISSCENE, fakefinalsceneenter_end)
    
    .status(Exec, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ENTRY, vissceneentry_exec)
    .status(Exec, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK_WAIT, vissceneattackwait_exec)





    .game_acmd("game_heavenlydashloop", game_finaldash)
    .game_acmd("game_heavenlydashattack", game_finaldashattack)
    .game_acmd("game_finalend", game_finalend)
    .game_acmd("game_finalairlend", game_finalend)
    .sound_acmd("sound_finalend", sound_finalend)
    .sound_acmd("sound_finalairend", sound_finalend)
    .effect_acmd("effect_finalend", effect_finalend)
    .effect_acmd("effect_finalairend", effect_finalend)
    .game_acmd("game_heavenlystart", game_heavenlystart)


    .install();

    Agent::new("kamui_waterdragon")
    //special_lw
    .game_acmd("game_speciallwhit", special_lw_hit)
    .effect_acmd("effect_speciallwhit", effect_special_lw_hit)
    .sound_acmd("sound_speciallwhit", sound_special_lw_hit)

    //special_hi
    .game_acmd("game_specialhi", special_dragon_hi)
    .game_acmd("game_entryr", special_dragon_hi_loop)
    .game_acmd("game_win3", special_dragon_hi_release)
    .game_acmd("game_entryl", entry_lr_throw_hi)
    .game_acmd("game_throwhi", entry_lr_throw_hi)
    .effect_acmd("effect_entryl", effect_entry_lr_throw_hi)
    .effect_acmd("effect_entryr", effect_entry_lr_throw_hi)
    .effect_acmd("effect_throwhi", effect_entry_lr_throw_hi)


    .install();

    Agent::new("kamui_spearhand")
    .game_acmd("game_attacks4", spear_attack_s4)
    .effect_acmd("effect_attacks4charge", effect_attack_s4_charge_spear)
    .install();

    skyline::install_hooks!(notify_log_event_collision_hit_replace);
    skyline::patching::Patch::in_text(0xb5cca0).nop().expect("Failed to patch 0xb5cca0");
}