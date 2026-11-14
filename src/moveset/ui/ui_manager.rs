//use crate::moveset::*;
use nnsdk::ui2d::*;
use smash::ui2d::SmashPane;
use parking_lot::RwLock;
use smash::app::lua_bind::*;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::moveset::ui::dual_meter::TensionMeter;
use crate::moveset::ui::dual_meter::CutInUI;
use crate::moveset::consts::UI_DRAW_HOOK_OFFSET;

use once_cell::sync::Lazy;
pub static UI_MANAGER: Lazy<RwLock<UiManager>> = Lazy::new(|| RwLock::new(UiManager { 
    tension_meter: [TensionMeter::default(); 8],
    cutin_ui: [CutInUI::default(); 8],
    is_enabled: [false; 8]
}));

#[repr(C)]
pub struct UiManager {
    pub tension_meter: [TensionMeter; 8],
    pub cutin_ui: [CutInUI; 8],
    pub is_enabled: [bool; 8]
}

impl UiManager {
    pub fn enable_ui(ui_id: usize) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.is_enabled[ui_id] = true;
        }
    }

    pub fn disable_ui(ui_id: usize) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.is_enabled[ui_id] = false;
        }
    }

    pub fn is_enabled(ui_id: usize) -> bool {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.is_enabled[ui_id]
        }
    }

    pub fn reset_on_rebirth(ui_id: usize) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.tension_meter[ui_id].reset();
            manager.cutin_ui[0].reset_all();
        }
    }

    pub fn add_tension(ui_id: usize, amount: f32) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.tension_meter[ui_id].add_meter(amount,true);
        }
    }

    pub fn add_tension_unscaled(ui_id: usize, amount: f32) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.tension_meter[ui_id].add_meter(amount,false);
        }
    }

    pub fn set_penalty(ui_id: usize, amount: f32) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.tension_meter[ui_id].set_penalty(amount);
        }
    }

    pub fn is_tension_available(ui_id: usize) -> bool {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.tension_meter[ui_id].is_available()
        }
    }

    pub fn is_tension_full(ui_id: usize) -> bool {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.tension_meter[ui_id].is_tension2_available()
        }
    }

    pub fn consume_tension(ui_id: usize) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.tension_meter[ui_id].consume_tension();
        }
    }
    pub fn get_frame_from_name(ui_id: usize, name: &str) -> f32 {
        let mut manager = UI_MANAGER.write();
        unsafe {
            if name == "tension_meter" {
                return manager.tension_meter[ui_id].get_meter_frame();
            }
            else if name == "gear1"{
                return manager.tension_meter[ui_id].get_gear_frame_1();
            }
            else if name == "fire1"{
                return manager.tension_meter[ui_id].get_flame_frame_1();
            }
            else if name == "gear2"{
                return manager.tension_meter[ui_id].get_gear_frame_2();  
            }
            else if name == "fire2"{
                return manager.tension_meter[ui_id].get_flame_frame_2();  
            } else if name == "bar_hi"
            || name == "bar_low"{
                return manager.cutin_ui[0].get_bar_frame();
            } else if name == "jetpack" {
                return manager.cutin_ui[0].get_jetpack_frame();
            } else if name == "explosion_1" {
                return manager.cutin_ui[0].get_explosion_1_frame();
            } else if name == "explosion_2" {
                return manager.cutin_ui[0].get_explosion_2_frame();
            } else {
                return 0.0;
            }
        }
    }

    pub fn get_border_availability_from_name(ui_id: usize, name: &str) -> bool {
        let mut manager = UI_MANAGER.write();
        unsafe {
            if name == "gear1" 
            || name == "fire1"{
                return manager.tension_meter[ui_id].is_tension1_available();
            }
            else if name == "fire2" 
            || name == "gear2"{
                return manager.tension_meter[ui_id].is_tension2_available();
            }else if name == "bar_hi"
            || name == "bar_low"{
                return manager.cutin_ui[0].is_bar_on();
            }else if name == "jetpack"{
                return manager.cutin_ui[0].is_jetpack_on();
            }else if name == "explosion_1"{
                return manager.cutin_ui[0].is_explosion_1_on();
            }else if name == "explosion_2"{
                return manager.cutin_ui[0].is_explosion_2_on();
            }
            else {
                return false;
            }
        }
    }
    pub fn reset_cutin() {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.cutin_ui[0].reset_all();
        }
    }
    pub fn set_game_speed(speed: f32) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.cutin_ui[0].set_game_speed(speed);
        }
    }
    pub fn set_jetpack_lr(lr: f32) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.cutin_ui[0].set_jetpack_lr(lr);
        }
    }
    pub fn get_jetpack_lr() ->f32 {
        let mut manager = UI_MANAGER.write();
        unsafe {
            return manager.cutin_ui[0].get_jetpack_lr();
        }
    }
    pub fn sync_jetpack(frame:f32) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.cutin_ui[0].set_jetpack_frame_sync(frame);
        }
    }

    pub fn set_bar_off(){
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.cutin_ui[0].set_bar_off();
        }
    }
    pub fn set_bar_on() {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.cutin_ui[0].set_bar_on();
        }
    }
    pub fn is_bar_on() {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.cutin_ui[0].is_bar_on();
        }
    }
    pub fn get_bar_frame() {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.cutin_ui[0].get_bar_frame();
        }
    }
    pub fn set_jetpack_on(){
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.cutin_ui[0].set_jetpack_on();
        }
    }
    pub fn is_jetpack_on() {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.cutin_ui[0].is_jetpack_on();
        }
    }
    pub fn set_explosion_1_on(){
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.cutin_ui[0].set_explosion_1_on();
        }
    }
    pub fn is_explosion_1_on() {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.cutin_ui[0].is_explosion_1_on();
        }
    }

}

/*#[repr(C)]
pub struct ExtUserDataNode {
    pub prev: *mut ExtUserDataNode,
    pub next: *mut ExtUserDataNode,
    pub curr: *mut ExtUserData
}

#[repr(C)]
pub struct ExtUserData {
    pub eff_name: String
}

pub fn get_usd(usd: [skyline::libc::c_char; 9]) -> String {
    usd
        .iter()
        .take_while(|b| **b != 0)
        .map(|b| *b as char)
        .collect::<String>()
}*/

#[skyline::hook(offset = UI_DRAW_HOOK_OFFSET, inline)]
pub unsafe fn handle_draw_potemkin(ctx: &mut skyline::hooks::InlineCtx) {
    let layout = ctx.registers[0].x() as *mut Layout;
    ctx.registers[0].set_x(handle_layout_potemkin(layout) as u64);    
}

unsafe fn handle_layout_potemkin(layout: *mut Layout) -> *mut Layout {
    let layout_name = skyline::from_c_str((*layout).layout_name);
    let root_pane = &mut *(*layout).root_pane;
    if layout_name == "info_melee" {
        if let Some(child) = root_pane.find_pane_by_name_recursive("pot") {
            (*child).alpha = 255;
            let anim_list = &mut (*child.as_parts().layout).anim_trans_list;
            let mut curr = anim_list as *mut AnimTransformNode;
            while !curr.is_null() {
                if curr != (*curr).next {
                    let anim_transform = (curr as *mut u64).add(2) as *mut AnimTransform;
                    let res_animation_block_data_start = (*anim_transform).res_animation_block as u64;
                    let res_animation_block = &(*anim_transform).res_animation_block;
                    let mut anim_cont_offsets = (res_animation_block_data_start + (*(*res_animation_block)).anim_cont_offsets_offset as u64) as *const u32;
                    for _ in 0..(*(*res_animation_block)).anim_cont_count {
                        let anim_cont_offset = *anim_cont_offsets;
                        let res_animation_cont = (res_animation_block_data_start + anim_cont_offset as u64) as *const ResAnimationContent;
                        let name = skyline::try_from_c_str((*res_animation_cont).name.as_ptr()).unwrap_or_else(|_| "UNKNOWN".to_string());
                        //println!("name: {}",name);
                        let name_list = ["bar_hi","bar_low","explosion_1","explosion_2","jetpack"];
                        for _name in name_list {
                            if name == _name {
                                (*anim_transform).enabled = true;
                                (*anim_transform).frame = UiManager::get_frame_from_name(0,_name);
                                if name == "jetpack"{
                                    if let Some(border) = child.find_pane_by_name_recursive(&name) {
                                        border.scale_x = UiManager::get_jetpack_lr();
                                        border.flags |= 1 << PaneFlag::IsGlobalMatrixDirty as u8;
                                    }
                                }
                                //rintln!("frame: {}",(*anim_transform).frame);
                                break;
                            }
                        }
                    }
                }
                curr = (*curr).next;
                if curr == anim_list as *mut AnimTransformNode || curr == (*curr).next {
                    break;
                }
            }
            let border_list = ["bar_hi","bar_low","explosion_1","explosion_2","jetpack"];
            for pane in border_list {
                if let Some(border) = child.find_pane_by_name_recursive(pane) {
                    border.set_visible(UiManager::get_border_availability_from_name(0,pane));
                }
            }
        }
        let player_smash_8 = FighterManager::entry_count(singletons::FighterManager()) > 4;
        let mut pane_list;
        if !player_smash_8{
            pane_list = ["p1","p2","p3","p4","p5","p6","p7","p8"];
        }else{
            pane_list = ["p1_2","p2_2","p3_2","p4_2","p5","p6","p7","p8"];
        }
        let mut idx = 0;
        for pane in pane_list {
            if let Some(parent) = root_pane.find_pane_by_name_recursive(pane) {
                if idx < 8
                && UiManager::is_enabled(idx) {
                    if let Some(child) = parent.find_pane_by_name_recursive("tension") {
                        (*child).alpha = 255;
                        let anim_list = &mut (*child.as_parts().layout).anim_trans_list;
                        let mut curr = anim_list as *mut AnimTransformNode;
                        while !curr.is_null() {
                            if curr != (*curr).next {
                                let anim_transform = (curr as *mut u64).add(2) as *mut AnimTransform;
                                let res_animation_block_data_start = (*anim_transform).res_animation_block as u64;
                                let res_animation_block = &(*anim_transform).res_animation_block;
                                let mut anim_cont_offsets = (res_animation_block_data_start + (*(*res_animation_block)).anim_cont_offsets_offset as u64) as *const u32;
                                for _ in 0..(*(*res_animation_block)).anim_cont_count {
                                    let anim_cont_offset = *anim_cont_offsets;
                                    let res_animation_cont = (res_animation_block_data_start + anim_cont_offset as u64) as *const ResAnimationContent;
                                    let name = skyline::try_from_c_str((*res_animation_cont).name.as_ptr()).unwrap_or_else(|_| "UNKNOWN".to_string());
                                    let name_list = ["fire1","fire2","gear1","gear2","tension_meter"];
                                    for _name in name_list {
                                        if name == _name {
                                            (*anim_transform).enabled = true;
                                            (*anim_transform).frame = UiManager::get_frame_from_name(idx,_name);
                                            break;
                                        }
                                    }
                                }
                            }
                            curr = (*curr).next;
                            if curr == anim_list as *mut AnimTransformNode || curr == (*curr).next {
                                break;
                            }
                        }
                        let border_list = ["fire1","fire2","gear1","gear2"];
                        for pane in border_list {
                            if let Some(border) = child.find_pane_by_name_recursive(pane) {
                                border.set_visible(UiManager::get_border_availability_from_name(idx,pane));
                            }
                        }
                    }
                }
                else {
                    if let Some(child) = parent.find_pane_by_name_recursive("tension") {
                        (*child).alpha = 0;
                    }
                }
                idx += 1;
            }
        }
//==========================================================================================================================================================================
//==========================================================================================================================================================================
//==========================================================================================================================================================================
    }
    return layout;
}






pub fn install() {
    skyline::install_hooks!(
        handle_draw_potemkin,
    );
}