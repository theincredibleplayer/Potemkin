use super::*;
use skyline::hooks::{Region,getRegionAddress};
use skyline::nn::ro::LookupSymbol;
use std::sync::atomic::{AtomicUsize, Ordering};

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
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SPECIAL_S_COMMAND : i32 = 0x200000f6;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_CAPTURE : i32 = 0x200000f7;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_HIT : i32 = 0x200000f8;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_FINAL_JAB_HIT_MOTION_PLAY : i32 = 0x200000f9;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_PURPLE_RC : i32 = 0x200000fa;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RED_RC : i32 = 0x200000fb;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_YELLOW_RC : i32 = 0x200000fc;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_BLUE_RC : i32 = 0x200000fd;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FAST_RC_ENABLE : i32 = 0x200000fe;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FAST_RC_DETECTED : i32 = 0x200000ff;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED : i32 = 0x20000100;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED_2 : i32 = 0x20000101;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_SOUND_ID_KILL : i32 = 0x20000104;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_WIN_FRAME : i32 = 0x20000105;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_RC_DASH : i32 = 0x20000106;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_STOP_FINAL_I_FRAMES : i32 = 0x20000107;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ARMOR_ON : i32 = 0x20000108;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ANIMATED : i32 = 0x20000109;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_FLIP : i32 = 0x2000010a;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_ATTACH_WEAPON : i32 = 0x2000010b;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RC_CLOCK_EFFECT : i32 = 0x2000010c;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_START : i32 = 0x2000010d;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_START : i32 = 0x2000010e;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_ATTACK_START : i32 = 0x2000010f;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_JUMP_START : i32 = 0x20000110;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SET_ADJUST_POST_CIRCLE : i32 = 0x20000111;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_FINAL_CANCELLED_INTO : i32 = 0x20000112;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RC_INITIAL_SLOW : i32 = 0x20000113;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_RC_END_SLOW : i32 = 0x20000114;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DEATH_IMMINENT : i32 = 0x20000115;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ATTACK_CANCELLED_INTO : i32 = 0x20000116;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_ENABLE_KARA : i32 = 0x20000117;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_INPUT : i32 = 0x20000118;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FORCE_RED_CANCEL : i32 = 0x20000119;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_ENABLE_CAMERA : i32 = 0x2000011a;


// for weapon
pub const WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_ATTACK : i32 = 0x20000007;
pub const WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_HIT_WALL : i32 = 0x20000008;
pub const WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_DID_IT_HIT : i32 = 0x20000009;
pub const WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_KILL_YOURSELF_NOW : i32 = 0x2000000a;




// floats
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_ACCEL: i32 = 0x60;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_TIMER: i32 = 0x61;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FLIP_SELECTOR: i32 = 0x62;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_RC_MOVE_X: i32 = 0x63;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_RC_MOVE_Y: i32 = 0x64;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_RC_LIMITING_SPEED: i32 = 0x65;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_RC_SPEED_MUL: i32 = 0x66;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_PRE_RC_X_SPEED: i32 = 0x67;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_PRE_RC_Y_SPEED: i32 = 0x68;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_REAL_POWER_REFLECT: i32 = 0x69;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_ROMAN_CANCEL_METER_DRAIN: i32 = 0x6a;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_OPPONENT_Z_SCALE: i32 = 0x6b;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FIRE_TIME_BASE: i32 = 0x6c;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_HEAT_TACKLE_STICK_X: i32 = 0x6d;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_HEAT_TACKLE_STICK_Y: i32 = 0x6e;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_HEAT_TACKLE_SPEED: i32 = 0x6f;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SLOW_MUL: i32 = 0x70;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_POTEMKIN_Z_SCALE: i32 = 0x71;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_X_JUMP_SPEED: i32 = 0x72;

pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_LOSE_POS_X: i32 = 0x73;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_LOSE_POS_Y: i32 = 0x74;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_LOSE_POS_Z: i32 = 0x75;


// for weapon
pub const WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLOAT_BOOM_POWER: i32 = 0x4;

// ints
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_GRABBED_ENEMY_ID: i32 = 0x100000c1;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_HPB_GOTTEM_ID: i32 = 0x100000c2;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LOOP_HITS_SPECIAL_HI: i32 = 0x100000c3;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_ARTICLE_ID: i32 = 0x100000c4;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FINAL_MOTION_SHARE: i32 = 0x100000c5;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FIRE_TIMER: i32 = 0x100000c6;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_FREEZE_DURATION: i32 = 0x100000c7;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_SLOW_START_TIME: i32 = 0x100000c8;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FAST_RC_TIME: i32 = 0x100000c9;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_RC_PENALTY_TIMER: i32 = 0x100000ca;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_SLOW_CLEAR_TIME: i32 = 0x100000cb;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LAST_ATTACK_SOUND_ID: i32 = 0x100000cc;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LAST_SPECIAL_SOUND_ID: i32 = 0x100000cd;
pub const FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_ROMAN_CANCEL_BUFFER_DIRECTION: i32 = 0x100000ce;


// for weapon
pub const WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_INT_TIMER: i32 = 0x10000524;

// new statuses
pub const FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_START: i32 = 0x1f4;
pub const FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_LOOP: i32 = 0x1f5;
pub const FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_RELEASE: i32 = 0x1f6;
pub const FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_START: i32 = 0x1f7;
pub const FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_LOOP: i32 = 0x1f8;
pub const FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_ATTACK: i32 = 0x1f9;
pub const FIGHTER_KAMUI_STATUS_KIND_FINAL_JUMP: i32 = 0x1fa;
pub const FIGHTER_KAMUI_STATUS_KIND_FINAL_FAKE_VISSCENE: i32 = 0x1fb;
pub const FIGHTER_KAMUI_STATUS_KIND_ROMAN_CANCEL: i32 = 0x1fc;
pub const FIGHTER_KAMUI_STATUS_KIND_WIN: i32 = 0x1fd;
pub const FIGHTER_KAMUI_STATUS_KIND_AIR_THROW: i32 = 0x1fe;

pub const FIGHTER_KAMUI_STATUS_KIND_WALK_BACK: i32 = 0x200;
pub const FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN: i32 = 0x201;
pub const FIGHTER_KAMUI_STATUS_KIND_WALK_BRAKE_BACK: i32 = 0x202;

pub const FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER_WHIFF: i32 = 0x203;
pub const FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER: i32 = 0x204;

pub const FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_HOLD: i32 = 0x205;
pub const FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE: i32 = 0x206;
pub const FIGHTER_KAMUI_STATUS_KIND_HEAT_TACKLE_END: i32 = 0x207;

pub const FIGHTER_KAMUI_STATUS_KIND_AIR_AUTO_TURN: i32 = 0x208;
pub const FIGHTER_KAMUI_STATUS_KIND_SQUAT_AUTO_TURN: i32 = 0x209;

pub const FIGHTER_KAMUI_STATUS_KIND_ITEM_HEAVY_WAIT: i32 = 0x20a;
pub const FIGHTER_KAMUI_STATUS_KIND_ITEM_HEAVY_JUMP: i32 = 0x20b;
pub const FIGHTER_KAMUI_STATUS_KIND_ITEM_HEAVY_FALL: i32 = 0x20c;
pub const FIGHTER_KAMUI_STATUS_KIND_ITEM_HEAVY_TURN: i32 = 0x20d;


// for weapon
pub const WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM: i32 = 0x1;



// new common statuses
pub const FIGHTER_STATUS_KIND_POT_SPECIALHI_CAUGHT: i32 = 0x555;
pub const FIGHTER_STATUS_KIND_POT_FINAL_CAUGHT: i32 = 0x556;
pub const FIGHTER_STATUS_KIND_POT_BUSTER_CAUGHT: i32 = 0x557;
pub const FIGHTER_STATUS_KIND_POT_AIR_THROW_CAUGHT: i32 = 0x558;
pub const FIGHTER_STATUS_KIND_POT_REBOUND_JUMP: i32 = 0x559;




// global table consts
pub const KIND:                             i32 = 0x2 ;
pub const CMD_CAT1:                         i32 = 0x20;
pub const CMD_CAT2:                         i32 = 0x21;
pub const CMD_CAT3:                         i32 = 0x22;
pub const CMD_CAT4:                         i32 = 0x23;
pub const CHECK_SPECIAL_COMMAND:            i32 = 0x3C;
pub const SITUATION_KIND:                   i32 = 0x16;
pub const PREV_SITUATION_KIND:              i32 = 0x17;
pub const STICK_X:                          i32 = 0x1A;
pub const STICK_Y:                          i32 = 0x1B;
pub const PREV_STATUS_KIND:                 i32 = 0xA ;
pub const STATUS_KIND:                      i32 = 0xB ;
pub const IS_STOP:                          i32 = 0x8 ;
pub const STATUS_KIND_INTERRUPT:            i32 = 0x9 ;
pub const CHECK_AIR_SPECIAL_UNIQ:           i32 = 0x26;
pub const STATUS_END_CONTROL:               i32 = 0x3E;
pub const CHECK_SPECIAL_S_UNIQ:             i32 = 0x39;


macro_rules! offset_statics {
    ($($name:ident),* $(,)?) => { $(pub static $name: AtomicUsize = AtomicUsize::new(0);)* };
}
offset_statics!(
    RESET_CLIFF_HANGDATA_HOOK_OFFSET     ,
    AUTOTURN_SITUATION_CHECK_HOOK_OFFSET ,
    CAPTURE_2_HOOK_OFFSET                ,
    WATERBALL_EFFECT_HANDELER_OFFSET     ,
    KAMUI_SWORD_EFFECT_HANDELER_OFFSET   ,
    GET_BATTLE_OBJECT_FROM_ID_OFFSET     ,
    SET_STAGE_VISIBILITY_FUNCTION_OFFSET ,
    GET_OPPONENT_LR_FUNCTION_OFFSET      ,
    SET_LR_FLOAT_FUNCTION_OFFSET         ,
    FINAL_SMASH_CHECKS_FUNCTION_OFFSET   ,
    UI_DRAW_HOOK                         ,
);

struct InstructionSearchItem {
    name: &'static str,
    pattern: &'static [u8],
    adjust: isize,
    out: &'static AtomicUsize,
}

static OFFSET_SEARCH_RESET_CLIFF_HANGDATA: &[u8] = &[
    0xff, 0x03, 0x02, 0xd1, 0xe9, 0x23, 0x04, 0x6d, 0xf6, 0x57, 0x05, 0xa9, 0xf4, 0x4f, 0x06, 0xa9, 0xfd, 0x7b, 0x07, 0xa9, 0xfd, 0xc3, 0x01, 0x91, 0x08, 0xa0, 0x40, 0xf9, 0x48, 0x09, 0x00, 0xb4
];

static OFFSET_SEARCH_AUTOTURN_SITUATION_HOOK: &[u8] = &[
    0xe0, 0x02, 0x00, 0x34, 0xe8, 0x03, 0x27, 0x1e, 0x0a, 0x00, 0x00, 0x14, 0x48, 0x70, 0x00, 0x51, 0x1f, 0x29, 0x02, 0x71, 0xc8, 0xfe, 0xff, 0x54
];

static OFFSET_SEARCH_CAPTURE_2_HOOK: &[u8] = &[
    0xe0, 0x03, 0x19, 0x2a, 0xbf, 0xf9, 0xfe, 0x97, 0xa0, 0x00, 0x00, 0xb4, 0x08, 0xe8, 0x40, 0x39, 0xf5, 0x03, 0x00, 0xaa, 0x1f, 0x0d, 0x00, 0x71, 0xe8, 0x00, 0x00, 0x54
]; // this is 2 instructions down


static OFFSET_SEARCH_GET_BATTLE_OBJECT_FROM_ID_OFFSET: &[u8] = &[
    0x1f, 0x60, 0x02, 0x39, 0xc0, 0x03, 0x5f, 0xd6, 0x00, 0x00, 0x00, 0x00, 0x08, 0x7c, 0x1c, 0x53, 0x1f, 0x11, 0x00, 0x71, 0x68, 0x07, 0x00, 0x54
]; // this is 3 instructions down from the target

static OFFSET_SEARCH_SET_STAGE_VISIBILITY_FUNCTION_OFFSET: &[u8] = &[
    0x0b, 0xf9, 0x02, 0x39, 0xfd, 0x7b, 0x45, 0xa9, 0xf4, 0x4f, 0x44, 0xa9, 0xf6, 0x57, 0x43, 0xa9, 0xf8, 0x5f, 0x42, 0xa9, 0xff, 0x83, 0x01, 0x91, 0xc0, 0x03, 0x5f, 0xd6, 0xf6, 0x57, 0xbd, 0xa9, 0xf4, 0x4f, 0x01, 0xa9, 0xfd, 0x7b, 0x02, 0xa9, 0xfd, 0x83, 0x00, 0x91
]; // this is 7 instructions down from the target

static OFFSET_SEARCH_GET_OPPONENT_LR_FUNCTION_OFFSET: &[u8] = &[
    0xff, 0x03, 0x03, 0xd1, 0xeb, 0x2b, 0x04, 0x6d, 0xe9, 0x23, 0x05, 0x6d, 0xfc, 0x6f, 0x06, 0xa9, 0xfa, 0x67, 0x07, 0xa9, 0xf8, 0x5f, 0x08, 0xa9, 0xf6, 0x57, 0x09, 0xa9, 0xf4, 0x4f, 0x0a, 0xa9, 0xfd, 0x7b, 0x0b, 0xa9, 0xfd, 0xc3, 0x02, 0x91, 0xf3, 0x03, 0x01, 0x2a, 0xf5, 0x03, 0x00, 0xaa, 0x23, 0x01, 0x00, 0x36, 0xd3, 0x00, 0x00, 0x37
];

static OFFSET_SEARCH_SET_LR_FLOAT_FUNCTION_OFFSET: &[u8] = &[
    0xe9, 0x23, 0xbb, 0x6d, 0xf7, 0x0b, 0x00, 0xf9, 0xf6, 0x57, 0x02, 0xa9, 0xf4, 0x4f, 0x03, 0xa9, 0xfd, 0x7b, 0x04, 0xa9, 0xfd, 0x03, 0x01, 0x91, 0x13, 0xd0, 0x44, 0xa9, 0x88, 0x02, 0x40, 0xf9, 0x08, 0x2d, 0x40, 0xf9, 0xf6, 0x03, 0x01, 0x2a
];

static OFFSET_SEARCH_FINAL_SMASH_CHECKS_FUNCTION_OFFSET: &[u8] = &[
    0xf4, 0x4f, 0xbe, 0xa9, 0xfd, 0x7b, 0x01, 0xa9, 0xfd, 0x43, 0x00, 0x91, 0x08, 0x00, 0x40, 0xf9, 0x08, 0x85, 0x40, 0xf9, 0x34, 0x01, 0x80, 0x52, 0x14, 0x00, 0xa4, 0x72, 0x81, 0x06, 0x00, 0x11, 0xf3, 0x03, 0x00, 0xaa, 0x00, 0x01, 0x3f, 0xd6
];

static OFFSET_SEARCH_UI_DRAW_HOOK: &[u8] = &[
    0xfd, 0x43, 0x01, 0x91, 0xf4, 0x03, 0x00, 0xaa, 0xf5, 0x03, 0x02, 0x2a, 0xe0, 0x23, 0x00, 0x91, 0xe2, 0x03, 0x14, 0xaa, 0xf3, 0x03, 0x01, 0xaa, 0x50, 0x3a, 0x00, 0x94, 0x74, 0x62, 0x00, 0xf9, 0x80, 0x0e, 0x40, 0xf9
];

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}
pub fn install() {
    unsafe {
        static INST_SEARCH_ARRAY: &[InstructionSearchItem] = &[
            // hooks
            InstructionSearchItem { name: "reset_cliff_hangdata_hook"   , pattern: OFFSET_SEARCH_RESET_CLIFF_HANGDATA                   , adjust: 0, out: &RESET_CLIFF_HANGDATA_HOOK_OFFSET     },
            InstructionSearchItem { name: "autoturn_situation_hook"     , pattern: OFFSET_SEARCH_AUTOTURN_SITUATION_HOOK                , adjust: 0, out: &AUTOTURN_SITUATION_CHECK_HOOK_OFFSET },
            InstructionSearchItem { name: "capture_2_hook"              , pattern: OFFSET_SEARCH_CAPTURE_2_HOOK                         , adjust: 2, out: &CAPTURE_2_HOOK_OFFSET                },
            InstructionSearchItem { name: "ui_draw_hook"                , pattern: OFFSET_SEARCH_UI_DRAW_HOOK                           , adjust: 0, out: &UI_DRAW_HOOK                         },
            // non hooks
            InstructionSearchItem { name: "get_battle_object_from_id"   , pattern: OFFSET_SEARCH_GET_BATTLE_OBJECT_FROM_ID_OFFSET       , adjust: 3, out: &GET_BATTLE_OBJECT_FROM_ID_OFFSET     },
            InstructionSearchItem { name: "set_stage_visibility"        , pattern: OFFSET_SEARCH_SET_STAGE_VISIBILITY_FUNCTION_OFFSET   , adjust: 7, out: &SET_STAGE_VISIBILITY_FUNCTION_OFFSET },
            InstructionSearchItem { name: "get_opponent_lr"             , pattern: OFFSET_SEARCH_GET_OPPONENT_LR_FUNCTION_OFFSET        , adjust: 0, out: &GET_OPPONENT_LR_FUNCTION_OFFSET      },
            InstructionSearchItem { name: "set_lr_float"                , pattern: OFFSET_SEARCH_SET_LR_FLOAT_FUNCTION_OFFSET           , adjust: 0, out: &SET_LR_FLOAT_FUNCTION_OFFSET         },
            InstructionSearchItem { name: "final_smash_checks"          , pattern: OFFSET_SEARCH_FINAL_SMASH_CHECKS_FUNCTION_OFFSET     , adjust: 0, out: &FINAL_SMASH_CHECKS_FUNCTION_OFFSET   },
        ];
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        for item in INST_SEARCH_ARRAY{
            if let Some(offset) = find_subsequence(text, item.pattern) {
                item.out.store((offset as isize + item.adjust * 4)as usize, Ordering::Relaxed);
            }
            println!("{}: {:x}",item.name,item.out.load(Ordering::Relaxed));
        }
    }
}
pub static mut FIGHTER_KAMUI_GENERATE_ARTICLE_FDB : i32 = 0;
pub static mut WEAPON_KIND_KAMUI_FDB : i32 = 0;
