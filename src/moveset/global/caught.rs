use std::ops::Rem;

use super::*;



pub unsafe extern "C" fn game_catchedpotspecialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //MotionModule::set_flip(agent.module_accessor,true,true,false);
        println!("hiiiiiiiiiiiiiiiiii");
    }
}



pub unsafe extern "C" fn caught_final_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let parent_id = LinkModule::get_parent_id(fighter.module_accessor,*LINK_NO_CAPTURE,true);
    let parent_boma = sv_battle_object::module_accessor(parent_id as u32);
    let motion_share = WorkModule::get_param_int(fighter.module_accessor,0xcad2ee25e,0xc07d88ea0);
    let mut motion = hash40("catched_pot_final_scene_01");
    let height = WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0);
    let egg_size = WorkModule::get_param_float(fighter.module_accessor, hash40("egg_collision_size"), 0);
    let fighter_kind = utility::get_kind(&mut (*fighter.module_accessor));
    let parent_lr = PostureModule::lr(parent_boma);

    let mut x_offset = 22.0 - height; // this is for left and right
    let mut z_offset = -(7.2 - egg_size)/2.0; // this is for in and out
    let mut y_offset = -(7.2 - egg_size)/3.0 + 0.25; // this is for up and down

    physics!(fighter,*MA_MSC_CMD_PHYSICS_PH_SWING_STOP, true);

    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);

    let head_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
    let hip_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
    let position = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
    ModelModule::joint_global_position(fighter.module_accessor, Hash40::new("hip"), hip_pos, true);
    ModelModule::joint_global_position(fighter.module_accessor, Hash40::new("head"), head_pos, true);
    position.x = hip_pos.x - head_pos.x;
    //ganon's offset is 8.321199
    x_offset = (-7.0 - position.x)/3.0 + 3.5;



    if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
        motion = FighterMotionModuleImpl::add_body_type_hash(
            fighter.module_accessor,
            Hash40::new_raw(motion),
            *BODY_TYPE_MOTION_DX
        );
        x_offset = 9.0 - height;
        y_offset = -1.0;
        z_offset = -1.0;
    }
    else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
        motion = FighterMotionModuleImpl::add_body_type_hash(
        fighter.module_accessor,
            Hash40::new_raw(motion),
            *BODY_TYPE_MOTION_GIRL
        );
        x_offset = 17.0 - height;
        y_offset = 0.5;
        z_offset = -1.0;
    }
    else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_BIG {
        motion = FighterMotionModuleImpl::add_body_type_hash(
        fighter.module_accessor,
            Hash40::new_raw(motion),
            *BODY_TYPE_MOTION_BIG
        );
        y_offset = 0.0;
        z_offset = 0.0;
        x_offset = 19.0 - height;
    }
    if fighter_kind == *FIGHTER_KIND_YOSHI {
        x_offset = -4.0;
    }
    if fighter_kind == *FIGHTER_KIND_PACKUN {
        x_offset = -7.0;
    }
    if fighter_kind == *FIGHTER_KIND_ROBOT {
        x_offset = -6.0;
    }
    if fighter_kind == *FIGHTER_KIND_PIKACHU {
        x_offset = 4.0;
    }
    if  [*FIGHTER_KIND_MIIFIGHTER,
        *FIGHTER_KIND_MIIGUNNER,
        *FIGHTER_KIND_MIISWORDSMAN,
        *FIGHTER_KIND_MIIENEMYF,
        *FIGHTER_KIND_MIIENEMYG,
        *FIGHTER_KIND_MIIENEMYS].contains(&fighter_kind) {
        x_offset = -2.0;
        z_offset = -2.5;
    }
    if is_potemkin(fighter.module_accessor) {
        y_offset = 2.5;
        z_offset = 0.0;
        x_offset = 0.5;
    }


    WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    fighter.status_Thrown();
    let constraint_flags = LinkModule::get_model_constraint_flag(fighter.module_accessor);
    LinkModule::set_model_constraint_flag(fighter.module_accessor,(*CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_DISABLE_RESTRICT_Z | constraint_flags as i32) as u32);
    LinkModule::set_constraint_translate_offset(fighter.module_accessor, &Vector3f{ x: x_offset, y: y_offset, z: z_offset });

    let z_scale = ModelModule::scale_z(fighter.module_accessor);
    WorkModule::set_float(fighter.module_accessor,z_scale,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_OPPONENT_Z_SCALE);
    ModelModule::set_scale_z(fighter.module_accessor,1.0);
    
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
    if parent_lr < 0.0{
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x23c33f3bdc));
    }
    
    fighter.sub_shift_status_main(L2CValue::Ptr(caught_main_loop as *const () as _))
}


pub unsafe extern "C" fn caught_final_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let z_scale = WorkModule::get_float(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_OPPONENT_Z_SCALE);
    ModelModule::set_scale_z(fighter.module_accessor,z_scale);
    physics!(fighter,*MA_MSC_CMD_PHYSICS_PH_SWING_STOP, false);
    return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_THROWN)(fighter)
}


pub unsafe extern "C" fn caught_specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_share = WorkModule::get_param_int(fighter.module_accessor,0xcad2ee25e,0xc07d88ea0);
    let mut motion = hash40("catched_pot_special_hi");
    let mut share_type = 0;
    let parent_id = LinkModule::get_parent_id(fighter.module_accessor,*LINK_NO_CAPTURE,true);
    let parent_boma = sv_battle_object::module_accessor(parent_id as u32);
    let parent_lr = PostureModule::lr(parent_boma);
    PostureModule::set_lr(fighter.module_accessor, -parent_lr);
    PostureModule::update_rot_y_lr(fighter.module_accessor);

    if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
        motion = FighterMotionModuleImpl::add_body_type_hash(
            fighter.module_accessor,
            Hash40::new_raw(motion),
            *BODY_TYPE_MOTION_DX
        );
    }
    else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
        motion = FighterMotionModuleImpl::add_body_type_hash(
        fighter.module_accessor,
            Hash40::new_raw(motion),
            *BODY_TYPE_MOTION_GIRL
        );
    }
    else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_BIG {
        motion = FighterMotionModuleImpl::add_body_type_hash(
        fighter.module_accessor,
            Hash40::new_raw(motion),
            *BODY_TYPE_MOTION_BIG
        );
    }
    //let constraint_flags = LinkModule::get_model_constraint_flag(fighter.module_accessor);
    //LinkModule::set_model_constraint_flag(fighter.module_accessor,(*CONSTRAINT_FLAG_DISABLE_RESTRICT_Z | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
    LinkModule::set_model_constraint_pos_ort(fighter.module_accessor, *LINK_NO_CAPTURE, Hash40::new("rot"), Hash40::new("throw"), (*CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_ORIENTATION) as u32, true);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x23c33f3bdc));
    //MotionModule::set_link_flip_target_joint(fighter.module_accessor,true);

    fighter.sub_shift_status_main(L2CValue::Ptr(caught_main_loop as *const () as _))
}


pub unsafe extern "C" fn caught_buster_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    let parent_id = LinkModule::get_parent_id(fighter.module_accessor,*LINK_NO_CAPTURE,true);
    let parent_boma = sv_battle_object::module_accessor(parent_id as u32);
    let motion_share = WorkModule::get_param_int(fighter.module_accessor,0xcad2ee25e,0xc07d88ea0);
    let mut motion = hash40("catched_potemkin_buster");
    let height = WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0);
    let egg_size = WorkModule::get_param_float(fighter.module_accessor, hash40("egg_collision_size"), 0);
    let fighter_kind = utility::get_kind(&mut (*fighter.module_accessor));
    let parent_lr = PostureModule::lr(parent_boma);
    let mut x_offset = 22.0 - height;
    let mut z_offset = (7.2 -egg_size)/4.0;
    let mut y_offset = 0.0;

    PostureModule::set_lr(fighter.module_accessor, -parent_lr);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);

    let head_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
    let hip_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
    let position = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
    ModelModule::joint_global_position(fighter.module_accessor, Hash40::new("hip"), hip_pos, true);
    ModelModule::joint_global_position(fighter.module_accessor, Hash40::new("head"), head_pos, true);
    position.x = hip_pos.y - head_pos.y;

    x_offset = (-4.0 - position.x)/3.0;



    if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
        motion = FighterMotionModuleImpl::add_body_type_hash(
            fighter.module_accessor,
            Hash40::new_raw(motion),
            *BODY_TYPE_MOTION_DX
        );
        x_offset = 9.0 - height;
        y_offset = -1.0;
        z_offset = -1.0;
    }
    else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
        motion = FighterMotionModuleImpl::add_body_type_hash(
        fighter.module_accessor,
            Hash40::new_raw(motion),
            *BODY_TYPE_MOTION_GIRL
        );
        x_offset = 17.0 - height;
        y_offset = 0.5;
        z_offset = -1.0;
    }
    else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_BIG {
        motion = FighterMotionModuleImpl::add_body_type_hash(
        fighter.module_accessor,
            Hash40::new_raw(motion),
            *BODY_TYPE_MOTION_BIG
        );
        y_offset = 0.0;
        z_offset = 0.0;
        x_offset = 19.0 - height;
    }
    if fighter_kind == *FIGHTER_KIND_YOSHI {
        x_offset = -4.0;
    }
    if fighter_kind == *FIGHTER_KIND_PACKUN {
        x_offset = -7.0;
    }
    if fighter_kind == *FIGHTER_KIND_ROBOT {
        x_offset = -6.0;
    }
    if  [*FIGHTER_KIND_MIIFIGHTER,
        *FIGHTER_KIND_MIIGUNNER,
        *FIGHTER_KIND_MIISWORDSMAN,
        *FIGHTER_KIND_MIIENEMYF,
        *FIGHTER_KIND_MIIENEMYG,
        *FIGHTER_KIND_MIIENEMYS].contains(&fighter_kind) {
        x_offset = -2.0;
        z_offset = -2.5;
    }
    if is_potemkin(fighter.module_accessor) {
        y_offset = 2.5;
        z_offset = 0.0;
        x_offset = 0.5;
    }
    //if WorkModule::is_flag(parent_boma,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_ENABLE_CAMERA){
    //    z_offset = 0.0;
    //}
    let constraint_flags = LinkModule::get_model_constraint_flag(fighter.module_accessor);
    LinkModule::set_model_constraint_flag(fighter.module_accessor,(*CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
    LinkModule::set_constraint_translate_offset(fighter.module_accessor, &Vector3f{ x: x_offset, y: y_offset, z: z_offset });
    //fighter.flip_have_item_capture();
    if parent_lr < 0.0{
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x23c33f3bdc));
    }
    //fighter.clear_lua_stack();
    //lua_args!(fighter, Hash40::new_raw(0x23c33f3bdc));
    //sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    //fighter.pop_lua_stack(1);
    //fighter.clear_lua_stack();
    //fighter.push_lua_stack(agent);
    //fighter.push_lua_stack(&mut L2CValue::new_int(0x23c33f3bdc));
    //sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    //fighter.pop_lua_stack(1);     

    fighter.sub_shift_status_main(L2CValue::Ptr(caught_main_loop as *const () as _))
}


pub unsafe extern "C" fn caught_airthrow_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_share = WorkModule::get_param_int(fighter.module_accessor,0xcad2ee25e,0xc07d88ea0);
    let mut motion = hash40("catched_pot_special_hi");
    let mut share_type = 0;
    if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
        motion = FighterMotionModuleImpl::add_body_type_hash(
            fighter.module_accessor,
            Hash40::new_raw(motion),
            *BODY_TYPE_MOTION_DX
        );
    }
    else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
        motion = FighterMotionModuleImpl::add_body_type_hash(
        fighter.module_accessor,
            Hash40::new_raw(motion),
            *BODY_TYPE_MOTION_GIRL
        );
    }
    else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_BIG {
        motion = FighterMotionModuleImpl::add_body_type_hash(
        fighter.module_accessor,
            Hash40::new_raw(motion),
            *BODY_TYPE_MOTION_BIG
        );
    }
    let constraint_flags = LinkModule::get_model_constraint_flag(fighter.module_accessor);
    LinkModule::set_model_constraint_flag(fighter.module_accessor,(*CONSTRAINT_FLAG_DISABLE_RESTRICT_Z | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(caught_main_loop as *const () as _))
}



pub unsafe extern "C" fn caught_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings
    (fighter.module_accessor,
    SituationKind(*SITUATION_KIND_NONE),
    *FIGHTER_KINETIC_TYPE_UNIQ,
    *GROUND_CORRECT_KIND_NONE as u32,
    GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
    false,
    *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
    *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
    *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
    0);
    FighterStatusModuleImpl::set_fighter_status_data
    (fighter.module_accessor,
    false,
    *FIGHTER_TREADED_KIND_NO_REAC,
    false,
    true,
    false,
    (0) as u64,
    (*FIGHTER_STATUS_ATTR_NO_DROP_ITEM | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE) as u32,
    (0) as u32,
    0);
    return L2CValue::I32(0)
    //return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_THROWN)(fighter)
}

pub unsafe extern "C" fn caught_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("frame/4 = {}",MotionModule::frame(fighter.module_accessor).rem(4.0) as i32);
    if MotionModule::frame(fighter.module_accessor).rem(4.0) == 0.0{
        //physics!(fighter,*MA_MSC_CMD_PHYSICS_PH_SWING_STOP, false);
        PhysicsModule::set_swing_rate(fighter.module_accessor, 1.0);
        //physics!(fighter,*MA_MSC_CMD_PHYSICS_PH_SWING_STOP, true);
    }else{
        //physics!(fighter,*MA_MSC_CMD_PHYSICS_PH_SWING_STOP, true);
        PhysicsModule::set_swing_rate(fighter.module_accessor, 0.0);
    }
    return 0.into();
}

pub unsafe extern "C" fn caught_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_THROWN)(fighter)
}

pub unsafe extern "C" fn caught_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(Exit, fighter, *FIGHTER_STATUS_KIND_THROWN)(fighter)
}

pub unsafe extern "C" fn caught_checkdmg(fighter: &mut L2CFighterCommon,param2: &L2CValue) -> L2CValue {
    return smashline::original_status(CheckDamage, fighter, *FIGHTER_STATUS_KIND_THROWN)(fighter,param2)
}

pub unsafe extern "C" fn caught_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(Exec, fighter, *FIGHTER_STATUS_KIND_THROWN)(fighter)
}

pub unsafe extern "C" fn caught_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_THROWN)(fighter)
}




pub fn install(agent: &mut Agent) {
    //agent.game_acmd("game_catchedpotspecialhi", game_catchedpotspecialhi, Priority::Default);

    agent.status(Pre,            FIGHTER_STATUS_KIND_POT_SPECIALHI_CAUGHT, caught_pre);
    agent.status(Main,           FIGHTER_STATUS_KIND_POT_SPECIALHI_CAUGHT, caught_specialhi_main);
    agent.status(Exit,           FIGHTER_STATUS_KIND_POT_SPECIALHI_CAUGHT, caught_exit);
    agent.status(Init,           FIGHTER_STATUS_KIND_POT_SPECIALHI_CAUGHT, caught_init);
    agent.status(Exec,           FIGHTER_STATUS_KIND_POT_SPECIALHI_CAUGHT, caught_exec);
    agent.status(End,            FIGHTER_STATUS_KIND_POT_SPECIALHI_CAUGHT, caught_end);
    agent.status(CheckDamage,    FIGHTER_STATUS_KIND_POT_SPECIALHI_CAUGHT, caught_checkdmg);

    agent.status(Pre,            FIGHTER_STATUS_KIND_POT_FINAL_CAUGHT, caught_pre);
    agent.status(Main,           FIGHTER_STATUS_KIND_POT_FINAL_CAUGHT, caught_final_main);
    agent.status(Exit,           FIGHTER_STATUS_KIND_POT_FINAL_CAUGHT, caught_exit);
    agent.status(Init,           FIGHTER_STATUS_KIND_POT_FINAL_CAUGHT, caught_init);
    agent.status(Exec,           FIGHTER_STATUS_KIND_POT_FINAL_CAUGHT, caught_exec);
    agent.status(End,            FIGHTER_STATUS_KIND_POT_FINAL_CAUGHT, caught_final_end);
    agent.status(CheckDamage,    FIGHTER_STATUS_KIND_POT_FINAL_CAUGHT, caught_checkdmg);

    agent.status(Pre,            FIGHTER_STATUS_KIND_POT_BUSTER_CAUGHT, caught_pre);
    agent.status(Main,           FIGHTER_STATUS_KIND_POT_BUSTER_CAUGHT, caught_buster_main);
    agent.status(Exit,           FIGHTER_STATUS_KIND_POT_BUSTER_CAUGHT, caught_exit);
    agent.status(Init,           FIGHTER_STATUS_KIND_POT_BUSTER_CAUGHT, caught_init);
    agent.status(Exec,           FIGHTER_STATUS_KIND_POT_BUSTER_CAUGHT, caught_exec);
    agent.status(End,            FIGHTER_STATUS_KIND_POT_BUSTER_CAUGHT, caught_end);
    agent.status(CheckDamage,    FIGHTER_STATUS_KIND_POT_BUSTER_CAUGHT, caught_checkdmg);

    agent.status(Pre,            FIGHTER_STATUS_KIND_POT_AIR_THROW_CAUGHT, caught_pre);
    agent.status(Main,           FIGHTER_STATUS_KIND_POT_AIR_THROW_CAUGHT, caught_airthrow_main);
    agent.status(Exit,           FIGHTER_STATUS_KIND_POT_AIR_THROW_CAUGHT, caught_exit);
    agent.status(Init,           FIGHTER_STATUS_KIND_POT_AIR_THROW_CAUGHT, caught_init);
    agent.status(Exec,           FIGHTER_STATUS_KIND_POT_AIR_THROW_CAUGHT, caught_exec);
    agent.status(End,            FIGHTER_STATUS_KIND_POT_AIR_THROW_CAUGHT, caught_end);
    agent.status(CheckDamage,    FIGHTER_STATUS_KIND_POT_AIR_THROW_CAUGHT, caught_checkdmg);


}