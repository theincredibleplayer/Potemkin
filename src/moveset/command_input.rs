//huge thanks to the goat WuBoy for this code
use smash::{lua2cpp::*, app::*};
use smashline::*;
use crate::moveset::command_structs::*;
use smash::lib::{L2CValue,lua_const::*};
/// Sets a command input to only use certain buttons.
pub unsafe fn set_command_input_button(module_accessor: *mut BattleObjectModuleAccessor, command: usize, buttons: u8) {
    let buttons = InputAllow::from_bits(buttons).unwrap();
    let control_module = *(module_accessor as *const *const u64).add(0x48 / 8);
    let command_input = *control_module.add((0x7f0 + (command * 8)) / 8) as *mut CommandInputState;
    (*command_input).input_allow = buttons;
}

/// Sets a command input function.
pub unsafe fn set_command_input_function(
    module_accessor: *mut BattleObjectModuleAccessor,
    command: usize,
    function: unsafe extern "C" fn(&mut CommandInputState, *const CommandInputFlags, f32) -> bool) {
    let control_module = *(module_accessor as *const *const u64).add(0x48 / 8);
    let command_input = *control_module.add((0x7f0 + (command * 8)) / 8) as *mut CommandInputState;
    *((*command_input).vtable as *mut u64).add(0x3) = function as u64;
}

/// Clones a command input to another cat4 flag
pub unsafe fn clone_command_input(module_accessor: *mut BattleObjectModuleAccessor, command: usize, replace_command: usize) {
    let control_module = *(module_accessor as *const *const u64).add(0x48 / 8);
    let original = *control_module.add((0x7f0 + (command * 8)) / 8) as *mut CommandInputState;
    let replace = *control_module.add((0x7f0 + (replace_command * 8)) / 8) as *mut CommandInputState;
    *replace = *original.clone();
}

unsafe extern "C" fn special_360_input(
    class: &mut CommandInputState,
    args: *const CommandInputFlags,
    lr: f32
) -> bool {
    let data = *args.add(2);
    if !data.intersects(CommandInputFlags::ANY_DIRECTION)
    && class.state != 0
    && class.state != 6
    && class.unk2 != 0 {
        class.command_timer = 0;
        class.state = 0;
        return false;
    }

    match class.state {
        0 => {
            if data.front(lr) {
                class.state = 1;
                class.lr = lr as i8;
            }
            //if data.front(lr) {
            //    class.state = 7;
            //    class.lr = lr as i8;
            //}
            false
        }
        // Forwards
        1 => {
            if data.front_down(class.lr as f32) {
                class.state = 2;
                class.command_timer = 0;
            }
            false
        }
        2 => {
            if data.down() {
                class.state = 3;
                class.command_timer = 0;
            }
            false
        }
        3 => {
            if data.back_down(class.lr as f32) {
                class.state = 4;
                class.command_timer = 0;
            }
            false
        }
        4 => {
            if data.back(class.lr as f32) {
                class.state = 5;
                class.command_timer = 0;
            }
            false
        }
        5 => {
            if data.front(class.lr as f32)
            || data.front_down(class.lr as f32)
            || data.front_up(class.lr as f32){
                class.state = 6;
                class.command_timer = 0;
                return class.input_allow.check(&data);
            }
            false
        }
        // Backwards
        //7 => {
        //    if data.front_down(class.lr as f32) {
        //        class.state = 8;
        //        class.command_timer = 0;
        //    }
        //    false
        //}
        //8 => {
        //    if data.down() {
        //        class.state = 9;
        //        class.command_timer = 0;
        //    }
        //    false
        //}
        //9 => {
        //    if data.back_down(class.lr as f32) {
        //        class.state = 10;
        //        class.command_timer = 0;
        //    }
        //    false
        //}
        //10 => {
        //    if data.back(class.lr as f32) {
        //        class.state = 11;
        //        class.command_timer = 0;
        //    }
        //    false
        //}
        //11 => {
        //    if data.front(class.lr as f32) {
        //        class.state = 6;
        //        class.command_timer = 0;
        //        return class.input_allow.check(&data);
        //    }
        //    false
        //}
        // Finish
        6 => class.input_allow.check(&data),
        _ => {
            unreachable!()
        }
    }
}

unsafe extern "C" fn special_46_input(
    class: &mut CommandInputState,
    args: *const CommandInputFlags,
    lr: f32
) -> bool {
    let class : &mut CommandInputStateHold = std::mem::transmute(class);
    let data = *args.add(2);
    if !data.intersects(CommandInputFlags::ANY_DIRECTION)
    && class.state != 0
    && class.unk2 != 0 {
        class.command_timer = 0;
        return false;
    }
    println!("lr:{}",lr);
    println!("class_lr:{}",class.lr as f32);
    match class.state {
        0 => {
            println!("case 0");
            if data.back(lr)
            || data.back_down(lr)
            || data.back_up(lr) {
                class.command_timer = 0;
                class.state = 1;
                class.lr = lr as i8;
                class.hold_timer = 1;
            }
            false
        }
        1 => {
            println!("case 1");
            if data.back(class.lr as f32)
            || data.back_down(class.lr as f32)
            || data.back_up(class.lr as f32) {
                class.command_timer = 0;
                let timer = class.hold_timer + 1;
                class.hold_timer = timer;
                if class.hold_timer_max <= timer {
                    println!("hold timer max: {}", class.hold_timer_max);
                    class.state = 2;
                }
            }
            false
        }
        2 => {
            println!("case 2");
            if data.back(class.lr as f32)
            || data.back_down(class.lr as f32)
            || data.back_up(class.lr as f32) {
                class.command_timer = 0;
            }
            if data.front(class.lr as f32)
            || data.front_down(class.lr as f32)
            || data.front_up(class.lr as f32) {
                class.command_timer = 0;
                class.state = 4;
                return class.input_allow.check(&data);
            }
            if !data.intersects(CommandInputFlags::ANY_DIRECTION) {
                class.command_timer = 0;
                class.state = 3;
            }
            false
        }
        3 => {
            println!("case 3");
            if data.front(class.lr as f32)
            || data.front_down(class.lr as f32)
            || data.front_up(class.lr as f32) {
                class.command_timer = 0;
                class.state = 4;
                return class.input_allow.check(&data);
            }
            false
        }
        4 => {
            println!("case 4");
            class.input_allow.check(&data)
        },
        _ => {
            unreachable!()
        }
    }
}
pub unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    set_command_input_function(fighter.module_accessor, 0x7, special_46_input); //hammerfall
    set_command_input_function(fighter.module_accessor, 0xA, special_360_input); //pot buster
    set_command_input_button(fighter.module_accessor, 0x3, 0x2);
    set_command_input_button(fighter.module_accessor, 0x2, 0x2);
    set_command_input_button(fighter.module_accessor, 0x0, 0x2);
    set_command_input_button(fighter.module_accessor, 0x7, 0x3);
    set_command_input_button(fighter.module_accessor, 0xA, 0x1);
}
