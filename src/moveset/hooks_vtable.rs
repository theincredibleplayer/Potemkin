use smash::lib::{L2CValue,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon};
use smash::hash40;
use smash::app::lua_bind::*;
use smashline::*;
use smash::app::*;
use smash_script::macros::*;
use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::phx::{Vector2f,Vector3f};
use skyline::nn::ro::LookupSymbol;
use smash_script::lua_args;
use smash_script::macros;
use std::arch::asm;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use crate::FIGHTER_MANAGER;

use crate::moveset::*;

//searchbox implementations
pub unsafe extern "C" fn kamui_on_search(_vtable: u64, fighter: &mut Fighter, log: u64, _some_float: f32) {
    let module_accessor = fighter.battle_object.module_accessor;
    
    if StatusModule::status_kind(module_accessor) == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL {
        let collision_log = *(log as *const u64).add(0x10 / 0x8);
        let collision_log = collision_log as *const CollisionLogScuffed;
        let opp_boma = sv_battle_object::module_accessor((*collision_log).opponent_object_id);
        //if AttackModule::is_attack(opp_boma,(*collision_log).collider_id as i32,false) {
            StatusModule::change_status_request(module_accessor,*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK,false);
        //}
    }
    if StatusModule::status_kind(module_accessor) == FIGHTER_KAMUI_STATUS_KIND_ROMAN_CANCEL {
        let collision_log = *(log as *const u64).add(0x10 / 0x8);
        let collision_log = collision_log as *const CollisionLogScuffed;
        let opp_boma = sv_battle_object::module_accessor((*collision_log).opponent_object_id);
        WorkModule::set_float(opp_boma, 0.3, *FIGHTER_STATUS_WORK_ID_FLOAT_REBOUND_MOTION_RATE);
        StatusModule::change_status_request_from_script(opp_boma, *FIGHTER_STATUS_KIND_REBOUND, false);
    }
    if StatusModule::status_kind(module_accessor) == FIGHTER_STATUS_KIND_SPECIAL_N {
        let collision_log = *(log as *const u64).add(0x10 / 0x8);
        let collision_log = collision_log as *const CollisionLogScuffed;
        let opp_boma = sv_battle_object::module_accessor((*collision_log).opponent_object_id);

        StatusModule::change_status_request_from_script(opp_boma, *FIGHTER_STATUS_KIND_REBOUND, false);
    }
}

pub unsafe extern "C" fn kamui_on_frame(_vtable: u64, fighter: &mut Fighter) {
    let scale = PostureModule::scale(fighter.battle_object.module_accessor); //gets the model's scake
    let module_accessor = fighter.battle_object.module_accessor;

    // scales the models of both pot and his articles by -1 on the x axis to mirror (accurate to strive)
    if ((PostureModule::lr(fighter.battle_object.module_accessor) < 0.0) && (StatusModule::status_kind(fighter.battle_object.module_accessor) != FIGHTER_STATUS_KIND_TURN))
    || ((PostureModule::lr(fighter.battle_object.module_accessor) > 0.0) && (StatusModule::status_kind(fighter.battle_object.module_accessor) == FIGHTER_STATUS_KIND_TURN)){
            ModelModule::set_joint_scale(fighter.battle_object.module_accessor, Hash40::new("top") , &Vector3f{x:-scale,y:scale,z:scale});
            if ArticleModule::is_exist(fighter.battle_object.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND){
                let article = ArticleModule::get_article(fighter.battle_object.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND);
                let article_boma = *(article as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
                ModelModule::set_joint_scale(article_boma, Hash40::new("hand1") , &Vector3f{x:-1.0,y:1.0,z:1.0});
            }
            if ArticleModule::is_exist(fighter.battle_object.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON){
                let article2 = ArticleModule::get_article(fighter.battle_object.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON);
                let article_boma2 = *(article2 as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
                ModelModule::set_joint_scale(article_boma2, Hash40::new("trans") , &Vector3f{x:-1.0,y:1.0,z:1.0});
            }
    } else { 
        // reverses the -ve scale (i am not sure if its actually needed but we ball ig)
        ModelModule::set_joint_scale(fighter.battle_object.module_accessor, Hash40::new("top") , &Vector3f{x:scale,y:scale,z:scale});
        if ArticleModule::is_exist(fighter.battle_object.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND){
            let article = ArticleModule::get_article(fighter.battle_object.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND);
            let article_boma = *(article as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
            ModelModule::set_joint_scale(article_boma, Hash40::new("hand1") , &Vector3f{x:1.0,y:1.0,z:1.0});
        }
        if ArticleModule::is_exist(fighter.battle_object.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON){
            let article2 = ArticleModule::get_article(fighter.battle_object.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON);
            let article_boma2 = *(article2 as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
            ModelModule::set_joint_scale(article_boma2, Hash40::new("trans") , &Vector3f{x:1.0,y:1.0,z:1.0});
        }
    }



    //let motion = MotionModule::motion_kind(fighter.battle_object.module_accessor);
    //let status = StatusModule::status_kind(fighter.battle_object.module_accessor);
    //println!("Status: {:#x}, Motion: {:#x}", status, motion);
}


// initialization vtable hook (using it as agent init since i cant use workmodule in agent init) (thanks wuboy)
#[skyline::from_offset(0xb5c360)]
pub unsafe extern "C" fn kamui_init_original(vtable: u64, fighter: &mut Fighter);

pub unsafe extern "C" fn kamui_init(vtable: u64, fighter: &mut Fighter) {
    println!("initialization hook successful");
    WorkModule::on_flag(fighter.battle_object.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
    kamui_init_original(vtable,fighter);
}


#[skyline::from_offset(0xb5c1a0)]
pub unsafe extern "C" fn kamui_shield_declare_og(vtable: u64, fighter: &mut Fighter);

pub unsafe extern "C" fn kamui_shield_declare(vtable: u64, fighter: &mut Fighter) {
    let module_accessor = (*fighter).battle_object.module_accessor;
    if (*fighter).battle_object.kind == *FIGHTER_KIND_KAMUI as u32 {
        let shield_data = ShieldDataResource::new(
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            13.0,
            Hash40::new("haver"),
            *COLLISION_SHAPE_TYPE_SPHERE as u8,
            *SHIELD_TYPE_GUARD as u8
        );
        let shield_datas = &mut (ShieldDatas::new().add(shield_data, 0));
        let resource = &mut ShieldGroupResource::new(
            shield_datas,
            1,
            1,
            false,
            false,
            true
        );
        
        MiscModule::add_shield_group(module_accessor, resource, 2);
    }
    //ShieldModule::set_target_category(module_accessor,0,*COLLISION_CATEGORY_MASK_NO_FIGHTER as i32);
    kamui_shield_declare_og(vtable,fighter);
}





#[skyline::from_offset(0xb63960)]
pub unsafe extern "C" fn kamui_on_shield_original(vtable: u64, fighter: &mut Fighter, collision_event: &mut ShieldAttackCollisionEvent);

pub unsafe extern "C" fn kamui_on_shield(vtable: u64, fighter: &mut Fighter, collision_event: &mut ShieldAttackCollisionEvent ) {

    let module_accessor = fighter.battle_object.module_accessor;
    let opponent_id =(*collision_event.collision_log).opponent_object_id;
    let opponent_cat =(*collision_event.collision_log).opponent_object_category;
    let opponent_raw_power =collision_event.raw_power;
    let opponent_real_power =collision_event.real_power;


    println!("on shield hook successful");
    println!("the opp id : {}", opponent_id );
    println!("the col kind category : {}", opponent_cat );
    println!("the raw power : {}", opponent_raw_power );
    println!("the real power : {}", opponent_real_power );

    if opponent_cat as i32 == *BATTLE_OBJECT_CATEGORY_WEAPON{
        if StatusModule::status_kind(module_accessor) == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD
        || StatusModule::status_kind(module_accessor) == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_SHOOT{
            WorkModule::on_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED);
            //WorkModule::set_float(module_accessor, opponent_real_power, WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLOAT_BOOM_POWER);
            //ArticleModule::generate_article(module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_RYUSENSYA, false, 0);
            ShieldModule::clear_all(module_accessor);
            WorkModule::set_float(module_accessor, opponent_real_power, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_REAL_POWER_REFLECT);
            ArticleModule::generate_article(module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_RYUSENSYA, false, -1);
            //let article = ArticleModule::get_article(module_accessor,*FIGHTER_KAMUI_GENERATE_ARTICLE_RYUSENSYA);
            //let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
            //println!("article id : {}", object_id );
            //WorkModule::set_float(sv_battle_object::module_accessor(object_id), opponent_real_power, WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLOAT_BOOM_POWER);
            StatusModule::change_status_request(module_accessor, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_SHOOT ,false);
        }
    }
    kamui_on_shield_original(vtable,fighter,collision_event);
}



