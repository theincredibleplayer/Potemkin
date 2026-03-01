use crate::moveset::*;


#[skyline::hook(offset = 0xb5f380)]
pub unsafe extern "C" fn kamui_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash_rs::app::LinkEvent) -> u64 {
    if event.link_event_kind.0 == hash40("capture") {
        let capture_event : &mut smash_rs::app::LinkEventCapture = std::mem::transmute(event);
        let module_accessor = fighter.battle_object.module_accessor;
        //upair linkmodule
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            capture_event.result = true;
            capture_event.node = smash_rs::phx::Hash40::new("throw");
            StatusModule::change_status_request(module_accessor, FIGHTER_KAMUI_STATUS_KIND_AIR_THROW, false);
            return 0;
        }
        //potemkin buster linkmodule
        if StatusModule::status_kind(module_accessor) == FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER_WHIFF {
            capture_event.result = true;
            capture_event.node = smash_rs::phx::Hash40::new("throw");
            capture_event.ignore_distance = true;
            StatusModule::change_status_request(module_accessor, FIGHTER_KAMUI_STATUS_KIND_POTEMKIN_BUSTER, false);
            return 0;
        }
        //special_hi's link module
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            capture_event.result = true;
            capture_event.node = smash_rs::phx::Hash40::new("throw");
            //capture_event.is_motion_hi_lw = false;
            StatusModule::change_status_request(module_accessor, FIGHTER_KAMUI_STATUS_KIND_SPECIAL_HI_START, false);
            return 0;
        }
        //final smash's link module
        if StatusModule::status_kind(module_accessor) == FIGHTER_KAMUI_STATUS_KIND_FINAL_JUMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_FINAL {
            capture_event.result = true;
            //capture_event.constraint_offset.y = 20.0;
            capture_event.node = smash_rs::phx::Hash40::new("throw");
            

            WorkModule::off_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_JUMP);

            WorkModule::off_flag(module_accessor ,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_START);
            WorkModule::off_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_START);
            WorkModule::off_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_ATTACK_START);
            WorkModule::off_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_JUMP_START);

            PostureModule::set_pos(module_accessor,&Vector3f{x: 0.0 , y: 0.0, z: 50.0});
            StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_FINAL, false);      
            return 0;
        }
        return 1;
    }
    original!()(vtable, fighter, event)
}

#[skyline::hook(offset = 0x34146C0)]
pub unsafe extern "C" fn noping_the_eff(vtable:u64, weapon: *mut smash::app::Weapon, log: CollisionLog) -> u64 {
    *(weapon as *mut bool).add(0x90) = true; //Lets the weapon persist on hit
    if (AttackModule::is_infliction((*weapon).battle_object.module_accessor, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction((*weapon).battle_object.module_accessor, *COLLISION_KIND_MASK_SHIELD) || AttackModule::is_infliction((*weapon).battle_object.module_accessor, *COLLISION_KIND_MASK_ATTACK) )
    {
        if WorkModule::is_flag((*weapon).battle_object.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_DID_IT_HIT){
            WorkModule::off_flag((*weapon).battle_object.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_DID_IT_HIT);
            StatusModule::change_status_request_from_script((*weapon).battle_object.module_accessor, WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM, false);
        }
    }
    call_original!(vtable, weapon, log)
}


pub unsafe fn param_config_ig() {
    let all: Vec<i32> = vec![-1];
    param_config::update_float_2(*FIGHTER_KIND_KAMUI, all.clone(), (smash::hash40("param_private"), 0x19996b81b6, 1200.0));
}




#[skyline::hook(offset = 0x454150)]
pub unsafe extern "C" fn post_effect_params_again(effectmodule:u64, effhash: Hash40, vec1: smash_rs::cpp::simd::Vector2, vec2: smash_rs::cpp::simd::Vector2, vec3: smash_rs::cpp::simd::Vector2, vec4: smash_rs::cpp::simd::Vector2, arg:bool, float1:f32, float2:f32) -> u64 {
    println!("
    effect hash = {:#x}
    vec1 x = {}
    vec1 y = {}
    vec2 x = {}
    vec2 y = {}
    vec3 x = {}
    vec3 y = {}
    vec4 x = {}
    vec4 y = {}
    arg = {}
    float1 = {}
    float2 = {}"
    ,effhash.hash
    ,vec1.vec[0]
    ,vec1.vec[1]
    ,vec2.vec[0]
    ,vec2.vec[1]
    ,vec3.vec[0]
    ,vec3.vec[1]
    ,vec4.vec[0]
    ,vec4.vec[1]
    ,arg
    ,float1
    ,float2
    );


    call_original!(effectmodule, effhash, vec1, vec2, vec3, vec4, arg, float1, float2)
}


#[skyline::hook(offset = 0x453a30)]
pub unsafe extern "C" fn post_circle_effect_params_again(effectmodule:u64, effhash: Hash40, bonehash: Hash40, vec1: smash_rs::cpp::simd::Vector2, vec2: smash_rs::cpp::simd::Vector3, arg:bool, float1:f32, float2:f32) -> u64 {
    println!("
    vec1 x = {}
    vec1 y = {}
    vec2 x = {}
    vec2 y = {}
    vec2 z = {}
    arg = {}
    float1 = {}
    float2 = {}"
    ,vec1.vec[0]
    ,vec1.vec[1]
    ,vec2.vec[0]
    ,vec2.vec[1]
    ,vec2.vec[2]
    ,arg
    ,float1
    ,float2
    );


    call_original!(effectmodule, effhash, bonehash, vec1, vec2, arg, float1, float2)
}


#[skyline::hook(offset = 0x69a728, inline)]
unsafe extern "C" fn autoturn_situation_check_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = ctx.registers[21].x() as *mut BattleObjectModuleAccessor;
    let fighter_kind = utility::get_kind(&mut *(module_accessor));
    if fighter_kind == *FIGHTER_KIND_KAMUI {
        ctx.registers[0].set_x(0);
    }
}