use crate::moveset::*;





unsafe fn motion_setup(fighter: &mut L2CFighterCommon, situation_change: bool) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if situation_change == false {
            MotionModule::change_motion(fighter.module_accessor,Hash40::new("final"),0.0,1.0,false,0.0,false,false);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_GROUND,0.0,0.0,0.0,0.0,0.0);
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        } else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("final"),-1.0,1.0,0.0,false,false);
        } 
    } else {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if situation_change == false {
            MotionModule::change_motion(fighter.module_accessor,Hash40::new("final"),0.0,1.0,false,0.0,false,false);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_GROUND,0.0,0.0,0.0,0.0,0.0);
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        } else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("final"),-1.0,1.0,0.0,false,false);
        }
    }
}



pub unsafe extern "C" fn final_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_START) {
        ArticleModule::generate_article(fighter.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,false,-1);
        potemkin_mirror_handler(fighter.module_accessor);
        ArticleModule::change_motion(fighter.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("final"),false,-1.0);



        fighter.clear_lua_stack();
        fighter.push_lua_stack(&mut L2CValue::new_int(0x201bc9217c));
        sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);     
        WorkModule::set_float(fighter.module_accessor, 0.0,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_TIMER);
        AreaModule::set_whole(fighter.module_accessor,false);
        KineticModule::clear_speed_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DISABLE_OUTSIDE_ENERGY);
        if ((fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND) != false) {
            MotionModule::change_motion(fighter.module_accessor,Hash40::new("final"),0.0,1.0,false,0.0,false,false); //should be air
            KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_AIR_ESCAPE);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor,Hash40::new("final"),0.0,1.0,false,0.0,false,false); //ground
            KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_ZOOM_START);
    } else if WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_START){
        motion_setup(fighter,false);
        let module_accessor = fighter.module_accessor;
        MotionModule::change_motion(fighter.module_accessor,Hash40::new("heavenly_dash_loop"),0.0,1.0,false,0.0,false,false);

        //fighter.clear_lua_stack();
        //lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,30.0,0.0);
        //sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_NONE,  13.0 * PostureModule::lr(module_accessor),0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,0.5,0.0);
        sv_kinetic_energy::set_brake(fighter.lua_state_agent);
    
    } else if WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_ATTACK_START){
        let module_accessor = fighter.module_accessor;
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::clear_speed_all(fighter.module_accessor);
        KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,*ENERGY_CONTROLLER_RESET_TYPE_FREE,0.0,0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        MotionModule::change_motion(fighter.module_accessor,Hash40::new("heavenly_dash_attack"),0.0,1.0,false,0.0,false,false);
        WorkModule::set_float(fighter.module_accessor,0.0,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_TIMER);
    } else if WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_JUMP_START){
        KineticModule::clear_speed_all(fighter.module_accessor);
        MotionModule::change_motion(fighter.module_accessor,Hash40::new("heavenly_start"),0.0,1.0,false,0.0,false,false);    
    } else {
        let z_scale = ModelModule::scale_z(fighter.module_accessor);
        WorkModule::set_float(fighter.module_accessor,z_scale,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_POTEMKIN_Z_SCALE);
        ModelModule::set_scale_z(fighter.module_accessor,1.0);
        KineticModule::clear_speed_all(fighter.module_accessor);
        let module_accessor = fighter.module_accessor;
        AreaModule::set_whole(fighter.module_accessor, false);

        HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
        AreaModule::set_whole(fighter.module_accessor, false);
        WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
        macros::REQ_FINAL_START_CAMERA_arg3(fighter,Hash40::new("d04finalvisualscene01.nuanmb"), false, false);
        MotionModule::change_motion(fighter.module_accessor,Hash40::new("fake_final_visual_scene_01"),0.0,1.0,false,0.0,false,false);

        ArticleModule::generate_article(fighter.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND,false,-1);
        potemkin_mirror_handler(fighter.module_accessor);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, Hash40::new("fake_final_visual_scene_01"), false, -1.0);
        let plane = ArticleModule::get_article(fighter.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND);
        let plane_boma = *(plane as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
        LinkModule::remove_model_constraint(plane_boma,true);
        LinkModule::set_model_constraint_pos_ort(plane_boma,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("attach"),Hash40::new("top"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32 | *CONSTRAINT_FLAG_OFFSET_ROT as u32,true);
        ModelModule::set_scale_z(plane_boma,1.0);

        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, -1);
        potemkin_mirror_handler(fighter.module_accessor);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40::new("final_visual_scene_01"), false, -1.0);
        let article = ArticleModule::get_article(fighter.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON);
        let article_boma = *(article as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
        ModelModule::set_scale_z(article_boma,1.0);


    }
    fighter.sub_shift_status_main(L2CValue::Ptr(final_main_loop as *const () as _))

}


pub unsafe extern "C" fn final_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_START) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        } else {
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }


        if WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_CHANGE_TO_FS_DASH) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_CHANGE_TO_FS_DASH);
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_START);
            WorkModule::on_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_START);
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_ATTACK_START);
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_JUMP_START);

            fighter.change_status(FIGHTER_STATUS_KIND_FINAL.into(),true.into());
            return L2CValue::I32(0)
        }


        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FINAL_JUMP_END.into(),false.into());
            } else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
            }
        } else {
            if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_SUCCESS) {
                fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_READY.into(),false.into());
                return L2CValue::I32(0)
            }
        }
        return L2CValue::I32(0) 
    } else if WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_START){
        let mut current_speed = WorkModule::get_float(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_ACCEL);
        let module_accessor = fighter.module_accessor;
        let timer = WorkModule::get_float(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_TIMER);
        let speed_x = KineticModule::get_sum_speed_x(module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        
    
        WorkModule::set_float(fighter.module_accessor, timer + 1.0 ,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_TIMER);
    
    
    
    

        if MotionModule::is_end(fighter.module_accessor) {
            MotionModule::change_motion(fighter.module_accessor,Hash40::new("heavenly_dash_loop"),0.0,1.0,false,0.0,false,false);
        }
        if (ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) == true)
        && (WorkModule::get_float(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_TIMER) >= 20.0) {
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_START);
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_START);
            WorkModule::on_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_ATTACK_START);
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_JUMP_START);
            fighter.change_status(FIGHTER_STATUS_KIND_FINAL.into(),false.into());
            return L2CValue::I32(0)
        }
        if ((WorkModule::get_float(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_TIMER) >= 60.0) || (speed_x == 0.0) ) {
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_START);
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_START);
            WorkModule::on_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_ATTACK_START);
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_JUMP_START);
            fighter.change_status(FIGHTER_STATUS_KIND_FINAL.into(),false.into());
            return L2CValue::I32(0)
        }
        if AttackModule::is_infliction(fighter.module_accessor,*COLLISION_KIND_MASK_HIT)
        || AttackModule::is_infliction(fighter.module_accessor,*COLLISION_KIND_MASK_SHIELD) {
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_START);
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_START);
            WorkModule::on_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_ATTACK_START);
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_JUMP_START);
            fighter.change_status(FIGHTER_STATUS_KIND_FINAL.into(),false.into());
            return L2CValue::I32(0)
        }
        return L2CValue::I32(0)
    } else if WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_ATTACK_START){
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT){
            WorkModule::on_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_GOTTEM)
        }
        if WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_GOTTEM)
        && WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SWITCH_TO_FS_JUMP){
        KineticModule::clear_speed_all(fighter.module_accessor);
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_HPB_GOTTEM);

            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_START);
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_START);
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_ATTACK_START);
            WorkModule::on_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_JUMP_START);

            fighter.change_status(FIGHTER_STATUS_KIND_FINAL.into(),false.into());
        }
        if MotionModule::is_end(fighter.module_accessor) {
            KineticModule::clear_speed_all(fighter.module_accessor);
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
        }
        return L2CValue::I32(0)
    } else if WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_JUMP_START){
        let module_accessor = fighter.module_accessor;

        if WorkModule::is_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_JUMP)  {
            KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
            KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
            KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            let speed_x = 3.0 + WorkModule::get_float(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_X_JUMP_SPEED)*2.0;
            let speed_limit_x = 10.0 + WorkModule::get_float(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FINAL_X_JUMP_SPEED)*4.0;

            println!("speed_x = {}",speed_x);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,10.0);
            sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
            sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,speed_limit_x);
            sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
            sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_STOP_RESET_TYPE_AIR, 0.0,2.0,0.0,0.0,0.0);
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR, speed_x * PostureModule::lr(module_accessor),0.0,0.0,0.0,0.0);
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0, 0.0);
            sv_kinetic_energy::set_accel(fighter.lua_state_agent);
        }
        if MotionModule::is_end(module_accessor) {
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_START);
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_START);
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_ATTACK_START);
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_JUMP_START);
            WorkModule::off_flag(module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_JUMP);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
        return L2CValue::I32(0)
    
    } else {
        let module_accessor = fighter.module_accessor;
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
        AreaModule::set_whole(fighter.module_accessor, false);
        UiManager::sync_jetpack(MotionModule::frame(fighter.module_accessor));
        UiManager::set_jetpack_lr(PostureModule::lr(fighter.module_accessor));
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);

        if MotionModule::is_end(fighter.module_accessor) {
            WorkModule::on_flag(fighter.module_accessor,*FIGHTER_KAMUI_STATUS_WORK_ID_FLAG_FINAL_SUCCESS);
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_READY.into(),false.into());
        }
        return L2CValue::I32(0)
    }
}


pub unsafe extern "C" fn final_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));

    if StatusModule::status_kind_next(module_accessor) != FIGHTER_STATUS_KIND_FINAL {
        println!("final exit og was ran");
        WorkModule::off_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SWITCH_TO_FS_JUMP);
        WorkModule::on_flag(fighter.module_accessor ,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_START);
        WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_START);
        WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DASH_ATTACK_START);
        WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_JUMP_START);
        WorkModule::off_flag(fighter.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_FINAL_CANCELLED_INTO);

        return smashline::original_status(Exit, fighter, *FIGHTER_STATUS_KIND_FINAL)(fighter);
    }
    return L2CValue::I32(0)
}









pub unsafe extern "C" fn finalsceneexit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    ArticleModule::remove_exist(module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    fighter.set_situation(SITUATION_KIND_GROUND.into());
    GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    grabbed_anim_selector(fighter,"catched_pot_final_scene_03", 1.0);
    WorkModule::on_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    //CAM_ZOOM_OUT_FINAL(fighter);
    let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
    let enemy = sv_battle_object::module_accessor(capture_id as u32);
    WorkModule::off_flag(enemy,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);




    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,15.0);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.25, -0.25);
    sv_kinetic_energy::set_accel(fighter.lua_state_agent);
    WorkModule::set_float(module_accessor, 3.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHECK_DEAD_OFFSET_Y);
    let check_distance:f32 = 500.0;
    WorkModule::set_float(module_accessor, check_distance, 0x1000008);
    WorkModule::set_float(module_accessor, 0.0, 0x1000007);

    PostureModule::set_pos(module_accessor,&Vector3f{x: 0.0 , y: check_distance, z: 0.0});

    let pos2f = Vector2f{x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)};
    let check_pos = Vector2f{x: 0.0, y: check_distance};
    let mut hit_pos = Vector2f{x: 0.0, y: 0.0};
    let distance_to_ground = GroundModule::get_distance_to_floor(fighter.module_accessor, PostureModule::pos(fighter.module_accessor), check_distance*2.0, true);
    let cliff_pos = GroundModule::hang_cliff_pos_3f(fighter.module_accessor);
    
    if distance_to_ground != -1.0 {
        macros::REQ_FINAL_START_CAMERA_arg3(fighter,Hash40::new("d04finalvisualscene03.nuanmb"), false, false);
        let new_pos_y = check_distance - distance_to_ground;
        PostureModule::set_pos(module_accessor,&Vector3f{x: 0.0 , y: new_pos_y, z: 0.0});
    }else{
        PostureModule::set_pos(module_accessor,&Vector3f{x: 0.0 , y: 40.0, z: 0.0});
        WorkModule::on_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DEATH_IMMINENT);
    }
    let pos = Vector3f{x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor), z: PostureModule::pos_z(fighter.module_accessor)};
    //let checkbool = GroundModule::ray_check_hit_pos(fighter.module_accessor,&pos2f,&check_pos,&mut hit_pos,true);
    println!("distance to ground: {}\nlast pos x: {}\nlast pos y: {}\nnow pos x: {}\nnow pos y:{}\ncliff pos x: {}\ncliff pos y:{}", distance_to_ground, pos2f.x, pos2f.y, pos.x, pos.y, cliff_pos.x,cliff_pos.y);

    //println!("hit_pos_x: {}\nhit_pos_y: {}\ncheckbool: {}", hit_pos.x, hit_pos.y, checkbool);

    return smashline::original_status(Main, fighter, *FIGHTER_KAMUI_STATUS_KIND_FINAL_END)(fighter);
}

pub unsafe extern "C" fn finalsceneexit_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    WorkModule::off_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_FINAL_DEATH_IMMINENT);
    return smashline::original_status(End, fighter, *FIGHTER_KAMUI_STATUS_KIND_FINAL_END)(fighter);
}


pub unsafe extern "C" fn finalsceneenter_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let check_distance:f32 = 500.0;
    WorkModule::set_float(fighter.module_accessor, check_distance, 0x1000008);
    WorkModule::set_float(fighter.module_accessor, 0.0, 0x1000007);
    FighterManager::exit_finalbg(singletons::FighterManager());
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, false, -1);
    potemkin_mirror_handler(fighter.module_accessor);
    ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, Hash40::new("final_visual_scene_01"), false, -1.0);
    let article = ArticleModule::get_article(fighter.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND);
    let article_boma = *(article as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
    LinkModule::remove_model_constraint(article_boma,true);
    LinkModule::set_model_constraint_pos_ort(article_boma,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("top"),Hash40::new("top"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32 | *CONSTRAINT_FLAG_OFFSET_ROT as u32,true);
    ModelModule::set_scale_z(article_boma,1.0);

    return smashline::original_status(Main, fighter, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ENTRY)(fighter);
}
pub unsafe extern "C" fn finalsceneattack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    WorkModule::on_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    
    return smashline::original_status(Main, fighter, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK)(fighter);
}





pub unsafe extern "C" fn vissceneentry_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK);
    return 1.into()
}




pub unsafe extern "C" fn finalvisscene2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    grabbed_anim_selector(fighter,"catched_pot_final_scene_02", 1.0);
    return smashline::original_status(Main, fighter, *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK)(fighter);
}



pub unsafe extern "C" fn vissceneentry_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK.into(),false.into());
    return L2CValue::I32(0)
}


pub unsafe extern "C" fn vissceneattackwait_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_status(FIGHTER_KAMUI_STATUS_KIND_FINAL_END.into(),false.into());
    return L2CValue::I32(0)
}





pub fn install(agent: &mut Agent) {
    agent.status(Main,  *FIGHTER_STATUS_KIND_FINAL,                              final_main);
    agent.status(Exit,  *FIGHTER_STATUS_KIND_FINAL,                              final_exit);

    agent.status(Main,  *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ENTRY,            finalsceneenter_main);

    agent.status(Main,  *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK,           finalsceneattack_main);

    agent.status(Main,  *FIGHTER_KAMUI_STATUS_KIND_FINAL_END,                    finalsceneexit_main);
    agent.status(End,   *FIGHTER_KAMUI_STATUS_KIND_FINAL_END,                    finalsceneexit_end);

    agent.status(Main,  *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK,           finalvisscene2_main);
    
    agent.status(Exec,  *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ENTRY,            vissceneentry_exec);
    agent.status(Exec,  *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK_WAIT,      vissceneattackwait_exec);
}