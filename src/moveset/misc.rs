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
use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CAgent},
    },
    smash_script::*,
    smashline::{*, Priority::*}
};
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use crate::FIGHTER_MANAGER;

use crate::moveset::*;





#[allow(non_snake_case)]
pub mod MiscModule {
    use super::*;
    
    pub unsafe fn add_shield_group(module_accessor: *mut BattleObjectModuleAccessor, resource: *mut ShieldGroupResource, group_id: i32) {
        // Get Vtable Function
        let ptr = get_module_vtable_func(module_accessor, 0x100, 0x58);
        let set_shield_group: extern "C" fn(*mut u64, *mut ShieldGroupResource, i32) = std::mem::transmute(ptr);

        // Redefine ShieldModule
        let shield_module = *(module_accessor as *mut *mut u64).add(0x100 / 8);

        set_shield_group(shield_module, resource, group_id);
        let count = (*resource).count as i32;
        if count > 0 {
            for x in 0..count {
                ShieldModule::set_status(module_accessor, x, ShieldStatus(*SHIELD_STATUS_NONE), group_id);
            }
        }
    }

    pub unsafe fn get_module_vtable_func(module_accessor: *mut BattleObjectModuleAccessor, module_offset: usize, func_offset: u64) -> u64 {
        let module = (module_accessor as *mut u64).add(module_offset / 0x8);
        let vtable = *module as *const u64;
        *((*vtable + func_offset) as *const u64)
    }
}




pub fn get_fighter_common_from_accessor<'a>(boma: &'a mut BattleObjectModuleAccessor) -> &'a mut L2CFighterCommon {
    unsafe {
        let lua_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CFighterCommon))
    }
}