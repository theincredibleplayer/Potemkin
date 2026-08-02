use super::*;



unsafe extern "C" fn lose_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    let pos = *PostureModule::pos(module_accessor);
    let rot = *(PostureModule::rot(module_accessor,0) as *const Vector3f);
    WorkModule::set_float(module_accessor, pos.x, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_LOSE_POS_X);
    WorkModule::set_float(module_accessor, pos.y, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_LOSE_POS_Y);
    WorkModule::set_float(module_accessor, pos.z, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_LOSE_POS_Z);

    if pos.x >= 1000.0{
        let new_pos = Vector3f{x:pos.x,y:8.0,z:pos.z};
        let new_rot = Vector3f{x:0.0,y:45.0,z:0.0};
        let scale = PostureModule::scale(module_accessor);
        PostureModule::set_scale(module_accessor, 0.45, false);
        PostureModule::set_pos(module_accessor, &new_pos);
        PostureModule::set_rot(module_accessor, &new_rot, 0);
    }
    //return 0.into();

    fighter.sub_shift_status_main(L2CValue::Ptr(lose_main_loop as *const () as _))
}
unsafe extern "C" fn lose_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.module_accessor;
    //let pos_x = WorkModule::get_float(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_LOSE_POS_X);
    //let pos_y = WorkModule::get_float(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_LOSE_POS_Y);
    //let pos_z = WorkModule::get_float(module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_LOSE_POS_Z);
    let pos = *PostureModule::pos(module_accessor);

    if pos.x >= 1000.0{
        let new_pos = Vector3f{x:pos.x,y:8.0,z:pos.z};
        //let new_pos = Vector3f{x:pos_x,y:pos_y+5.0,z:pos_z};
        let new_rot = Vector3f{x:0.0,y:40.0,z:0.0};

        PostureModule::set_pos(module_accessor, &new_pos);
        PostureModule::set_scale(module_accessor, 0.45, false);
        PostureModule::set_rot(module_accessor, &new_rot, 0);
    }
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_DEMO, lose_main);
}