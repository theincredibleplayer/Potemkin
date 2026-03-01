use crate::moveset::*;



pub unsafe extern "C" fn potbusterwhiff_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    //GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    MotionModule::change_motion(module_accessor,Hash40::new("potemkin_buster_whiff"),0.0,1.0,false,0.0,false,false);
    
    fighter.clear_lua_stack();
    fighter.push_lua_stack(&mut L2CValue::I32(*FIGHTER_KINETIC_ENERGY_ID_MOTION));
    let mut lr = PostureModule::lr(fighter.module_accessor);
    fighter.push_lua_stack(&mut L2CValue::F32(lr));
    sv_kinetic_energy::set_chara_dir(fighter.lua_state_agent);

    fighter.sub_shift_status_main(L2CValue::Ptr(potbusterwhiff_main_loop as *const () as _))
}

unsafe fn potbusterwhiff_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }

    else if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
    }
    return L2CValue::I32(0)
}


pub unsafe extern "C" fn potbuster_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    MotionModule::change_motion(module_accessor,Hash40::new("potemkin_buster"),0.0,1.0,false,0.0,false,false);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    fighter.sub_shift_status_main(L2CValue::Ptr(potbuster_main_loop as *const () as _))
}

unsafe fn potbuster_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    //LinkModule::send_event_nodes_throw(fighter.module_accessor, Hash40::new("throw"), Hash40::new("invalid"), true, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO, *FIGHTER_STATUS_THROW_WORK_FLOAT_MOTION_RATE);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }

    else if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
    }
    return L2CValue::I32(0)
}

pub unsafe extern "C" fn potbuster_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    CameraModule::zoom_out(fighter.module_accessor, 0);
    lua_bind::FighterCutInManager::request_end(singletons::FighterCutInManager());
    if CatchModule::is_catch(fighter.module_accessor) {
        CatchModule::set_send_cut_event(fighter.module_accessor, true);
        CatchModule::catch_cut(fighter.module_accessor,false,false);
        println!("catch cut was ran");
    }
    return L2CValue::I32(0)
}








pub unsafe extern "C" fn potbusterwhiff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ARMOR_ON);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0); 
        macros::CATCH(agent, 0, Hash40::new("top"), 6.1, 0.0, 9.0, 7.0, Some(0.0), Some(9.0), Some(12.7), *FIGHTER_STATUS_KIND_THROWN, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 4.55, 0.0, 9.0, 5.45, Some(0.0), Some(9.0), Some(15.25), *FIGHTER_STATUS_KIND_THROWN, *COLLISION_SITUATION_MASK_A);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ARMOR_ON);
    }
}


pub unsafe extern "C" fn sound_potbusterwhiff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_busterwhiff_X".to_string(),3,false,false,1);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
    }
}




pub unsafe extern "C" fn potbuster(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 80, 140, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        WorkModule::on_flag(agent.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    }
    frame(agent.lua_state_agent, 128.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 30.0, 80, 80, 0, 50,20.0, 0.0, 20.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 158.0);
    if macros::is_excute(agent){
        WorkModule::off_flag(agent.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
        macros::CHECK_FINISH_CAMERA(agent, 14, 0);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 159.0);
    if macros::is_excute(agent) {
        let capture_id = LinkModule::get_node_object_id(agent.module_accessor, *LINK_NO_CAPTURE);
        if capture_id != 0x50000000 {
            AttackModule::hit_absolute_joint(agent.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,capture_id as u32,Hash40::new("throw"), 0, 0);
        }
    }
}

pub unsafe extern "C" fn effect_potbuster(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        for _ in 0..2 {
            macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_buster_fire"), Hash40::new("haver"), 0, -1, 3.5, 0.0, 0.0, 0.0, 1.7, true);
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_buster_fire"), Hash40::new("havel"), -0, -1, 3.5, 0.0, 0.0, 0.0, 1.7, true);
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        }
    }
    frame(agent.lua_state_agent, 64.0);
    if macros::is_excute(agent) {

        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 74.0);
    if macros::is_excute(agent) {
        if camera_checks(agent){
            let vec1 = smash_rs::cpp::simd::Vector2{vec: [0.0, 0.0]}; //
            let vec2 = smash_rs::cpp::simd::Vector2{vec: [0.0, 1.0]}; //both of these seem to be angle related but i cant put my finger on how exactly
            let vec3 = smash_rs::cpp::simd::Vector2{vec: [1.0 , 1.0]}; // effect width related
            let vec4 = smash_rs::cpp::simd::Vector2{vec: [1.0 , 1.0]}; // effect width related
            let speedlines = EffectModule::request_post_effect_line_parallel_2d(agent.module_accessor,Hash40::new("dead_area_out"),vec1,vec2,vec3,vec4,true,3.0/sv_information::slow_rate() /*duration*/,15.0/sv_information::slow_rate() /*fade out*/);
        }
    }
    frame(agent.lua_state_agent, 129.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("kamui_floor_crack_buster"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.5, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        macros::EFFECT(agent, Hash40::new("kamui_buster_land"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EffectModule::set_scale_last(agent.module_accessor, &Vector3f{x:2.0,y:1.0,z:2.0} );
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);

    }

    wait(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("kamui_buster_fire"), false, true);
    }
}

pub unsafe extern "C" fn sound_potbuster(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("vc_pot_busterwhiff_a"));
        macros::STOP_SE(agent, Hash40::new("vc_pot_busterwhiff_b"));
        macros::STOP_SE(agent, Hash40::new("vc_pot_busterwhiff_c"));
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_impact_catch_large"));
        let sound = sound_selector(agent,"vc_pot_potemkin_buster_X_X".to_string(),3,true,false,0);
        //macros::PLAY_SE(agent, Hash40::new_raw(sound));
        SoundModule::play_se(agent.module_accessor, Hash40::new_raw(sound), true, false, false, false, enSEType(0));
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_special_n03"));
    }
    frame(agent.lua_state_agent, 128.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_potemkin_buster_X_X".to_string(),3,true,true,2);
        SoundModule::play_se(agent.module_accessor, Hash40::new_raw(sound), true, false, false, false, enSEType(0));
        //macros::PLAY_SE(agent, Hash40::new_raw(sound));
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_landbreak_clay_l"));
    }
}

pub unsafe extern "C" fn expression_potbuster(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if camera_checks(agent){
            macros::CAM_ZOOM_OUT(agent);
            let scale = PostureModule::scale(agent.module_accessor);
            let lr = PostureModule::lr(agent.module_accessor);
            CameraModule::reset_all(agent.module_accessor);            
            macros::CAM_ZOOM_IN_arg6(
                agent,
                /*frames to zoom in totally*/5.0,
                /*according to calude the object id?*/0.0,
                /*the amt of zoom needed, higher the number lower the zoom*/1.1*scale,
                /*y_rot*/0.0, // up and down
                /*x_rot*/0.0, //left and right
                true
            );
            agent.clear_lua_stack();
            lua_args!(agent,
            *MA_MSC_CMD_CAMERA_CAM_OFFSET,
            /*x*/10.6619 * lr,
            /*y*/5.0,
            /*z*/0);
            smash::app::sv_module_access::camera(agent.lua_state_agent);
        }
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        if camera_checks(agent){
            macros::CAM_ZOOM_OUT(agent);
            macros::REQ_MOTION_CAMERA(agent, Hash40::new("d04potemkinbuster.nuanmb"), false);
        }
    }
    frame(agent.lua_state_agent, 103.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 150.0);
    if macros::is_excute(agent) {
        macros::CAM_ZOOM_OUT(agent);
    }
}
