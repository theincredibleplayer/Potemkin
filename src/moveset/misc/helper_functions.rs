use super::*;


pub unsafe extern "C" fn potemkin_mirror_handler(module_accessor: *mut BattleObjectModuleAccessor) {
    let status_kind =StatusModule::status_kind(module_accessor) ;
    let scale = PostureModule::scale(module_accessor); //gets the model's scale
    let situation_kind = StatusModule::situation_kind(module_accessor);
    let motion_kind =MotionModule::motion_kind(module_accessor);
    let lr = PostureModule::lr(module_accessor);
    let mut can_mirror = true;
    if [
        hash40("ladder_wait"),
        hash40("ladder_up"),
        hash40("ladder_down"),
        hash40("ladder_catch_r"),
        hash40("ladder_catch_l"),
        hash40("ladder_catch_air_r"),
        hash40("ladder_catch_air_l"),
        hash40("ladder_catch_end_r"),
        hash40("ladder_catch_end_l"),
    ].contains(&motion_kind){
        can_mirror = false;
    }
    // scales the models of both pot and his articles by -1 on the x axis to mirror (accurate to strive)
    if (((lr < 0.0) && (![*FIGHTER_STATUS_KIND_TURN,FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN].contains(&status_kind) && !((motion_kind == hash40("air_turn")) || (motion_kind == hash40("turn")) || (motion_kind == hash40("squat_turn"))) ) )
    || (((lr > 0.0) && ( [*FIGHTER_STATUS_KIND_TURN,FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN].contains(&status_kind) || ((motion_kind == hash40("air_turn")) || (motion_kind == hash40("turn")) || (motion_kind == hash40("squat_turn"))) ) ) )
    && can_mirror)
    {
            ModelModule::set_joint_scale(module_accessor, Hash40::new("top")   , &Vector3f{x:-scale,y:scale,z:scale});
            ModelModule::set_joint_scale(module_accessor, Hash40::new("havel") , &Vector3f{x:-scale,y:scale,z:-scale});
            ModelModule::set_joint_scale(module_accessor, Hash40::new("haver") , &Vector3f{x:-scale,y:scale,z:-scale});
            //ModelModule::set_joint_scale(module_accessor, Hash40::new("throw") , &Vector3f{x:-scale,y:scale,z:-scale});

            //ModelModule::set_joint_rotate(boma, Hash40::new("waist"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})
            //ModelModule::set_joint_rotate(boma, Hash40::new("waist"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})

            //ModelModule::set_joint_scale(module_accessor, Hash40::new("throw") , &Vector3f{x:-scale,y:-scale,z:scale});
            if ArticleModule::is_exist(module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND){
                let article = ArticleModule::get_article(module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND);
                let article_boma = *(article as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
                ModelModule::set_joint_scale(article_boma, Hash40::new("hand1") , &Vector3f{x:-1.0,y:1.0,z:1.0});
            }
            if ArticleModule::is_exist(module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON){
                let article2 = ArticleModule::get_article(module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON);
                let article_boma2 = *(article2 as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
                ModelModule::set_joint_scale(article_boma2, Hash40::new("trans") , &Vector3f{x:-1.0,y:1.0,z:1.0});
            }
            if ArticleModule::is_exist(module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND){
                let article3 = ArticleModule::get_article(module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND);
                let article_boma3 = *(article3 as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
                ModelModule::set_joint_scale(article_boma3, Hash40::new("trans") , &Vector3f{x:-1.0,y:1.0,z:1.0});
            }
    } else { 
        // reverses the -ve scale (i am not sure if its actually needed but we ball ig)
        ModelModule::set_joint_scale(module_accessor, Hash40::new("top")   , &Vector3f{x:scale,y:scale,z:scale});
        //ModelModule::set_joint_scale(module_accessor, Hash40::new("havel") , &Vector3f{x:scale,y:scale,z:scale});
        //ModelModule::set_joint_scale(module_accessor, Hash40::new("haver") , &Vector3f{x:scale,y:scale,z:scale});
        //ModelModule::set_joint_scale(module_accessor, Hash40::new("throw") , &Vector3f{x:scale,y:scale,z:scale});

        //ModelModule::set_joint_scale(module_accessor, Hash40::new("throw") , &Vector3f{x:scale,y:scale,z:scale});

        if ArticleModule::is_exist(module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND){
            let article = ArticleModule::get_article(module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND);
            let article_boma = *(article as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
            ModelModule::set_joint_scale(article_boma, Hash40::new("hand1") , &Vector3f{x:1.0,y:1.0,z:1.0});
        }
        if ArticleModule::is_exist(module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON){
            let article2 = ArticleModule::get_article(module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON);
            let article_boma2 = *(article2 as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
            ModelModule::set_joint_scale(article_boma2, Hash40::new("trans") , &Vector3f{x:1.0,y:1.0,z:1.0});
        }
        if ArticleModule::is_exist(module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND){
            let article3 = ArticleModule::get_article(module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND);
            let article_boma3 = *(article3 as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
            ModelModule::set_joint_scale(article_boma3, Hash40::new("trans") , &Vector3f{x:1.0,y:1.0,z:1.0});
        }
    }
}

pub unsafe extern "C" fn potemkin_weapon_attach_handler(module_accessor: *mut BattleObjectModuleAccessor) {
    //let status_kind =StatusModule::status_kind(module_accessor) ;
    //let scale = PostureModule::scale(module_accessor); //gets the model's scale
    //let situation_kind = StatusModule::situation_kind(module_accessor);
    //let motion_kind =MotionModule::motion_kind(module_accessor);
    //if WorkModule::is_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_ATTACH_WEAPON){
    //    let lr = PostureModule::lr(module_accessor);
    //    let dragon = ArticleModule::get_article(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON);
    //    let dragon_boma = *(dragon as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
    //    let pot_bone_names     = &["g_lowarm_weapon_r_attach", "g_lowarm_weapon_l_attach","g_hand_weapon_r_attach", "g_hand_weapon_l_attach", "g_back_gear_r", "g_back_gear_l", "g_chest_gear_r", "g_chest_gear_l"];
    //    let article_bone_names = &["g_coverbase_r" , "g_coverbase_l" ,"g_hand_wephigh_r_base" ,  "g_hand_wephigh_l_base", "g_back_gear_r", "g_back_gear_l", "g_chest_gear_r", "g_chest_gear_l"];
    //    for i in 0..8{
    //        let pos   = &mut Vector3f{x:0.0,y:0.0,z:0.0};
    //        let rot   = &mut Vector3f{x:0.0,y:0.0,z:0.0};
    //        let scale = &mut Vector3f{x:1.0,y:1.0,z:1.0};
    //        ModelModule::joint_global_rotation(module_accessor,Hash40::new(pot_bone_names[i]), rot,false);
    //        ModelModule::joint_global_position(module_accessor,Hash40::new(pot_bone_names[i]),pos,true);
    //        if (i > 1) && (lr > 0.0) { // hand weapons
    //        }else if (i <= 1) && (lr > 0.0){ // arm weapons facing right
    //            let rot_corrected = &mut Vector3f{x:rot.x, y:rot.y, z:rot.z};
    //            ModelModule::set_joint_rotate(dragon_boma, Hash40::new(article_bone_names[i]), rot_corrected, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_WORLD as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    //        }else if (i <= 1) && (lr < 0.0){ //arm weapons facing left
    //            let mut multiplier : f32= -1.0;
    //            if (i == 1) {
    //                multiplier = 1.0;
    //            }
    //            let rot_corrected = &mut Vector3f{x:rot.x, y:rot.y*multiplier, z:rot.z}; //if facing left his right arm works at x +180 and y+180 
    //            ModelModule::set_joint_rotate(dragon_boma, Hash40::new(article_bone_names[i]), rot_corrected, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_WORLD as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    //        }
    //        ModelModule::set_joint_translate(dragon_boma, Hash40::new(article_bone_names[i]), pos, true, false);
    //    }
    //    // i dont exactly know why this doesnt like fix the rotation like i expected its kinda offset a bit
    //}
}

pub unsafe extern "C" fn grabbed_anim_selector(fighter: &mut L2CFighterCommon,anim_name: &str, mot_rate: f32) {
    let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);

    if capture_id != 0x50000000 {
        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
        //println!("playing anim");
        //MotionModule::change_motion(captured_boma,Hash40::new("capture_wait_hi"),0.0,1.0,false,0.0,false,false);
        let capture_category = sv_battle_object::category((*capture_boma).battle_object_id);
        // finds out the body type of the opponent and changes the motion variable to that
        if capture_category == *BATTLE_OBJECT_CATEGORY_FIGHTER{
            let motion_share = WorkModule::get_param_int(capture_boma,0xcad2ee25e,0xc07d88ea0); //grabs the body type param
            let mut motion = hash40(anim_name);
            let mut share_type = 0;
            
            if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
                motion = FighterMotionModuleImpl::add_body_type_hash(
                    capture_boma,
                    Hash40::new_raw(motion),
                    *BODY_TYPE_MOTION_DX
                );
            }

            else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
                motion = FighterMotionModuleImpl::add_body_type_hash(
                    capture_boma,
                    Hash40::new_raw(motion),
                    *BODY_TYPE_MOTION_GIRL
                );
            }

            else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_BIG {
                motion = FighterMotionModuleImpl::add_body_type_hash(
                    capture_boma,
                    Hash40::new_raw(motion),
                    *BODY_TYPE_MOTION_BIG
                );
            }


            //plays the anim
            MotionModule::change_motion(capture_boma, Hash40::new_raw(motion), 0.0, mot_rate, false, 0.0, false, false);
            //println!("flipped state: {}", boi);


            
            //let pre_flip_state = MotionModule::is_flip(capture_boma);
            ////WorkModule::set_flag(capture_boma,boi,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_FLIP);
            //// this should flip the opponent if im facing left but it doesnt work for some reason??
            //let parent_lr = PostureModule::lr(fighter.module_accessor);
            //PostureModule::set_lr(capture_boma, -parent_lr);
            //PostureModule::update_rot_y_lr(capture_boma);
            //if parent_lr < 0.0 {
            //    println!("flipped allegedly from elsewhere");
            //    MotionModule::set_flip(capture_boma,true,true,false);
            //} else {
            //    MotionModule::set_flip(capture_boma,false,true,false);
            //}
        }
    }
}

pub unsafe extern "C" fn camera_checks(fighter: &mut L2CAgentBase) -> bool {
    let ret = 
    !sv_information::is_flat_stage() 
    && FighterCutInManager__is_one_on_one() 
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA);

    return ret;
}

pub unsafe extern "C" fn attack_sound_randomizer(fighter: &mut L2CAgentBase, mut attack_type: &str) -> u64 {
    let mut random = smash::app::sv_math::rand(hash40("fighter"), 6);
    let mut sound_name = ["vc_pot_attack_".to_string(),attack_type.to_string()].join("");
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LAST_ATTACK_SOUND_ID) == random {
        random = (random + 1)%6;
    }
    match random{
        0 => sound_name = [sound_name,"_a".to_string()].join(""),
        1 => sound_name = [sound_name,"_b".to_string()].join(""),
        2 => sound_name = [sound_name,"_c".to_string()].join(""),
        3 => sound_name = [sound_name,"_d".to_string()].join(""),
        4 => sound_name = [sound_name,"_e".to_string()].join(""),
        5 => sound_name = [sound_name,"_f".to_string()].join(""),
        _ => (),
    }
    WorkModule::set_int(fighter.module_accessor, random, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LAST_ATTACK_SOUND_ID);
    return SoundModule::play_se(fighter.module_accessor, Hash40::new(sound_name.as_str()), true, false, false, false, enSEType(0));
}

// used for sound randomization
pub unsafe extern "C" fn sound_selector(fighter: &mut L2CAgentBase, mut sound_name: String, num_of_vars: i32, is_sequence: bool, continue_sequence: bool, sequence_order_number: i32) -> u64 {
    let mut random = smash::app::sv_math::rand(hash40("fighter"), num_of_vars);
    let random2 = smash::app::sv_math::rand(hash40("fighter"), num_of_vars-1);
    let last_sound_id = WorkModule::get_int(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LAST_SPECIAL_SOUND_ID);
    
    if !continue_sequence{
        if last_sound_id == random {
            random = (random + 1)%(num_of_vars);
        }
        if is_sequence{
            for _ in 0..3 {
                sound_name.pop();
            }
            match random{
                0 => sound_name = [sound_name,"a_1".to_string()].join(""),
                1 => sound_name = [sound_name,"b_1".to_string()].join(""),
                2 => sound_name = [sound_name,"c_1".to_string()].join(""),
                3 => sound_name = [sound_name,"d_1".to_string()].join(""),
                4 => sound_name = [sound_name,"e_1".to_string()].join(""),
                5 => sound_name = [sound_name,"f_1".to_string()].join(""),
                _ => (),
            }
        }else{
            sound_name.pop();
            match random{
                0 => sound_name = [sound_name,"a".to_string()].join(""),
                1 => sound_name = [sound_name,"b".to_string()].join(""),
                2 => sound_name = [sound_name,"c".to_string()].join(""),
                3 => sound_name = [sound_name,"d".to_string()].join(""),
                4 => sound_name = [sound_name,"e".to_string()].join(""),
                5 => sound_name = [sound_name,"f".to_string()].join(""),
                _ => (),
            }
        }
        WorkModule::set_int(fighter.module_accessor, random, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_LAST_SPECIAL_SOUND_ID);
    } else {
        for _ in 0..3 {
            sound_name.pop();
        }
        match last_sound_id{
            0 => sound_name = [sound_name,"a_".to_string()].join(""),
            1 => sound_name = [sound_name,"b_".to_string()].join(""),
            2 => sound_name = [sound_name,"c_".to_string()].join(""),
            3 => sound_name = [sound_name,"d_".to_string()].join(""),
            4 => sound_name = [sound_name,"e_".to_string()].join(""),
            5 => sound_name = [sound_name,"f_".to_string()].join(""),
            _ => (),
        }
        sound_name = [sound_name,sequence_order_number.to_string()].join("");
    }
    return hash40(sound_name.as_str())
}

pub unsafe extern "C" fn rc_speed_float_setter(fighter: &mut L2CFighterCommon) {
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    WorkModule::set_float(fighter.module_accessor, speed_x, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_PRE_RC_X_SPEED);
    WorkModule::set_float(fighter.module_accessor, speed_y, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_PRE_RC_Y_SPEED);
}

