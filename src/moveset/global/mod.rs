use super::*;




mod caught;

unsafe extern "C" fn opff_global(fighter: &mut L2CFighterCommon) {
    let fighter_kind = fighter.global_table[KIND].get_i32();
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if !is_potemkin(fighter.module_accessor) {
        UiManager::disable_ui(get_ui_index_from_entry_id(entry_id));
    }
}

unsafe extern "C" fn agent_init_global(fighter: &mut L2CFighterCommon) {
    if smash::app::lua_bind::FighterManager::entry_count(singletons::FighterManager() as _) > 4 {
        UiManager::player_count_over_4(true);
    }else{
        UiManager::player_count_over_4(false);
    }
}



pub fn install(agent: &mut Agent) {
    agent.on_line(Main, opff_global);
    agent.on_start(agent_init_global);
    caught::install(agent);

}