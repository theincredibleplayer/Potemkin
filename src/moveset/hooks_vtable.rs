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
use crate::ITEM_MANAGER;
use skyline::hooks::{getRegionAddress, Region};


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
    if StatusModule::status_kind(module_accessor) == FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_LOOP {
        let collision_log = *(log as *const u64).add(0x10 / 0x8);
        let collision_log = collision_log as *const CollisionLogScuffed;
        let opp_boma = sv_battle_object::module_accessor((*collision_log).opponent_object_id);
        StatusModule::change_status_request_from_script(module_accessor, FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_ATTACK, false);
    }
    if StatusModule::status_kind(module_accessor) == FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_LOOP {
        let collision_log = *(log as *const u64).add(0x10 / 0x8);
        let collision_log = collision_log as *const CollisionLogScuffed;
        let opp_boma = sv_battle_object::module_accessor((*collision_log).opponent_object_id);
        StatusModule::change_status_request_from_script(module_accessor, FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_ATTACK, false);
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



    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    //UiManager::add_tension(entry_id, 0.1);
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    if smash::app::lua_bind::FighterManager::is_result_mode(fighter_manager) {
        UiManager::disable_ui(entry_id);
    }


/*
    if StatusModule::status_kind(fighter.battle_object.module_accessor) == *FIGHTER_STATUS_KIND_FINAL
    || StatusModule::status_kind(fighter.battle_object.module_accessor) == *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK
    || StatusModule::status_kind(fighter.battle_object.module_accessor) == *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK_WAIT
    || StatusModule::status_kind(fighter.battle_object.module_accessor) == *FIGHTER_KAMUI_STATUS_KIND_FINAL_END{
        let capture_id = LinkModule::get_node_object_id(module_accessor, *LINK_NO_CAPTURE);
        if capture_id != 0x50000000 {
            let enemy = sv_battle_object::module_accessor(capture_id as u32);
            let hight = WorkModule::get_param_float(enemy, hash40("height"), 0);
            let motion_share =   WorkModule::get_int(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FINAL_MOTION_SHARE);

            let mut base_hight = 22.0;
            if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
                base_hight = 15.0;
            }
            else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
                base_hight = 17.0;
            }
            else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_BIG {
                base_hight = 19.0;
            } 
            //let scale = PostureModule::scale(module_accessor);
        
            let base_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
            let mut position = ModelModule::joint_global_position(module_accessor, Hash40::new("throw"), base_pos, true);
            println!("global x: {}\nglobal y: {} \nglobal z: {}",position.x,position.y,position.z);
            println!("base x: {}\nbase y: {} \nbase z: {}",base_pos.x,base_pos.y,base_pos.z);
            println!("hight :{}",hight);
            println!("base hight :{}",base_hight);
            //println!("scale :{}",scale);
            let mut offset_y = 11.0;
            let mut z_mul = 1.0;
            if StatusModule::status_kind(fighter.battle_object.module_accessor) == *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK{
                offset_y = 1511.0;
                z_mul = -1.0;
            }


            let mut final_pos = Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
            final_pos.x = base_pos.x - (base_hight - hight);
            final_pos.y = base_pos.y - offset_y;
            final_pos.z = base_pos.z * z_mul;
            ModelModule::set_joint_translate(module_accessor, Hash40::new("throw"), &mut final_pos, false, false);
        
        }
    }

*/
    let motion = MotionModule::motion_kind(fighter.battle_object.module_accessor);
    let status = StatusModule::status_kind(fighter.battle_object.module_accessor);
    println!("Status: {:#x}, Motion: {:#x}", status, motion);
}


// initialization vtable hook (using it as agent init since i cant use workmodule in agent init) (thanks wuboy)
#[skyline::from_offset(0xb5c360)]
pub unsafe extern "C" fn kamui_reset_original(vtable: u64, fighter: &mut Fighter);

pub unsafe extern "C" fn kamui_reset(vtable: u64, fighter: &mut Fighter) {
    let module_accessor = (*fighter).battle_object.module_accessor;
    println!("initialization hook successful");
    WorkModule::on_flag(fighter.battle_object.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
    //ShieldModule::set_target_category(fighter.battle_object.module_accessor,*COLLISION_CATEGORY_WEAPON as i32,2); both work ig but property is more broad 
    ShieldModule::set_target_property(fighter.battle_object.module_accessor, *COLLISION_PROPERTY_REFLECT,2);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    UiManager::reset_on_rebirth(entry_id);
    UiManager::reset_cutin();
    kamui_reset_original(vtable,fighter);
}


#[skyline::from_offset(0xb5c1a0)]
pub unsafe extern "C" fn kamui_init_og(vtable: u64, fighter: &mut Fighter);

pub unsafe extern "C" fn kamui_init(vtable: u64, fighter: &mut Fighter) {
    let module_accessor = (*fighter).battle_object.module_accessor;
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    UiManager::enable_ui(entry_id);
    println!("entry id :{}", entry_id);
    if (*fighter).battle_object.kind == *FIGHTER_KIND_KAMUI as u32 {
        let shield_data = ShieldDataResource::new(
            0.0,
            7.0,
            11.0,
            0.0,
            17.0,
            11.0,
            8.0,
            Hash40::new("top"),
            *COLLISION_SHAPE_TYPE_CAPSULE as u8,
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
    kamui_init_og(vtable,fighter);
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
    let opponent_accessor = sv_battle_object::module_accessor(opponent_id);
    let opponent_common = get_fighter_common_from_accessor(&mut* opponent_accessor);



    if opponent_cat as i32 == *BATTLE_OBJECT_CATEGORY_WEAPON
    || opponent_cat as i32 == *BATTLE_OBJECT_CATEGORY_ITEM{
        if StatusModule::status_kind(module_accessor) == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD
        || StatusModule::status_kind(module_accessor) == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_SHOOT{
            if opponent_cat as i32 == *BATTLE_OBJECT_CATEGORY_ITEM {
                let item_manager = *(ITEM_MANAGER as *mut *mut smash::app::ItemManager);
                smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager,opponent_id);
            }
            WorkModule::on_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_SPECIAL_N_HAS_REFLECTED);
            notify_event_msc_cmd!(opponent_common, Hash40::new_raw(0x199c462b5d));
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


#[skyline::from_offset(0xb5e710)]
pub unsafe extern "C" fn kamui_on_damage_og(vtable: u64, fighter: &mut Fighter,damage_log:u64);

pub unsafe extern "C" fn kamui_on_damage(vtable: u64, fighter: &mut Fighter,damage_log:u64) {
    println!("on damage hook successful");
    let module_accessor = (*fighter).battle_object.module_accessor;

    let module_accessor = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let dmg = *((*(((damage_log as u64) + 0x10) as *const u64) + 0x4) as *const f32);

    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    UiManager::add_tension(entry_id, dmg);
    kamui_on_damage_og(vtable,fighter,damage_log);
}

#[skyline::from_offset(0xb5e240)]
pub unsafe extern "C" fn kamui_on_attack_og(vtable: u64, fighter: &mut Fighter,collision_log:CollisionLog);

pub unsafe extern "C" fn kamui_on_attack(vtable: u64, fighter: &mut Fighter,collision_log:CollisionLog) {
    println!("on hit hook successful");
    let attack_data = AttackModule::attack_data(fighter.battle_object.module_accessor, collision_log.collider_id as i32, false);
    let module_accessor = (*fighter).battle_object.module_accessor;
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status = StatusModule::status_kind(module_accessor);
    if 
    ![FIGHTER_KAMUI_STATUS_KIND_ROMAN_CANCEL,
    FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_ATTACK,
    FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_LOOP,
    FIGHTER_KAMUI_STATUS_KIND_FINAL_DASH_START,
    *FIGHTER_KAMUI_STATUS_KIND_FINAL_END,
    FIGHTER_KAMUI_STATUS_KIND_FINAL_JUMP,
    FIGHTER_KAMUI_STATUS_KIND_FINAL_FAKE_VISSCENE,
    *FIGHTER_KAMUI_STATUS_KIND_FINAL_READY,
    *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK,
    *FIGHTER_KAMUI_STATUS_KIND_FINAL_SCENE_ATTACK_WAIT,
    *FIGHTER_STATUS_KIND_FINAL].contains(&status){
    UiManager::add_tension(entry_id, (*attack_data).power);
    }
    kamui_on_attack_og(vtable,fighter,collision_log);
}


// testing section ==============================================================================



#[skyline::from_offset(0xb5caf0)]
pub unsafe extern "C" fn kamui_reset_2_original(vtable: u64, fighter: &mut Fighter);

pub unsafe extern "C" fn kamui_reset_2(vtable: u64, fighter: &mut Fighter) {
    println!("reset 2 active");
    kamui_reset_2_original(vtable,fighter);
}

#[skyline::from_offset(0x68D600)]
pub unsafe extern "C" fn kamui_reset_3_original(vtable: u64, fighter: &mut Fighter);

pub unsafe extern "C" fn kamui_reset_3(vtable: u64, fighter: &mut Fighter) {
    println!("reset 3 active");
    //kamui_reset_3_original(vtable,fighter);
}


#[skyline::from_offset(0x68D660)]
pub unsafe extern "C" fn kamui_on_frame_2_original(vtable: u64, fighter: &mut Fighter);

pub unsafe extern "C" fn kamui_on_frame_2(vtable: u64, fighter: &mut Fighter) {
    println!("on frame 2 active");
    kamui_on_frame_2_original(vtable,fighter);
}

#[skyline::from_offset(0xB5CDB0)]
pub unsafe extern "C" fn kamui_on_frame_1_original(vtable: u64, fighter: &mut Fighter);

pub unsafe extern "C" fn kamui_on_frame_1(vtable: u64, fighter: &mut Fighter) {
    println!("on_frame_1 active");
    //kamui_on_frame_3_original(vtable,fighter);
}