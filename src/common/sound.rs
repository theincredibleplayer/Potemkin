use super::*;

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

//Used for sound randomization
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
    } 
    else {
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