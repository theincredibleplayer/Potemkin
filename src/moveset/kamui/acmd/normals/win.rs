use super::*;

unsafe extern "C" fn game_win4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("body_base"),    true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("body_emi"),     true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("body_decal"),   true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("head_base"),    true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("head_emi"),     true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("low_bodybase"),    false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("low_bodyemi"),     false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("low_bodydecal"),   false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("low_weaponbase"),  false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("low_weapondecal"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("low_weaponemi"),   false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("low_headbase"),    false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("low_heademi"),     false);



        let new_rot = Vector3f{x:0.0,y:80.0,z:0.0};
        PostureModule::set_rot(agent.module_accessor, &new_rot, 0);
        PostureModule::set_scale(agent.module_accessor, 0.6, false);

        ArticleModule::generate_article(agent.module_accessor,FIGHTER_KAMUI_GENERATE_ARTICLE_WEAPON,false,-1);
        potemkin_mirror_handler(agent.module_accessor);
        ArticleModule::change_motion(agent.module_accessor,FIGHTER_KAMUI_GENERATE_ARTICLE_WEAPON,Hash40::new("appeal_s"),false,-1.0);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor,FIGHTER_KAMUI_GENERATE_ARTICLE_EFFECT,false,-1);
        potemkin_mirror_handler(agent.module_accessor);
        ArticleModule::set_visibility_whole(agent.module_accessor, FIGHTER_KAMUI_GENERATE_ARTICLE_EFFECT, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(agent.module_accessor, FIGHTER_KAMUI_GENERATE_ARTICLE_EFFECT, Hash40::new("special_air_n_hold"), false, -1.0);
        let plane = ArticleModule::get_article(agent.module_accessor,FIGHTER_KAMUI_GENERATE_ARTICLE_EFFECT);
        let plane_boma = *(plane as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
        LinkModule::remove_model_constraint(plane_boma,true);
        LinkModule::set_model_constraint_pos_ort(plane_boma,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("attach"),Hash40::new("top"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32 | *CONSTRAINT_FLAG_OFFSET_ROT as u32,true);
        ArticleModule::change_motion(agent.module_accessor, FIGHTER_KAMUI_GENERATE_ARTICLE_EFFECT, Hash40::new("special_air_n_hold"), false, -1.0);
    }
}


pub unsafe extern "C" fn sound_win4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_btl_pot_11"));
    }

    frame(agent.lua_state_agent, 104.0);
    if macros::is_excute(agent) {
        let sound = sound_selector(agent,"vc_pot_taunt_hi_X".to_string(),3,false,false,0);
        macros::PLAY_SE(agent, Hash40::new_raw(sound));
    }
} 

pub unsafe extern "C" fn effect_win4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 101.0);
    if macros::is_excute(agent) {
        for n in 1..10{
            let random = smash::app::sv_math::rand(hash40("fighter"), 2);
            let effect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("handr"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 1.3, true, 0, 0, 0, 0, 0, true, true) as u32;
            if random == 0 {
                EffectModule::set_rgb(agent.module_accessor, effect, 1.2, 0.8, 0.0);
            }else{
                EffectModule::set_rgb(agent.module_accessor, effect, 0.25, 0.5, 1.0);
            }
            let effect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("handl"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 1.3, true, 0, 0, 0, 0, 0, true, true) as u32;    
            if random == 0 {
                EffectModule::set_rgb(agent.module_accessor, effect, 1.2, 0.8, 0.0);
            }else{
                EffectModule::set_rgb(agent.module_accessor, effect, 0.25, 0.5, 1.0);
            }
            wait(agent.lua_state_agent, 4.0);
        }
    }
    frame(agent.lua_state_agent, 143.0);
    if macros::is_excute(agent){
        for n in 0..4{
            let random = smash::app::sv_math::rand(hash40("fighter"), 2);
            let effect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("handr"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 1.3, true, 0, 0, 0, 0, 0, true, true) as u32;
            if random == 0 {
                EffectModule::set_rgb(agent.module_accessor, effect, 1.2, 0.8, 0.0);
            }else{
                EffectModule::set_rgb(agent.module_accessor, effect, 0.25, 0.5, 1.0);
            }
            let effect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("handl"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 1.3, true, 0, 0, 0, 0, 0, true, true) as u32;    
            if random == 0 {
                EffectModule::set_rgb(agent.module_accessor, effect, 1.2, 0.8, 0.0);
            }else{
                EffectModule::set_rgb(agent.module_accessor, effect, 0.25, 0.5, 1.0);
            }
            wait(agent.lua_state_agent, 4.0);
        }
    }
    
    frame(agent.lua_state_agent, 160.0);
    if macros::is_excute(agent) {
        
        ArticleModule::set_frame(agent.module_accessor, FIGHTER_KAMUI_GENERATE_ARTICLE_WEAPON, 143.0);
        //ArticleModule::set_rate(agent.module_accessor,FIGHTER_KAMUI_GENERATE_ARTICLE_WEAPON,0.0);
        //MotionModule::set_rate(agent.module_accessor,0.0);
        //MotionAnimcmdModule::call_script_single(agent.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_win4wait"), -1);
        MotionModule::change_motion(agent.module_accessor,Hash40::new("win_4_wait"),143.0,1.0,false,0.0,false,false);
        //MotionModule::set_frame_sync_anim_cmd(agent.module_accessor, 143.0, true, false, false);
        //MotionModule::set_frame(agent.module_accessor,143.0,false);
    }
} 

unsafe extern "C" fn game_win4wait(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("body_base"),    true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("body_emi"),     true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("body_decal"),   true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("head_base"),    true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("head_emi"),     true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("low_bodybase"),    false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("low_bodyemi"),     false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("low_bodydecal"),   false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("low_weaponbase"),  false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("low_weapondecal"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("low_weaponemi"),   false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("low_headbase"),    false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("low_heademi"),     false);
    }
    frame(agent.lua_state_agent, 160.0);
    if macros::is_excute(agent) {
        ArticleModule::set_frame(agent.module_accessor, FIGHTER_KAMUI_GENERATE_ARTICLE_WEAPON, 143.0);
        //MotionModule::set_frame_sync_anim_cmd(agent.module_accessor, 143.0, true, false, false);
        //ArticleModule::set_rate(agent.module_accessor,FIGHTER_KAMUI_GENERATE_ARTICLE_WEAPON,0.0);
        //MotionModule::set_rate(agent.module_accessor,0.0);

        MotionModule::change_motion(agent.module_accessor,Hash40::new("win_4_wait"),143.0,1.0,false,0.0,false,false);
    }
}

pub unsafe extern "C" fn effect_win4wait(agent: &mut L2CAgentBase) {
    execute(agent.lua_state_agent, 143.0);
    frame(agent.lua_state_agent, 143.0);
    if macros::is_excute(agent){
        loop{
            let random = smash::app::sv_math::rand(hash40("fighter"), 2);
            let effect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("handr"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 1.3, true, 0, 0, 0, 0, 0, true, true) as u32;
            if random == 0 {
                EffectModule::set_rgb(agent.module_accessor, effect, 1.2, 0.8, 0.0);
            }else{
                EffectModule::set_rgb(agent.module_accessor, effect, 0.25, 0.5, 1.0);
            }
            let effect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("handl"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 1.3, true, 0, 0, 0, 0, 0, true, true) as u32;    
            if random == 0 {
                EffectModule::set_rgb(agent.module_accessor, effect, 1.2, 0.8, 0.0);
            }else{
                EffectModule::set_rgb(agent.module_accessor, effect, 0.25, 0.5, 1.0);
            }
            wait(agent.lua_state_agent, 4.0);
        }
    }   
}


pub fn install(agent: &mut Agent) {
    agent.game_acmd          ("game_win4",              game_win4,              Priority::Low);
    agent.sound_acmd         ("sound_win4",             sound_win4,             Priority::Low);
    agent.effect_acmd        ("effect_win4",            effect_win4,            Priority::Low);


    agent.game_acmd          ("game_win4wait",              game_win4wait,              Priority::Low);
    agent.effect_acmd        ("effect_win4wait",            effect_win4wait,            Priority::Low);


}