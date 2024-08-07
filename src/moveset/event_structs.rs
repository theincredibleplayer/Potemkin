use crate::moveset::*;

#[allow(non_snake_case)]

#[repr(C)]
pub struct CollisionLogScuffed {
    pub x00: *const u64,
    pub x08: *const u64,
    pub location: smash_rs::cpp::simd::Vector3,
    pub x20: u8,
    pub x21: u8,
    pub x22: u8,
    pub x23: u8,
    pub opponent_object_id: u32,
    pub opponent_object_category: u8,
    pub x29: u8,
    pub x2A: u8,
    pub x2B: u8,
    pub x2C: u8,
    pub x2D: u8,
    pub x2E: u8,
    pub collision_kind: u8,
    pub receiver_part_id: u8,
    pub collider_part_id: u8,
    pub receiver_id: u8,
    pub collider_id: u8,
}

#[repr(C)]
pub struct VtableEntry(usize);

impl VtableEntry {
    pub const UNK0: usize = 0;
    pub const UNK1: usize = 1;
    pub const UNK2: usize = 2;
    pub const UNK3: usize = 3;
    pub const INITIALIZE: usize = 4;
    pub const UNK5: usize = 5;
    pub const UNK6: usize = 6;
    pub const ON_DEATH: usize = 7;
    pub const UNK8: usize = 8;
    pub const UNK9: usize = 9;
    pub const UNK10: usize = 10;
    pub const UNK11: usize = 11;
    pub const UNK12: usize = 12;
    pub const ON_FRAME: usize = 13;
    pub const UNK14: usize = 14;
    pub const UNK15: usize = 15;
    pub const UNK16: usize = 16;
    pub const UNK17: usize = 17;
    pub const ON_HIT: usize = 18;
    pub const UNK19: usize = 19;
    pub const UNK20: usize = 20;
    pub const UNK21: usize = 21;
    pub const UNK22: usize = 22;
    pub const UNK23: usize = 23;
    pub const UNK24: usize = 24;
    pub const ON_DEATH_RESET: usize = 25;
    pub const UNK26: usize = 26;
    pub const UNK27: usize = 27;
    pub const UNK28: usize = 28;
    pub const UNK29: usize = 29;
    pub const ON_STATUS_CHANGE: usize = 30;
    pub const ON_SITUATION_CHANGE: usize = 31;
    pub const UNK32: usize = 32;
    pub const UNK33: usize = 33;
    pub const UNK34: usize = 34;
    pub const UNK35: usize = 35;
    pub const ON_ATTACK: usize = 36;
    pub const UNK37: usize = 37;
    pub const UNK38: usize = 38;
    pub const UNK39: usize = 39;
    pub const UNK40: usize = 40;
    pub const ON_ATTACK_EVENT: usize = 41;
    pub const UNK42: usize = 42;
    pub const SOME_GANON_EVENT: usize = 43;
    pub const UNK44: usize = 44;
    pub const SECOND_SITUATION_CALLBACK: usize = 45;
    pub const ON_LINK_EVENT: usize = 46;
    pub const UNK47: usize = 47;
    pub const ON_SEARCH: usize = 48;
    pub const SOME_KIRBY_EVENT: usize = 49;
    pub const UNK50: usize = 50;
    pub const ON_SHIELD_HIT: usize = 51;
    pub const UNK52: usize = 52;
    pub const UNK53: usize = 53;
    pub const UNK54: usize = 54;
    pub const ON_SHIELD: usize = 55;                         //called on eflection
    pub const UNK56: usize = 56;
    pub const UNK57: usize = 57;
    pub const ON_NOTIFY_EVENT_MSC_CMD: usize = 58;
    pub const ON_CHANGE_MOTION: usize = 59;
    pub const UNK60: usize = 60;
    pub const UNK61: usize = 61;
    pub const UNK62: usize = 62;
    pub const UNK63: usize = 63;
    pub const UNK64: usize = 64;
    pub const UNK65: usize = 65;
    pub const UNK66: usize = 66;
    pub const UNK67: usize = 67;
    pub const ON_DAMAGE: usize = 68;
    pub const UNK69: usize = 69;
    pub const UNK70: usize = 70;
    pub const UNK71: usize = 71;
    pub const UNK72: usize = 72;
    pub const UNK73: usize = 73;
    pub const UNK74: usize = 74;
    pub const UNK75: usize = 75;
    pub const UNK76: usize = 76;
    pub const UNK77: usize = 77;
    pub const UNK78: usize = 78;
    pub const UNK79: usize = 79;
    pub const UNK80: usize = 80;
    pub const UNK81: usize = 81;
    pub const UNK82: usize = 82;
    pub const UNK83: usize = 83;
    pub const UNK84: usize = 84;
    pub const UNK85: usize = 85;
    pub const UNK86: usize = 86;
    pub const UNK87: usize = 87;
    pub const UNK88: usize = 88;
    pub const UNK89: usize = 89;
    pub const UNK90: usize = 90;
    pub const UNK91: usize = 91;
    pub const UNK92: usize = 92;
    pub const UNK93: usize = 93;
    pub const UNK94: usize = 94;
    pub const ON_REBIRTH: usize = 95;
    pub const UNK96: usize = 96;
    pub const UNK97: usize = 97;
    pub const UNK98: usize = 98;
    pub const UNK99: usize = 99;
    pub const UNK100: usize = 100;
    pub const ON_STATUS_TRANSITION: usize = 101;
    pub const UNK102: usize = 102;
    pub const UNK103: usize = 103;
    pub const UNK104: usize = 104;
    pub const UNK105: usize = 105;
    pub const UNK106: usize = 106;
    pub const UNK107: usize = 107;
    pub const UNK108: usize = 108;
    pub const UNK109: usize = 109;
    pub const UNK110: usize = 110;
    pub const UNK111: usize = 111;
    pub const UNK112: usize = 112;
    pub const UNK113: usize = 113;
    pub const UNK114: usize = 114;
    pub const UNK115: usize = 115;
    pub const UNK116: usize = 116;
    pub const UNK117: usize = 117;
    pub const UNK118: usize = 118;
    pub const UNK119: usize = 119;
    pub const UNK120: usize = 120;
    pub const UNK121: usize = 121;
    pub const UNK122: usize = 122;
    pub const UNK123: usize = 123;
    pub const UNK124: usize = 124;
    pub const UNK125: usize = 125;
    pub const UNK126: usize = 126;
    pub const UNK127: usize = 127;
    pub const UNK128: usize = 128;
    pub const UNK129: usize = 129;
    pub const UNK130: usize = 130;
    pub const UNK131: usize = 131;
    pub const UNK132: usize = 132;
    pub const UNK133: usize = 133;
    pub const UNK134: usize = 134;
    pub const UNK135: usize = 135;
    pub const UNK136: usize = 136;
    pub const UNK137: usize = 137;
    pub const UNK138: usize = 138;
    pub const UNK139: usize = 139;
    pub const UNK140: usize = 140;
    pub const UNK141: usize = 141;
    pub const UNK142: usize = 142;
    pub const UNK143: usize = 143;
    pub const UNK144: usize = 144;
    pub const UNK145: usize = 145;
}