#![feature(asm, concat_idents, proc_macro_hygiene, simd_ffi)]
use {
    crate::common::{
        cancels::*,
        consts::*,
        extern_func::*,
        globals::*,
        misc::*,
        roman_cancel::*,
        sound::*,
        vtable::*,
    },
    skyline::{
        hooks::{
            getRegionAddress,
            Region
        },
        install_hooks,
        libc::c_void,
        nn::ro::LookupSymbol,
        patching::*
    },
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
            *
        },
        hash40,
        lib::{
            L2CAgent,
            L2CValue,
            lua_const::*,
        },
        lua2cpp::*,
        phx::*,
    },
    smash_script::{
        macros::*,
        *
    },
    smashline::{
        Priority::*,
        *
    },
    std::arch::asm
};

pub mod common;
mod kamui;

#[skyline::main(name = "hotpot")]
pub fn main() {
    kamui::install();
}