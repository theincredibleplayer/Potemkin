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
    let status_kind =StatusModule::status_kind(fighter.battle_object.module_accessor) ;
    let situation_kind = StatusModule::situation_kind(module_accessor);
    let motion_kind =MotionModule::motion_kind(module_accessor);
    // scales the models of both pot and his articles by -1 on the x axis to mirror (accurate to strive)
    if ((PostureModule::lr(module_accessor) < 0.0) && (![*FIGHTER_STATUS_KIND_TURN,FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN].contains(&status_kind) && ((motion_kind != hash40("air_turn")) && (motion_kind != hash40("turn"))) ) )
    || ((PostureModule::lr(module_accessor) > 0.0) && ( [*FIGHTER_STATUS_KIND_TURN,FIGHTER_KAMUI_STATUS_KIND_AUTO_TURN].contains(&status_kind) && ((motion_kind == hash40("air_turn")) || (motion_kind == hash40("turn"))) ) )
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
    }

    
    let time_base = WorkModule::get_float(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FIRE_TIME_BASE);
    let agent = get_fighter_common_from_accessor((&mut*module_accessor));
    let fire_timer = WorkModule::get_int(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FIRE_TIMER);
    if (fire_timer > 0) {
        WorkModule::dec_int(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FIRE_TIMER);
        let fire_timer = WorkModule::get_int(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FIRE_TIMER);
        macros::BURN_COLOR_FRAME(agent, 0, 24, 1, 1, 0);
        macros::BURN_COLOR(agent, 24.0 * ( time_base/((fire_timer as f32)+1.0) ),  1.0 * ( time_base/((fire_timer as f32)+1.0) ),  1.0 * ( time_base/((fire_timer as f32)+1.0) ), (0.5/time_base)*(fire_timer as f32));
    }


    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    if smash::app::lua_bind::FighterManager::is_result_mode(fighter_manager) {
        UiManager::disable_ui(entry_id);
    }else{
        UiManager::enable_ui(entry_id);
    }
    UiManager::set_game_speed(sv_information::slow_rate()); 
    //println!("game_speed= {}",sv_information::slow_rate());


    WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_RUN);
    WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
    WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);

    let is_damage =StopModule::is_damage(module_accessor);

    let lr = get_opponent_lr(module_accessor,is_damage,status_kind,0);
    set_lr_float(lr,module_accessor,status_kind,0);

    let lr_1on1 = WorkModule::get_float(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    //println!("the lr float: {}",lr_1on1);
    let lr = PostureModule::lr(module_accessor);
    if lr_1on1 != 0.0 {
        if situation_kind == *SITUATION_KIND_AIR
        && MotionModule::motion_kind(module_accessor) != hash40("air_turn")
        && lr_1on1 != lr
        && [
            *FIGHTER_STATUS_KIND_JUMP,
            *FIGHTER_STATUS_KIND_JUMP_AERIAL,
        ].contains(&status_kind){
            PostureModule::reverse_lr(module_accessor);
            MotionModule::change_motion(module_accessor, Hash40::new("air_turn"), 0.0, 1.0, false, 0.0, false, false);
        }
    }

    ColorBlendModule::off_flash(module_accessor, false);
    COL_NORMAL(agent);
    /*
    if FighterCutInManager__is_one_on_one(){
        let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
        let entry_id = if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) == 0 {
            1
        }
        else {
            0
        };
        let entry = smash::app::lua_bind::FighterManager::get_fighter_entry(fighter_manager,FighterEntryID(entry_id));
        let battle_object = *(entry as *mut u64).add(0x4160/8) as *mut u64;
        let enemy = *battle_object.add(0x20/8) as *mut BattleObjectModuleAccessor;
        if PostureModule::pos_x(enemy) > PostureModule::pos_x(module_accessor) {
            WorkModule::set_float(module_accessor, 1.0,*FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
        }
        else {
            WorkModule::set_float(module_accessor, -1.0,*FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
        }
    }*/
    //let motion = MotionModule::motion_kind(module_accessor);
    //let status = StatusModule::status_kind(module_accessor);
    //println!("Status: {:#x}, Motion: {:#x}", status, motion);
}


// initialization vtable hook (using it as agent init since i cant use workmodule in agent init) (thanks wuboy)
#[skyline::from_offset(0xb5c360)]
pub unsafe extern "C" fn kamui_reset_original(vtable: u64, fighter: &mut Fighter);

pub unsafe extern "C" fn kamui_reset(vtable: u64, fighter: &mut Fighter) {
    let module_accessor = (*fighter).battle_object.module_accessor;
    println!("initialization hook successful");
    WorkModule::on_flag(fighter.battle_object.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
    ShieldModule::set_target_property(fighter.battle_object.module_accessor, *COLLISION_PROPERTY_REFLECT,2);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    UiManager::reset_on_rebirth(entry_id);
    UiManager::reset_cutin();
    WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_RUN);
    WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
    WorkModule::enable_transition_term_forbid(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);

    kamui_reset_original(vtable,fighter);
}


#[skyline::from_offset(0xb5c1a0)]
pub unsafe extern "C" fn kamui_init_og(vtable: u64, fighter: &mut Fighter);

pub unsafe extern "C" fn kamui_init(vtable: u64, fighter: &mut Fighter) {
    let module_accessor = (*fighter).battle_object.module_accessor;


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
    let status_kind =StatusModule::status_kind(fighter.battle_object.module_accessor);

    if [*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL,
        *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_END,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
        *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK,
        ].contains(&status_kind)
    && WorkModule::is_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ARMOR_ON) {
        let time_base = 45.0 * (1.0/sv_information::slow_rate());
        WorkModule::set_float(module_accessor, time_base, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_FIRE_TIME_BASE);
        WorkModule::set_int(module_accessor, time_base as i32, FIGHTER_KAMUI_INSTANCE_WORK_ID_INT_FIRE_TIMER);
        let agent = get_fighter_common_from_accessor((&mut*module_accessor));
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0); 
        macros::BURN_COLOR(agent, 24,  1,  1, 0.5);
        WorkModule::off_flag(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_IS_ARMOR_ON);
    }
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
    println!("unk22 active");
    kamui_on_frame_2_original(vtable,fighter);
}

#[skyline::from_offset(0xB5CDB0)]
pub unsafe extern "C" fn kamui_on_frame_1_original(vtable: u64, fighter: &mut Fighter);

pub unsafe extern "C" fn kamui_on_frame_1(vtable: u64, fighter: &mut Fighter) {
    println!("on_final_end active");
    //kamui_on_frame_3_original(vtable,fighter);
}