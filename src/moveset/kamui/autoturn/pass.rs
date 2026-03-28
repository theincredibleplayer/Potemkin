use super::*;





pub unsafe extern "C" fn pass_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    println!("pass exec");
    fighter.sub_uniq_process_Pass_exec_status();
    if WorkModule::is_flag(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLAG_COMMAND_EXTEND){
        println!("less than 15");
        let speed_y = WorkModule::get_float(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLOAT_SPEED_Y);
        let offset_y = WorkModule::get_float(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLOAT_OFFSET_Y);
        let air_accel_y = -WorkModule::get_param_float(module_accessor, hash40("air_accel_y"),0);
        println!("air accel y = {}",air_accel_y);
        let offset = air_accel_y + offset_y;
        println!("offset = {}",offset);
        GroundModule::set_offset_y(module_accessor, offset);
        WorkModule::set_float(module_accessor, offset, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLOAT_SPEED_Y);
        WorkModule::set_float(module_accessor, offset, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLOAT_OFFSET_Y);
        WorkModule::dec_int(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_INT_FRAME);
    }else{
        println!("more than 15");
    }

    return L2CValue::I32(0)
}


pub unsafe extern "C" fn pass_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    GroundModule::set_offset_y(module_accessor, 0.0);
    if WorkModule::is_flag(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLAG_COMMAND_EXTEND){
        println!("less than 15 end");

        fighter.end_pass_ground();
        let speed_y = WorkModule::get_float(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLOAT_SPEED_Y);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
        sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    }
    fighter.status_end_Pass();
    return L2CValue::I32(0)
}


pub unsafe extern "C" fn pass_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    fighter.status_Pass_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(pass_main_loop as *const () as _))
}


pub unsafe extern "C" fn pass_function_thingy(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    if fighter.sub_transition_group_check_special_command().get_bool(){
        WorkModule::off_flag(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLAG_COMMAND_EXTEND);
    }else{
        if !WorkModule::is_flag(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLAG_COMMAND_EXTEND){
            let value = 8; //WorkModule::get_param_float(module_accessor, hash40("param_private"),hash40("pass_cancel_command_frame"));
            WorkModule::set_int(module_accessor, value, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_INT_FRAME);
            WorkModule::on_flag(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLAG_COMMAND_EXTEND);
            //return L2CValue::I32(1)
        }
        let frame = WorkModule::get_int(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_INT_FRAME);
        if !(frame == 0) {
            return L2CValue::I32(1);
        }
        GroundModule::set_offset_y(module_accessor, 0.0);
        WorkModule::off_flag(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLAG_COMMAND_EXTEND);
        fighter.end_pass_ground();
        let speed_y = WorkModule::get_float(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_STATUS_WORK_ID_PASS_FLOAT_SPEED_Y);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
        sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        return L2CValue::I32(0);
    }
    return L2CValue::I32(1);
}


pub unsafe extern "C" fn pass_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Pass_Main_sub(L2CValue::Ptr(pass_function_thingy as *const () as _));
    return L2CValue::I32(0)
}


pub fn install(agent: &mut Agent) {

    agent.status(Main,  *FIGHTER_STATUS_KIND_PASS, pass_main);
    agent.status(Exec,  *FIGHTER_STATUS_KIND_PASS, pass_exec);
    agent.status(End,   *FIGHTER_STATUS_KIND_PASS, pass_end);

}