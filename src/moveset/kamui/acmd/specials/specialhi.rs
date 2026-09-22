use super::*;


unsafe extern "C" fn special_hi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,false,-1);
        potemkin_mirror_handler(agent.module_accessor);
        ArticleModule::change_motion(agent.module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,Hash40::new("special_hi"),false,-1.0);    
    }
    sv_animcmd::frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);

        macros::CATCH(agent, 0, Hash40::new("top"), 10.0 ,0.0,36.0,23.0, None, None, None, FIGHTER_STATUS_KIND_POT_SPECIALHI_CAUGHT, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(agent, 2, Hash40::new("top"), 5.0 ,0.0,21.6,8.0, Some(0.0),Some(30.7),Some(17.0), FIGHTER_STATUS_KIND_POT_SPECIALHI_CAUGHT, *COLLISION_SITUATION_MASK_GA);

    }
    sv_animcmd::wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        macros::FT_MOTION_RATE(agent, 1.75);
    }
}



unsafe extern "C" fn effect_special_hi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_special_hi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_16"));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_01"));
    }
}

unsafe extern "C" fn special_hi_start(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
}

unsafe extern "C" fn sound_special_hi_start(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_btl_pot_02"));
        let handle = SoundModule::play_se(agent.module_accessor, Hash40::new("se_btl_pot_02"), true, false, false, false, enSEType(0));
        if !WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_ENABLE_CAMERA){
            SoundModule::set_se_speed(agent.module_accessor,handle as i32, 2.0)
        }
    }
    sv_animcmd::frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_heat_extender_X_X".to_string(),2,true,false,1);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
    }
}

unsafe extern "C" fn expression_special_hi_start(agent: &mut L2CAgentBase) {
    if !WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_ENABLE_CAMERA){
        if macros::is_excute(agent) {
            macros::FT_MOTION_RATE(agent, 0.5);
        }
    }
}



unsafe extern "C" fn special_hi_loop(agent: &mut L2CAgentBase) {
    if !WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_ENABLE_CAMERA){
        if macros::is_excute(agent) {
            macros::FT_MOTION_RATE(agent, 0.5);
        }
    }
    sv_animcmd::frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent,0,0,Hash40::new("top"),1.0,361,30,10,12,5.0,0.0,30.0,14.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_none"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    sv_animcmd::frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}


unsafe extern "C" fn effect_special_hi_loop(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let lr = PostureModule::lr(agent.module_accessor);
        let pos = Vector3f{x:10.0, y:15.0, z:-2.0};
        let rot = Vector3f{x:-60.0, y:0.0, z:-20.0*lr};
        let pos2 = Vector3f{x:10.0, y:30.0, z:15.0}; //z is left ad right, y is up and down, x is in and out
        let rot2 = Vector3f{x:0.0, y:0.0, z:120.0*lr}; 

        EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_spark"), Hash40::new("top"), &pos2, &rot2, 2.5, true, 0, 0, 0, 0, 0, true, true) as u32;
        let effect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_attack_line_b"), Hash40::new("top"), &pos, &rot, 3.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rgb(agent.module_accessor, effect, 1.0, 0.24, 0.0);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
    }
    sv_animcmd::frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        let lr = PostureModule::lr(agent.module_accessor);
        let pos = Vector3f{x:-10.0, y:25.0, z:9.0};
        let rot = Vector3f{x:-60.0, y:0.0, z:-20.0*lr};
        EffectModule::req_follow(agent.module_accessor, Hash40::new("kamui_extender_bullet"), Hash40::new("top"), &pos, &rot, 4.0, true, 0, 0, 0, 0, 0, true, true) as u32;
    }
}

unsafe extern "C" fn expression_special_hi_loop(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_ENABLE_CAMERA){
        sv_animcmd::frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        }
        sv_animcmd::frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            CameraModule::stop_quake(agent.module_accessor, *CAMERA_QUAKE_KIND_XL);
        }
    }else{
        sv_animcmd::frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
        sv_animcmd::frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            CameraModule::stop_quake(agent.module_accessor, *CAMERA_QUAKE_KIND_S);
        }
    }
}



unsafe extern "C" fn special_hi_release(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 361, 100, 0, 60, 1.5, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 10.0, 361, 100, 0, 60, 1.5, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        WorkModule::on_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_CAPTURE);
    }
    sv_animcmd::frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent,0,0,Hash40::new("top"),1.0,361,30,10,12,5.0,0.0,30.0,14.0,None,None,None
        ,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_none"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        WorkModule::on_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_FINAL_JAB_HIT_MOTION_PLAY);
    }
    sv_animcmd::wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    sv_animcmd::frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        macros::ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("top"), 10.0, 361, 100, 0, 60, 12.0, 0.0, 28.0, 12.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    
    sv_animcmd::frame(agent.lua_state_agent, 47.0);
    if macros::is_excute(agent) {
        let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        UiManager::add_tension(get_ui_index_from_entry_id(entry_id),18.0);
        WorkModule::on_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_HI_FLAG_HIT);
    }
    sv_animcmd::wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_ENABLE_CAMERA){
            macros::FT_MOTION_RATE(agent, 2.0);
        }else{
            macros::FT_MOTION_RATE(agent, 1.0);
        }
    }

}

unsafe extern "C" fn effect_special_hi_release(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        let lr = PostureModule::lr(agent.module_accessor);
        let pos = Vector3f{x:10.0, y:15.0, z:-2.0}; //z is left ad right, y is up and down, x is in and out
        let rot = Vector3f{x:-60.0, y:0.0, z:-20.0*lr};
        let pos2 = Vector3f{x:10.0, y:30.0, z:15.0}; 
        let rot2 = Vector3f{x:0.0, y:0.0, z:120.0*lr}; 

        EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_spark"), Hash40::new("top"), &pos2, &rot2, 2.5, true, 0, 0, 0, 0, 0, true, true) as u32;
        let effect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_attack_line_b"), Hash40::new("top"), &pos, &rot, 3.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rgb(agent.module_accessor, effect, 1.0, 0.24, 0.0);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
    }
    sv_animcmd::frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        let lr = PostureModule::lr(agent.module_accessor);
        let pos = Vector3f{x:-10.0, y:25.0, z:9.0};
        let rot = Vector3f{x:-60.0, y:0.0, z:-20.0*lr};
        EffectModule::req_follow(agent.module_accessor, Hash40::new("kamui_extender_bullet"), Hash40::new("top"), &pos, &rot, 4.0, true, 0, 0, 0, 0, 0, true, true) as u32;
    }
    sv_animcmd::frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        macros::FILL_SCREEN_MODEL_COLOR(agent, 0, 0, 0.0, 0.0, 0.2, 1, 1, 0.8, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, *EFFECT_SCREEN_PRIO_FINAL);
        macros::CANCEL_FILL_SCREEN(agent, 0, 5);
    }
    sv_animcmd::frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bomb_d"), Hash40::new("haver"), 0, 0, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.6);
    }
    sv_animcmd::wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FILL_SCREEN_MODEL_COLOR(agent, 0, 0, 0.0, 0.0, 0.2, 1, 1, 0.8, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, *EFFECT_SCREEN_PRIO_FINAL);
        macros::CANCEL_FILL_SCREEN(agent, 0, 30);
    }

}


unsafe extern "C" fn sound_special_hi_release(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_heat_extender_X_X".to_string(),2,true,true,2);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
        macros::PLAY_SE(agent, Hash40::new("se_kamui_special_h02"));
    }
}

unsafe extern "C" fn expression_special_hi_release(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_ENABLE_CAMERA){
        sv_animcmd::frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            CameraModule::stop_quake(agent.module_accessor, *CAMERA_QUAKE_KIND_XL);
        }
        sv_animcmd::frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        }
        sv_animcmd::frame(agent.lua_state_agent, 45.0);
        if macros::is_excute(agent) {        
            macros::CAM_ZOOM_OUT(agent);
            let scale = PostureModule::scale(agent.module_accessor);
            let lr = PostureModule::lr(agent.module_accessor);
            CameraModule::reset_all(agent.module_accessor);            
            //macros::CAM_ZOOM_IN_arg6(
            //    agent,
            //    /*frames to zoom in totally*/3.0,
            //    /*according to claude1308 the object id?*/0.0,
            //    /*the amt of zoom needed, higher the number lower the zoom*/2.5*scale*1.3,
            //    /*y_rot*/0.35, // up and down
            //    /*x_rot*/-0.6 * lr, //left and right
            //    true
            //);
            macros::CAM_ZOOM_IN_arg6(
                agent,
                /*frames to zoom in totally*/3.0,
                /*according to claude1308 the object id?*/0.0,
                /*the amt of zoom needed, higher the number lower the zoom*/3.5*scale*1.3,
                /*y_rot*/0.13, // up and down
                /*x_rot*/-0.7 * lr, //left and right
                /*this seems to be a quake multiplier*/ true,
            );
            CameraModule::stop_quake(agent.module_accessor, *CAMERA_QUAKE_KIND_XL);
            macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
            agent.clear_lua_stack();
            lua_args!(agent,
            *MA_MSC_CMD_CAMERA_CAM_OFFSET,
            /*x*/12.0 * lr,
            /*y*/17.0,
            /*z*/0);
            smash::app::sv_module_access::camera(agent.lua_state_agent);

        }
        let og_fov = sv_camera_manager::get_fov();
        for n in 0..3{
            sv_animcmd::wait(agent.lua_state_agent, 1.0);
            if macros::is_excute(agent) {
                let dat = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64 + 0x52b7f00) as *const *const u64;
                let mut fov_ptr = (**dat + 0xdac) as *mut f32;
                let fov_ratio = *(fov_ptr);
                let fov = (fov_ratio * 180.0) / 3.1415927;
                
                let adjusted_fov = og_fov + (80.0-og_fov)/3.0 * (n+1) as f32;

                *fov_ptr = adjusted_fov/180.0 * 3.1415927;
            }    
        }
        sv_animcmd::wait(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::CAM_ZOOM_OUT(agent);
            let scale = PostureModule::scale(agent.module_accessor);
            let lr = PostureModule::lr(agent.module_accessor);
            CameraModule::reset_all(agent.module_accessor);            
            macros::CAM_ZOOM_IN_arg6(
                agent,
                /*frames to zoom in totally*/3.0,
                /*according to claude1308 the object id?*/0.0,
                /*the amt of zoom needed, higher the number lower the zoom*/3.5*scale*1.3,
                /*y_rot*/0.0, // up and down
                /*x_rot*/0.0, //left and right
                /*this seems to be a quake multiplier*/ true,
            );
            agent.clear_lua_stack();
            lua_args!(agent,
            *MA_MSC_CMD_CAMERA_CAM_OFFSET,
            /*x*/12.0 * lr,
            /*y*/17.0,
            /*z*/0);
            smash::app::sv_module_access::camera(agent.lua_state_agent);
        }
        sv_animcmd::wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::CAM_ZOOM_OUT(agent);
        }
    }else{
        if macros::is_excute(agent) {
            macros::FT_MOTION_RATE(agent, 0.5);
        }
        sv_animcmd::frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            CameraModule::stop_quake(agent.module_accessor, *CAMERA_QUAKE_KIND_S);
        }
        sv_animcmd::frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }

    }
}








pub fn install(agent: &mut Agent) {
    agent.game_acmd         ("game_specialhi",                  special_hi,                     Priority::Low);
    agent.sound_acmd        ("sound_specialhi",                 sound_special_hi,               Priority::Low);
    agent.effect_acmd       ("effect_specialhi",                effect_special_hi,              Priority::Low);


    agent.game_acmd         ("game_specialhistart",             special_hi_start,               Priority::Low);
    agent.sound_acmd        ("sound_specialhistart",            sound_special_hi_start,         Priority::Low);
    agent.expression_acmd   ("expression_specialhistart",       expression_special_hi_start,    Priority::Low);

    agent.game_acmd         ("game_specialhiloop",              special_hi_loop,                Priority::Low);
    agent.effect_acmd       ("effect_specialhiloop",            effect_special_hi_loop,         Priority::Low);
    agent.expression_acmd   ("expression_specialhiloop",        expression_special_hi_loop,     Priority::Low);

    agent.game_acmd         ("game_specialhirelease",           special_hi_release,             Priority::Low);
    agent.effect_acmd       ("effect_specialhirelease",         effect_special_hi_release,      Priority::Low);
    agent.sound_acmd        ("sound_specialhirelease",          sound_special_hi_release,       Priority::Low);
    agent.expression_acmd   ("expression_specialhirelease",     expression_special_hi_release,  Priority::Low);
}