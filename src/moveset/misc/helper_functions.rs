use super::*;


pub unsafe extern "C" fn potemkin_mirror_handler(module_accessor: *mut BattleObjectModuleAccessor) {
    let status_kind =StatusModule::status_kind(module_accessor) ;
    let scale = PostureModule::scale(module_accessor); //gets the model's scale
    let situation_kind = StatusModule::situation_kind(module_accessor);
    let motion_kind =MotionModule::motion_kind(module_accessor);
    let lr = PostureModule::lr(module_accessor);
    // scales the models of both pot and his articles by -1 on the x axis to mirror (accurate to strive)
    if ((lr < 0.0) && (![*FIGHTER_STATUS_KIND_TURN,FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN].contains(&status_kind) && !((motion_kind == hash40("air_turn")) || (motion_kind == hash40("turn"))) ) )
    || (((lr > 0.0) && ( [*FIGHTER_STATUS_KIND_TURN,FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN].contains(&status_kind) || ((motion_kind == hash40("air_turn")) || (motion_kind == hash40("turn"))) ) ) )
    {
            ModelModule::set_joint_scale(module_accessor, Hash40::new("top") , &Vector3f{x:-scale,y:scale,z:scale});
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
        ModelModule::set_joint_scale(module_accessor, Hash40::new("top") , &Vector3f{x:scale,y:scale,z:scale});
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
    let status_kind =StatusModule::status_kind(module_accessor) ;
    let scale = PostureModule::scale(module_accessor); //gets the model's scale
    let situation_kind = StatusModule::situation_kind(module_accessor);
    let motion_kind =MotionModule::motion_kind(module_accessor);
    if WorkModule::is_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_ATTACH_WEAPON){
        let lr = PostureModule::lr(module_accessor);
        let dragon = ArticleModule::get_article(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON);
        let dragon_boma = *(dragon as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
        let pot_bone_names     = &["g_lowarm_weapon_r_attach", "g_lowarm_weapon_l_attach","g_hand_weapon_r_attach", "g_hand_weapon_l_attach", "g_back_gear_r", "g_back_gear_l", "g_chest_gear_r", "g_chest_gear_l"];
        let article_bone_names = &["g_coverbase_r" , "g_coverbase_l" ,"g_hand_wephigh_r_base" ,  "g_hand_wephigh_l_base", "g_back_gear_r", "g_back_gear_l", "g_chest_gear_r", "g_chest_gear_l"];
        for i in 0..8{
            let pos   = &mut Vector3f{x:0.0,y:0.0,z:0.0};
            let rot   = &mut Vector3f{x:0.0,y:0.0,z:0.0};
            let scale = &mut Vector3f{x:1.0,y:1.0,z:1.0};
            ModelModule::joint_global_rotation(module_accessor,Hash40::new(pot_bone_names[i]), rot,false);
            ModelModule::joint_global_position(module_accessor,Hash40::new(pot_bone_names[i]),pos,true);
            if (i > 1) && (lr > 0.0) { // hand weapons
            }else if (i <= 1) && (lr > 0.0){ // arm weapons facing right
                let rot_corrected = &mut Vector3f{x:rot.x, y:rot.y, z:rot.z};
                ModelModule::set_joint_rotate(dragon_boma, Hash40::new(article_bone_names[i]), rot_corrected, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_WORLD as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            }else if (i <= 1) && (lr < 0.0){ //arm weapons facing left
                let mut multiplier : f32= -1.0;
                if (i == 1) {
                    multiplier = 1.0;
                }
                let rot_corrected = &mut Vector3f{x:rot.x, y:rot.y*multiplier, z:rot.z}; //if facing left his right arm works at x +180 and y+180 
                ModelModule::set_joint_rotate(dragon_boma, Hash40::new(article_bone_names[i]), rot_corrected, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_WORLD as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            }
            ModelModule::set_joint_translate(dragon_boma, Hash40::new(article_bone_names[i]), pos, true, false);
        }
        // i dont exactly know why this doesnt like fix the rotation like i expected its kinda offset a bit
    }
}



