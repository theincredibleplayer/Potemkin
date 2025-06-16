//huge thanks to the goat WuBoy for this code
use smash::{lua2cpp::*, app::*};
use smashline::*;
use crate::moveset::*;
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct CommandInputFlags: u32 {
        const UP = 0b10;
        const DOWN = 0b100;
        const LEFT = 0b1000;
        const RIGHT = 0b10000;

        const UP_LEFT = 0b100000;
        const DOWN_LEFT = 0b1000000;
        const UP_RIGHT = 0b10000000;
        const DOWN_RIGHT = 0b100000000;

        const ATTACK_EDGE = 0b1000000000;
        const SPECIAL_EDGE = 0b10000000000;
        const GRAB_EDGE = 0b100000000000;

        const ATTACK_PRESSING = 0b1000000000000;
        const SPECIAL_PRESSING = 0b10000000000000;
        const ATTACK_RAW_PRESSING = 0b100000000000000; // IDK CHIEF

        const ANY_DIRECTION = 0x1FE;
    }

    #[derive(Debug, Copy, Clone)]
    pub struct InputAllow: u8 {
        const ATTACK = 0x1;
        const SPECIAL = 0x2;
    }
}

impl CommandInputFlags {
    pub fn back(&self, lr: f32) -> bool {
        if lr < 0.0 {
            self.intersects(Self::RIGHT)
        } else {
            self.intersects(Self::LEFT)
        }
    }

    pub fn back_down(&self, lr: f32) -> bool {
        if lr < 0.0 {
            self.intersects(Self::DOWN_RIGHT)
        } else {
            self.intersects(Self::DOWN_LEFT)
        }
    }

    pub fn back_up(&self, lr: f32) -> bool {
        if lr < 0.0 {
            self.intersects(Self::UP_RIGHT)
        } else {
            self.intersects(Self::UP_LEFT)
        }
    }

    pub fn front(&self, lr: f32) -> bool {
        if lr > 0.0 {
            self.intersects(Self::RIGHT)
        } else {
            self.intersects(Self::LEFT)
        }
    }

    pub fn front_down(&self, lr: f32) -> bool {
        if lr > 0.0 {
            self.intersects(Self::DOWN_RIGHT)
        } else {
            self.intersects(Self::DOWN_LEFT)
        }
    }

    pub fn front_up(&self, lr: f32) -> bool {
        if lr > 0.0 {
            self.intersects(Self::UP_RIGHT)
        } else {
            self.intersects(Self::UP_LEFT)
        }
    }

    pub fn up(&self) -> bool {
        self.intersects(Self::UP)
    }

    pub fn down(&self) -> bool {
        self.intersects(Self::DOWN)
    }

    pub fn left(&self) -> bool {
        self.intersects(Self::LEFT)
    }

    pub fn right(&self) -> bool {
        self.intersects(Self::RIGHT)
    }

    pub fn up_right(&self) -> bool {
        self.intersects(Self::UP_RIGHT)
    }

    pub fn up_left(&self) -> bool {
        self.intersects(Self::UP_LEFT)
    }

    pub fn down_right(&self) -> bool {
        self.intersects(Self::DOWN_RIGHT)
    }

    pub fn down_left(&self) -> bool {
        self.intersects(Self::DOWN_LEFT)
    }
}

impl InputAllow {
    pub fn check(&self, inputs: &CommandInputFlags) -> bool {
        let mut input = false;
        if self.intersects(Self::ATTACK) && inputs.intersects(CommandInputFlags::ATTACK_EDGE) {
            input |= true;
        }

        if self.intersects(Self::SPECIAL) && inputs.intersects(CommandInputFlags::SPECIAL_EDGE) {
            input |= true;
        }

        input
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CommandInputState {
    pub vtable: u64,
    pub command_timer: u8,
    pub state: u8,
    pub unk2: u8,
    pub input_allow: InputAllow,
    pub max_timer: u8,
    pub enable_timer: u8,
    pub lr: i8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CommandInputStateHold {
    pub input_state: CommandInputState,
    pub hold_timer_max: u16,
    pub hold_timer: u16
}

impl core::ops::Deref for CommandInputStateHold {
    type Target = CommandInputState;

    fn deref(&self) -> &Self::Target {
        &self.input_state
    }
}

impl core::ops::DerefMut for CommandInputStateHold {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.input_state
    }
}

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
            if data.back(lr) {
                class.state = 1;
                class.lr = lr as i8;
            }
            if data.front(lr) {
                class.state = 7;
                class.lr = lr as i8;
            }
            false
        }
        // Forwards
        1 => {
            if data.back_down(class.lr as f32) {
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
            if data.front_down(class.lr as f32) {
                class.state = 4;
                class.command_timer = 0;
            }
            false
        }
        4 => {
            if data.front(class.lr as f32) {
                class.state = 5;
                class.command_timer = 0;
            }
            false
        }
        5 => {
            if data.front_up(class.lr as f32) {
                class.state = 6;
                class.command_timer = 0;
                return class.input_allow.check(&data);
            }
            false
        }
        // Backwards
        7 => {
            if data.front_down(class.lr as f32) {
                class.state = 8;
                class.command_timer = 0;
            }
            false
        }
        8 => {
            if data.down() {
                class.state = 9;
                class.command_timer = 0;
            }
            false
        }
        9 => {
            if data.back_down(class.lr as f32) {
                class.state = 10;
                class.command_timer = 0;
            }
            false
        }
        10 => {
            if data.back(class.lr as f32) {
                class.state = 11;
                class.command_timer = 0;
            }
            false
        }
        11 => {
            if data.back_up(class.lr as f32) {
                class.state = 6;
                class.command_timer = 0;
                return class.input_allow.check(&data);
            }
            false
        }
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
}
