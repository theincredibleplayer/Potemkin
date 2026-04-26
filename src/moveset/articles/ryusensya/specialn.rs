use super::*;



pub unsafe extern "C" fn game_regular(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let owner_id = WorkModule::get_int(agent.module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
        let owner_accessor = sv_battle_object::module_accessor(owner_id as u32);
        let mut damage = WorkModule::get_float(owner_accessor,FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_REAL_POWER_REFLECT);
        if damage < 7.0{
            damage = 7.0/1.4;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"),  damage * 1.4 ,
         55, 90, 0, 70, 25.0, 0.0, 0.0, 0.0, None, None, None,
         2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false,
         0, 0.0, 0, true, true
         , false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
         AttackModule::disable_tip(agent.module_accessor);

    }
    //frame(agent.lua_state_agent, 60.0);
    //if macros::is_excute(agent) {
    //    WorkModule::on_flag(agent.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_ATTACK);
    //}
}

pub unsafe extern "C" fn effect_regular(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_fdb_fire"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, true);
        //EffectModule::set_rate_last(agent.module_accessor, 0.5);
        let random_number = smash::app::sv_math::rand(hash40("fighter"), 3);
        let mut eff_string = "sys_bomb_a";
        if random_number == 0{
            let mut eff_string = "se_common_fire_s";
        } else if random_number == 1 {
            let mut eff_string = "se_common_fire_l";
        } else {
            let mut eff_string = "se_common_fire_m";
        }
        macros::EFFECT(agent, Hash40::new("kamui_fdb_explode"), Hash40::new("top"),0, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EffectModule::set_rate_last(agent.module_accessor, 1.5);
        EffectModule::set_scale_last(agent.module_accessor, &Vector3f{x:0.8,y:0.5,z:0.8} );
        macros::EFFECT_FLW_POS(agent, Hash40::new("kamui_fdb_fire"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 180, 0, 1, true);
        //macros::EFFECT(agent, Hash40::new(eff_string), Hash40::new("top"),0, 0, 0, 0, 0, 0, 1.0 + (smash::app::sv_math::rand(hash40("fighter"), 3) as f32 * 0.5), 0, 0, 0, 0, 0, 0, false);
    }
    for _ in 0..99{
        sv_animcmd::wait(agent.lua_state_agent, 10.0/* + (smash::app::sv_math::rand(hash40("fighter"), 3) * 2 ) ) as f32*/);
        if macros::is_excute(agent) {
            let random_number = smash::app::sv_math::rand(hash40("fighter"), 3);
            let mut eff_string = "sys_bomb_a";
            if random_number == 0{
                let mut eff_string = "sys_bomb_a";
                macros::PLAY_SE(agent, Hash40::new("se_common_fire_s"));
            } else if random_number == 1 {
                let mut eff_string = "sys_bomb_b";
                macros::PLAY_SE(agent, Hash40::new("se_common_fire_l"));
            } else {
                let mut eff_string = "sys_bomb_c";
                macros::PLAY_SE(agent, Hash40::new("se_common_fire_m"));
            }
            macros::EFFECT_FLW_POS(agent, Hash40::new("kamui_fdb_fire"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 180, 0, 1, true);
            macros::EFFECT(agent, Hash40::new("kamui_fdb_explode"), Hash40::new("top"),0, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
            EffectModule::set_rate_last(agent.module_accessor, 1.5);
            EffectModule::set_scale_last(agent.module_accessor, &Vector3f{x:0.8,y:0.5,z:0.8} );
            //macros::EFFECT(agent, Hash40::new(eff_string), Hash40::new("top"),0, 0, 0, 0, 0, 0, 1.0 + (smash::app::sv_math::rand(hash40("fighter"), 3) as f32 * 0.5), 0, 0, 0, 0, 0, 0, false);
        }
    }
}

pub unsafe extern "C" fn game_shotmax(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d)); 
    }

}

pub unsafe extern "C" fn effect_shotmax(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bomb_d"), Hash40::new("top"),0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, false);
    }
}

pub unsafe extern "C" fn sound_shotmax(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_bomb"));
    }
}







pub unsafe extern "C" fn fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::on_flag(weapon.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_DID_IT_HIT);
    WorkModule::set_int(weapon.module_accessor,90,*WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor,90,*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    
    MotionModule::change_motion(weapon.module_accessor,Hash40::new("regular"),0.0,1.0,false,0.0,false,false);
  
    let posture = PostureModule::lr(weapon.module_accessor);
    weapon.clear_lua_stack();
    lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 2.0 * posture, 0.0);
    sv_kinetic_energy::set_speed(weapon.lua_state_agent);
  
    weapon.clear_lua_stack();
    lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 2.0 * posture, 0.0);
    sv_kinetic_energy::set_accel(weapon.lua_state_agent);
  
    weapon.clear_lua_stack();
    lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 2.0, 0.0);
    sv_kinetic_energy::set_limit_speed(weapon.lua_state_agent);
    WorkModule::set_int(weapon.module_accessor, 0, WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_INT_TIMER);
    weapon.fastshift(L2CValue::Ptr(fly_main_loop as *const () as _))
}

pub unsafe extern "C" fn fly_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    //if (WorkModule::get_int(weapon.module_accessor, WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_INT_TIMER) % 3) == 0{
    //    EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, true, 0, 0, 0, 0, 0, false, false);
    //}
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if WorkModule::is_flag(weapon.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_ATTACK) {
        weapon.change_status(WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM.into(),false.into());
    }
    if (WorkModule::get_int(weapon.module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0) {
        weapon.change_status(WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM.into(),false.into());
        return L2CValue::I32(0)
    }
    if (WorkModule::is_flag(weapon.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_HIT_WALL)) {
        if ( 2 <= weapon.global_table[0xE].get_int() ) {
            weapon.change_status(WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM.into(),false.into());
            return L2CValue::I32(0)
        }
    } else {
        if (weapon.sub_ground_module_is_touch_all_consider_speed().get_bool()) {
            if ( 1 <= weapon.global_table[0xE].get_int() ){
                if !WorkModule::is_flag(weapon.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_ATTACK) {
                    if (!GroundModule::is_touch(weapon.module_accessor,*GROUND_TOUCH_FLAG_ALL as u32)) {
                        return L2CValue::I32(0)
                    }
                }
                weapon.change_status(WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM.into(),false.into());
                return L2CValue::I32(0)
            }
            WorkModule::on_flag(weapon.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_HIT_WALL);
            if (weapon.global_table[0xE].get_int() <= 1) {
                weapon.change_status(WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM.into(),false.into());
                return L2CValue::I32(0)
            }
            //StopModule::set_other_stop(weapon.module_accessor,2,smash::StopOtherKind(0));
        }
    }
    if !WorkModule::is_flag(weapon.module_accessor,WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLAG_ATTACK) {
        if (!GroundModule::is_touch(weapon.module_accessor,*GROUND_TOUCH_FLAG_ALL as u32)) {
            return L2CValue::I32(0)
        }
    }
    weapon.change_status(WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM.into(),false.into());
    return L2CValue::I32(0)   
}

pub unsafe extern "C" fn boom_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return smashline::original_status(Pre, weapon, *WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_REGULAR)(weapon);
}

pub unsafe extern "C" fn boom_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor,Hash40::new("shot_max"),0.0,1.0,false,0.0,false,false);
  

    weapon.clear_lua_stack();
    lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy::set_speed(weapon.lua_state_agent);
  
    weapon.clear_lua_stack();
    lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy::set_accel(weapon.lua_state_agent);
  
    weapon.clear_lua_stack();
    lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy::set_limit_speed(weapon.lua_state_agent);
  
    weapon.fastshift(L2CValue::Ptr(boom_main_loop as *const () as _))
}

pub unsafe extern "C" fn boom_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return L2CValue::I32(0)   
}



pub fn install(agent: &mut Agent) {
    agent.game_acmd("game_regular", game_regular, Priority::Low);
    agent.effect_acmd("effect_regular", effect_regular, Priority::Low);
    agent.game_acmd("game_shotmax", game_shotmax, Priority::Low);
    agent.effect_acmd("effect_shotmax", effect_shotmax, Priority::Low);
    agent.sound_acmd("sound_shotmax", sound_shotmax, Priority::Low);

    agent.status(Main,*WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_REGULAR, fly_main);
    agent.status(Pre, WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM, boom_pre);
    agent.status(Main,WEAPON_KAMUI_RYUSENSYA_STATUS_KIND_BOOM, boom_main);

}