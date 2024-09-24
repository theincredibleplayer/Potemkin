use crate::moveset::*;
use smash::lib::{L2CValue,lua_const::*};
use smash::lua2cpp::{L2CFighterCommon,L2CAgentBase,L2CFighterBase};
use smash::hash40;
use smash::app::lua_bind::*;
use smash_script::macros::*;
use smash_script::lua_args;
use smashline::*;
use smash::app::*;
use smash::phx::*;
use skyline::nn::ro::LookupSymbol;
use nnsdk::ui2d::*;
use smash::ui2d::SmashPane;
use skyline::libc::c_void;
use parking_lot::RwLock;
use once_cell::sync::Lazy;
pub static UI_MANAGER: Lazy<RwLock<UiManager>> = Lazy::new(|| RwLock::new(UiManager { 
    limit_meter: [LimitMeter::default(); 8],
    atb_meter: [ATBMeter::default(); 8],
    is_enabled: [false; 8]
}));

#[repr(C)]
pub struct UiManager {
    pub limit_meter: [LimitMeter; 8],
    pub atb_meter: [ATBMeter; 8],
    pub is_enabled: [bool; 8]
}

impl UiManager {
    pub fn enable_ui(entry_id: usize) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.is_enabled[entry_id] = true;
        }
    }

    pub fn disable_ui(entry_id: usize) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.is_enabled[entry_id] = false;
        }
    }

    pub fn is_enabled(entry_id: usize) -> bool {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.is_enabled[entry_id]
        }
    }

    pub fn reset_on_rebirth(entry_id: usize) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.limit_meter[entry_id].reset();
            manager.atb_meter[entry_id].reset();
        }
    }

    pub fn add_atb(entry_id: usize, amount: f32) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.atb_meter[entry_id].add_meter(amount);
        }
    }

    pub fn is_atb_available(entry_id: usize) -> bool {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.atb_meter[entry_id].is_available()
        }
    }

    pub fn is_atb_full(entry_id: usize) -> bool {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.atb_meter[entry_id].is_atb2_available()
        }
    }

    pub fn consume_atb(entry_id: usize) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.atb_meter[entry_id].consume_atb();
        }
    }

    pub fn add_limit(entry_id: usize, amount: f32) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.limit_meter[entry_id].add_meter(amount);
        }
    }

    pub fn is_limit_available(entry_id: usize) -> bool {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.limit_meter[entry_id].is_available()
        }
    }

    pub fn reset_limit(entry_id: usize) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.limit_meter[entry_id].reset();
        }
    }

    pub fn get_frame_from_name(entry_id: usize, name: &str) -> f32 {
        let mut manager = UI_MANAGER.write();
        unsafe {
            if name == "limit" {
                return manager.limit_meter[entry_id].get_meter_frame();
            }
            else if name == "atb" {
                return manager.atb_meter[entry_id].get_meter_frame();
            }
            else if name == "atb_border1"
            || name == "atb_border2" {
                return manager.atb_meter[entry_id].get_glow_frame();
            }
            else if name == "limit_border" {
                return manager.limit_meter[entry_id].get_glow_frame();
            }
            else {
                return 0.0;
            }
        }
    }

    pub fn get_border_availability_from_name(entry_id: usize, name: &str) -> bool {
        let mut manager = UI_MANAGER.write();
        unsafe {
            if name == "atb_border1" {
                return manager.atb_meter[entry_id].is_atb1_available();
            }
            else if name == "atb_border2" {
                return manager.atb_meter[entry_id].is_atb2_available();
            }
            else if name == "limit_border" {
                return manager.limit_meter[entry_id].is_available();
            }
            else {
                return false;
            }
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

#[skyline::hook(offset = 0x4b638, inline)]
pub unsafe fn handle_draw(ctx: &mut skyline::hooks::InlineCtx) {
    let layout = *ctx.registers[0].x.as_ref() as *mut Layout;
    *ctx.registers[0].x.as_mut() = handle_layout(layout) as u64;    
}

unsafe fn handle_layout(layout: *mut Layout) -> *mut Layout {
    let layout_name = skyline::from_c_str((*layout).layout_name);
    let root_pane = &mut *(*layout).root_pane;
    if layout_name == "info_melee" {
        let pane_list = ["p1","p2","p3","p4","p5","p6","p7","p8"];
        let mut idx = 0;
        for pane in pane_list {
            if let Some(parent) = root_pane.find_pane_by_name_recursive(pane) {
                /*if let Some(child) = parent.find_pane_by_name_recursive("set_fxui_heal") {
                    println!("name: {}",(*child).get_name());
                    //let usd = (*child).ext_user_data_list as *const ExtUserDataNode;
                    //println!("usd: {}",(*(*usd).curr).eff_name);
                    let usd = ((*child).ext_user_data_list as *mut u64).add(4);
                    let eff_name = std::ffi::CStr::from_ptr(*usd as *const skyline::libc::c_char as _).to_str().unwrap();
                    println!("eff_name: {}",eff_name);
                }*/
                if UiManager::is_enabled(idx) {
                    if let Some(child) = parent.find_pane_by_name_recursive("limit") {
                        (*child).alpha = 0;
                    }
                    if let Some(child) = parent.find_pane_by_name_recursive("cloud") {
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
                                    let name_list = ["limit","atb","atb_border1","atb_border2","limit_border"];
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
                        let border_list = ["atb_border1","atb_border2","limit_border"];
                        for pane in border_list {
                            if let Some(border) = child.find_pane_by_name_recursive(pane) {
                                border.set_visible(UiManager::get_border_availability_from_name(idx,pane));
                            }
                        }
                    }
                }
                else {
                    if let Some(child) = parent.find_pane_by_name_recursive("cloud") {
                        (*child).alpha = 0;
                    }
                }
                idx += 1;
            }
        }
    }
    return layout;
}