use super::*;


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
pub struct CollisionLog {
    pub next: *mut CollisionLog,
    pub end: *mut CollisionLog,
    pub location: Vector3f,
    pub padding_0: u32,
    pub padding_1: u32,
    pub opponent_battle_object_id: u32,
    pub padding_2: [u8;7],
    pub collision_kind: u8,
    pub receiver_part_id: u8,
    pub collider_part_id: u8,
    pub receiver_id: u8,
    pub collider_id: u8,
    pub padding_3: [u8;10]
}


#[repr(C)]
pub struct ShieldAttackCollisionEvent {
    pub vtable: u64,
    pub shield_id: u32,
    pub unk: u8,
    pub unk1: u8,
    pub unk2: u8,
    pub unk3: u8,
    pub attack_module: u64,
    pub raw_power: f32,
    pub real_power: f32,
    pub collision_log: *const CollisionLogScuffed,
    pub group_index: i32,
    pub pos_x: f32,
    pub lr: f32,
    pub unk4: u8,
    pub unk5: u8,
    pub unk6: u8,
    pub unk7: u8,
}

impl ShieldAttackCollisionEvent {
    pub fn new(
        vtable: u64,
        shield_id: u32,
        unk: u8,
        unk1: u8,
        unk2: u8,
        unk3: u8,
        attack_module: u64,
        raw_power: f32,
        real_power: f32,
        collision_log: *const CollisionLogScuffed,
        group_index: i32,
        pos_x: f32,
        lr: f32,
        unk4: u8,
        unk5: u8,
        unk6: u8,
        unk7: u8
    ) -> Self {
        ShieldAttackCollisionEvent {
            vtable: vtable,
            shield_id: shield_id,
            unk: unk,
            unk1: unk1,
            unk2: unk2,
            unk3: unk3,
            attack_module: attack_module,
            raw_power: raw_power,
            real_power: real_power,
            collision_log: collision_log,
            group_index: group_index,
            pos_x: pos_x,
            lr: lr,
            unk4: unk4,
            unk5: unk5,
            unk6: unk6,
            unk7: unk7
        }
    }
}



#[repr(C)]
pub struct ShieldDataResource {
    pub offset: smash_rs::cpp::simd::Vector3,
    pub offset2: smash_rs::cpp::simd::Vector3,
    pub size: f32,
    pub x24: u32,
    pub joint: smash::phx::Hash40,
    pub shape: u8,
    pub shield_type: u8,
}

impl ShieldDataResource {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        x2: f32,
        y2: f32,
        z2: f32,
        size: f32,
        joint: smash::phx::Hash40,
        shape: u8,
        shield_type: u8
    ) -> Self {
        ShieldDataResource {
            offset: smash_rs::cpp::simd::Vector3{vec: [x, y, z]},
            offset2: smash_rs::cpp::simd::Vector3{vec: [x2, y2, z2]},
            size: size,
            x24: 0,
            joint: joint,
            shape: shape,
            shield_type: shield_type
        }
    }
}

#[repr(C)]
pub struct ShieldDatas {
    pub datas: [ShieldDataResource; 8]
}

impl ShieldDatas {
    pub fn new() -> ShieldDatas {
        ShieldDatas { datas: [
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0)
        ] }
    }
    pub fn add(mut self, shield_data: ShieldDataResource, index: usize) -> ShieldDatas {
        self.datas[index] = shield_data;
        self
    }
}

#[repr(C)]
pub struct ShieldGroupResource {
    pub shield_datas: *const ShieldDatas,
    pub count: u64,
    pub front: u8,
    pub hop: bool,
    pub turn: bool,
    pub no_hop: bool
}

impl ShieldGroupResource {
    pub fn new(
        shield_datas: *const ShieldDatas,
        count: u64,
        front: u8,
        hop: bool,
        turn: bool,
        no_hop: bool
    ) -> Self {
        ShieldGroupResource {
            shield_datas: shield_datas,
            count: count,
            front: front,
            hop: hop,
            turn: turn,
            no_hop: no_hop
        }
    }
}

// REFLECTOR BOXES

#[repr(C)]
pub struct ShieldData {
    pub offset: smash_rs::cpp::simd::Vector3,
    pub offset2: smash_rs::cpp::simd::Vector3,
    pub size: f32,
    pub x24: u32,
    pub joint: smash::phx::Hash40,
    pub shape: u8,
    pub shield_type: u8,
    pub x32: u16,
    pub x34: u32,
    pub x38: u64,
    pub status: i32,
    pub x44: u32,
    pub x48: u64,
}

impl ShieldData {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        x2: f32,
        y2: f32,
        z2: f32,
        size: f32,
        joint: smash::phx::Hash40,
        shape: u8,
        shield_type: u8
    ) -> Self {
        ShieldData {
            offset: smash_rs::cpp::simd::Vector3{vec: [x, y, z]},
            offset2: smash_rs::cpp::simd::Vector3{vec: [x2, y2, z2]},
            size: size,
            x24: 0,
            joint: joint,
            shape: shape,
            shield_type: shield_type,
            x32: 0,
            x34: 0,
            x38: 0,
            status: 0,
            x44: 0,
            x48: 0,
        }
    }
}

#[repr(C)]
pub struct ShieldDatas2 {
    pub datas: [ShieldData; 8]
}

impl ShieldDatas2 {
    pub fn new() -> ShieldDatas2 {
        ShieldDatas2 { datas: [
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0)
        ] }
    }
    pub fn add(mut self, shield_data: ShieldData, index: usize) -> ShieldDatas2 {
        self.datas[index] = shield_data;
        self
    }
}

#[repr(C)]
pub struct ShieldGroupResource2 {
    pub shield_datas: *const ShieldDatas2,
    pub count: u64,
    pub attack_mul: f32,
    pub speed_mul: f32,
    pub attack_limit: f32,
    pub life_mul: f32,
    pub no_m_ball: bool,
    pub front: u8
}

impl ShieldGroupResource2 {
    pub fn new(
        shield_datas: *const ShieldDatas2,
        count: u64,
        attack_mul: f32,
        speed_mul: f32,
        attack_limit: f32,
        life_mul: f32,
        no_m_ball: bool,
        front: u8
    ) -> Self {
        ShieldGroupResource2 {
            shield_datas: shield_datas,
            count: count,
            attack_mul: attack_mul,
            speed_mul: speed_mul,
            attack_limit: attack_limit,
            life_mul: life_mul,
            no_m_ball: no_m_ball,
            front: front
        }
    }
}

#[repr(C)]
pub struct VtableEntry(usize);
                                                                    // as a general rule every vtable has at least 1 usage 
impl VtableEntry {
    pub const INITIALIZE: usize = 0;
    pub const EVENT_LISTENER_INITIALIZE: usize = 1;
    pub const UNK2: usize = 2;                                      // only duck hunt for some item and gnw for event listener initializer use it
    pub const SOME_FUNC_SETTER: usize = 3;
    pub const RESET_1: usize = 4;
    pub const RESET_2: usize = 5;
    pub const RESET_3: usize = 6;                                   // maybe not sure, just used for m2 and snake
    pub const ON_DEATH: usize = 7;
    pub const ON_REBIRTH2: usize = 8;
    pub const ON_FRAME_11: usize = 9;                               // unsure, only used for bowser jr. from findings (its another opff i tested,claude1308) 
    pub const ON_FRAME_1: usize = 10;
    pub const ON_FRAME_2: usize = 11;                               // opff of sorts for some corrective action, like setting shape or detecting distance from ground and taking some action
    pub const ON_FRAME_3: usize = 12;
    pub const ON_FRAME_4: usize = 13;                               // some use for Partial Motion
    pub const UNK14: usize = 14;
    pub const SOME_KIRBY_RELATED: usize = 15;
    pub const MOUTH_BREATHERS_OBJECT_RESET: usize = 16;             // DDD and kirby related
    pub const ON_PRE_DAMAGE: usize = 17;                            // only ever used by snake, it runs right before going into a damage status it seems
    pub const ON_FRAME_5: usize = 18;                               // lol, lmao even
    pub const ON_FRAME_6: usize = 19;                               // seems to be final smash related except on rob and lucario for some reason
                                                                    // i am actually not fully convinced of this naming cuz cloud is doing some weird ass shit i wanna  test when this gets called later
                                                                    // its a fucking opff LMFAOOOOOOOOOOOOOOOOOOOOOOOO
    pub const ON_FRAME_7: usize = 20;                               // this is also a massive guess i should test this later (THIS IS ANOTHER OPFFFF AAAAAAAAAAAAAAAAAAAAAAAAA)
    pub const ON_PAUSE: usize = 21;
    pub const ON_PAUSE_FRAME: usize = 22;                           // its an on frame while paused
    pub const ON_UNPAUSE: usize = 23;
    pub const ON_MATCH_GO: usize = 24;                              // it runs once when the announcer says GO
    pub const ON_DEATH_RESET_1: usize = 25;
    pub const ON_DEATH_RESET_2: usize = 26;                         // seems to be another on death
    pub const ON_FRAME_8: usize = 27;                               // you cant make this shit up LMFAO 7 OPFFS
    pub const ON_FRAME_9: usize = 28;                               // only kirby uses this (im gonna fucking cry)
    pub const ON_FRAME_10: usize = 29;                              // only kirby uses this (i couldnt even roman cancel to this, I BURSTED IMMEDIATLEY)
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
    pub const SOME_GANON_EVENT: usize = 43;                         // apparently this removes the hammer and the kart from the victory screen for koopajr?
    pub const UNK44: usize = 44;
    pub const SECOND_SITUATION_CALLBACK: usize = 45;
    pub const ON_LINK_EVENT: usize = 46;
    pub const UNK47: usize = 47;
    pub const ON_SEARCH: usize = 48;
    pub const SOME_KIRBY_EVENT: usize = 49;
    pub const ON_SHIELD_COLLISION: usize = 50;
    pub const SHIELD_LR_AND_COUNTER_STATUS: usize = 51;
    pub const UNK52: usize = 52;
    pub const UNK53: usize = 53;                                    // shield related but usure what its doing
    pub const UNK54: usize = 54;
    pub const ON_SHIELD: usize = 55;                                // aka on rflection
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
    pub const ON_GRABBED: usize = 66;                               // triggers once a grabbox connects, it also seems to make you immune to being grabbed if the return value isn't 1
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
    pub const UNK122: usize = 122;                                  // something knockout voice related?
    pub const UNK123: usize = 123;
    pub const UNK124: usize = 124;
    pub const UNK125: usize = 125;
    pub const UNK126: usize = 126;
    pub const UNK127: usize = 127;
    pub const UNK128: usize = 128;
    pub const UNK129: usize = 129;
    pub const UNK130: usize = 130;
    pub const UNK131: usize = 131;                                  // might be on status end or on motion end for pacman
                                                                    // for popo and nana its literally the same as the 2 below
                                                                    // no clue what minmin is doing but whenever its called it sets the situation kind to air using:
                                                                    // StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), false);

    pub const UNK132: usize = 132;                                  // literally the same function as 133 but less args i am so confused kys sakurai
    pub const UNK133: usize = 133;                                  // stubbed, shared to everyone except popo and nana
                                                                    // its something sideb and link module related?

    pub const UNK134: usize = 134;                                  // has a whole 5 params, sets 3,4 and 5 to 0 then returns 0, shared for every character except hero
                                                                    // for hero it sets the reflection parameters for bounce
                                                                    // passes the params reflect_attack_mul to param_3, reflect_speed_mul to param_4 and reflect_life_mul to param_5 if bounce is active
                                                                    // if bounce isnt active it sets param_3, param_4 and param_5 to 0
                                                                    // function returns 1 as long as bounce is active, otherwise 0
                                                                    // im saying "passes the params" here under the assumption that these args are passed by refrence cuz it wont really make sense if it just changes local variables and leaves lmao

    pub const UNK135: usize = 135;                                  // stubbed, shared for every character except hero
                                                                    // SetVisibleKind on the "brave_fire3_hold_max" effect and "brave_reflect" effect, true or false depends on the third arg

    pub const UNK136: usize = 136;                                  // always true, shared for every character except master
                                                                    // seems to check if current status is not FIGHTER_MASTER_STATUS_KIND_SPECIAL_HI_HIT and returns true if so 
                                                                    // but it does it in a wierd expression            return (status_kind & 0xfffffffe) != 0x1ea

    pub const UNK137: usize = 137;                                  // always true, shared for every character except elight
                                                                    // interestingly the 2nd arg isnt fighter but instead its battleobject, or so it seems
                                                                    // no clue what param_3 is or what the function is doing

    pub const ON_GRAB: usize = 138;                                 // shared for every character, returns the function's third argument (idk what it is tho) except for demon
                                                                    // demon: checks if param_3 == 0x41, then checks if rage is enabled and some other condition i dont understand, then does some buffering related stuff and returns either 0x41 or 0x1ca depending on outcome
                                                                    // 0x41 seems to be capture pulled and 0x1ca is FIGHTER_STATUS_KIND_DEMON_DIVED
                                                                    // this seems to handle grab buffering when rage drive is active
                                                                    // aka handles weather to select rage drive or a throw
                                                                    // i am so confused
                                                                    // i think this just returns which status to go to if grabbed

    pub const UNK139: usize = 139;                                  // always false, shared for every character
                                                                    // except for demon, if it isnt FIGHTER_DEMON_STATUS_KIND_THROW_COMMAND or FIGHTER_STATUS_KIND_THROW it returns 0, otherwise returns 1

    pub const SORA_EVENT_1: usize = 140;                            // always false, shared for every character except sora
                                                                    // sora: param_3 and param_4 seem to be ints
                                                                    // if param_3 == 3 then some code that i dont understand is ran
                                                                    // if param_3 == 2 && param_4 ==1 it returns 1

    pub const SORA_EVENT_2: usize = 141;                            // always false, shared for every character except sora
                                                                    // sora: i dont exactly know what its doing but its checking if the status is downb and doing stuff

    pub const SORA_EVENT_3: usize = 142;                            // always false, shared for every character except sora
                                                                    // sora: its doing some checks for sora but i cant tell what exactly its doing or what the args are

    pub const RETRUNS_NUMBER_2: usize = 143;                        // shared across the entire cast except sora it , always returns 2
                                                                    // in sora's case it checks if (param_3 == '\x03') and if so returns 3, otherwise returns 2

    pub const RETURN_IN_X3: usize = 144;                            // shared across the entire cast except sora and kazuya it seems, returns whatever was in the register in_x3 + 0x58
                                                                    // sora and kazuya have 14 params to the function and i am not reading allat

    pub const RETURNS_ITEM_HAND_GRIP_HEX: usize = 145;              // shared across the entire cast except minmin it seems, this one always returns the hash for item_hand_grip even with minmin
}

#[repr(C)]
pub struct WeaponVtableEntry(usize);

impl WeaponVtableEntry {
    pub const INITIALIZATION: usize = 0;        //looks like initializing the projectile
    pub const UNK1: usize = 1;
    pub const UNK2: usize = 2;
    pub const UNK3: usize = 3;
    pub const UNK4: usize = 4;
    pub const UNK5: usize = 5;
    pub const UNK6: usize = 6;
    pub const UNK7: usize = 7;
    pub const UNK8: usize = 8;
    pub const UNK9: usize = 9;
    pub const UNK10: usize = 10;
    pub const UNK11: usize = 11;
    pub const UNK12: usize = 12;
    pub const UNK13: usize = 13;
    pub const UNK14: usize = 14;
    pub const UNK15: usize = 15;
    pub const UNK16: usize = 16;
    pub const UNK17: usize = 17;
    pub const UNK18: usize = 18;
    pub const UNK19: usize = 19;
    pub const UNK20: usize = 20;
    pub const UNK21: usize = 21;
    pub const UNK22: usize = 22;
    pub const UNK23: usize = 23;
    pub const UNK24: usize = 24;
    pub const UNK25: usize = 25;
    pub const UNK26: usize = 26;
    pub const UNK27: usize = 27;
    pub const UNK28: usize = 28;
    pub const UNK29: usize = 29;
    pub const UNK30: usize = 30;
    pub const UNK31: usize = 31;
    pub const UNK32: usize = 32;
    pub const UNK33: usize = 33;
    pub const UNK34: usize = 34;
    pub const UNK35: usize = 35;
    pub const UNK36: usize = 36;
    pub const UNK37: usize = 37;
    pub const UNK38: usize = 38;
    pub const UNK39: usize = 39;
    pub const UNK40: usize = 40;
    pub const UNK41: usize = 41;
    pub const UNK42: usize = 42;
    pub const UNK43: usize = 43;
    pub const UNK44: usize = 44;
    pub const UNK45: usize = 45;
    pub const UNK46: usize = 46;
    pub const UNK47: usize = 47;
    pub const UNK48: usize = 48;
    pub const UNK49: usize = 49;
    pub const UNK50: usize = 50;
    pub const UNK51: usize = 51;
    pub const UNK52: usize = 52;
    pub const UNK53: usize = 53;
    pub const UNK54: usize = 54;
    pub const UNK55: usize = 55;                         
    pub const UNK56: usize = 56;
    pub const UNK57: usize = 57;
    pub const UNK58: usize = 58;
    pub const UNK59: usize = 59;
    pub const UNK60: usize = 60;
    pub const UNK61: usize = 61;
    pub const UNK62: usize = 62;
    pub const UNK63: usize = 63;
    pub const UNK64: usize = 64;
    pub const UNK65: usize = 65;
    pub const UNK66: usize = 66;
    pub const UNK67: usize = 67;
    pub const UNK68: usize = 68;
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
    pub const UNK95: usize = 95;
    pub const UNK96: usize = 96;
    pub const UNK97: usize = 97;
    pub const UNK98: usize = 98;
    pub const UNK99: usize = 99;
    pub const UNK100: usize = 100;
    pub const UNK101: usize = 101;
}






