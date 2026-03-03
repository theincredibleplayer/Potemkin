#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![allow(warnings)]
#![feature(simd_ffi)]
use std::collections::HashMap;
use smash::lib::lua_const::{FIGHTER_KIND_KAMUI};
use motion_lib::{self, mlist::{Animation, Extra}, hash40::WriteHash40};
use arcropolis_api::arc_callback;
use motion_lib::{hash40, mlist::Flags};
use arcropolis_api::hash40;

pub static mut FIGHTER_MANAGER: usize = 0;
pub static mut FIGHTER_CUTIN_MANAGER_ADDR: usize = 0;
pub static mut ITEM_MANAGER : usize = 0;


mod moveset;


pub static mut MARKED_COLORS: [bool; 256] = [false; 256];








pub fn check_deps() -> bool {
    let mut passed = true;

    for dep in [
        "rom:/skyline/plugins/libparam_config.nro",
        "rom:/skyline/plugins/libthe_csk_collection.nro",
        "rom:/skyline/plugins/libarcropolis.nro",
        "rom:/skyline/plugins/libnro_hook.nro",
        "rom:/skyline/plugins/libsmashline_plugin.nro",
    ] {
        if !std::path::Path::new(dep).is_file() {
            println!("{} not found! This installation is incomplete. Please download all dependencies listed in the README file.", dep);
            passed = false;
        }
    }

    passed
}

extern "C" fn mods_mounted(_ev: arcropolis_api::Event) {
    // EVERYTHING HERE IS UNIQUE THAT'S MEANT TO BE MODIFIED FOR YOUR SPECIFIC PLUGIN.

    const FIGHTER_NAME: &str = "kamui";
    const MARKER_FILE: &str = "potemkin.marker";
    let mut lowest_color: i32 = -1;
    let mut marked_slots: Vec<i32> = vec![];
    for x in 0..256 {
        if let Ok(_) = std::fs::read(format!(
            "mods:/fighter/{}/model/body/c{:02}/{}",
            FIGHTER_NAME, x, MARKER_FILE
        )) {
            unsafe {
                marked_slots.push(x as _);
                MARKED_COLORS[x as usize] = true;
                if lowest_color == -1 {
                    lowest_color = x as _ ;
                }
            }
        }
    }


    if lowest_color == -1 {
        // if no marker exist, leave
        return;
    }

    let color_num = {
        unsafe {
            let mut index = lowest_color;
            while index < 256 && MARKED_COLORS[index as usize] {
                index += 1;
            }
            index - lowest_color
        }
    };


    param_config::update_float_2(*FIGHTER_KIND_KAMUI, marked_slots.clone(), (smash::hash40("param_private"), 0x19996b81b6, 1200.0));




    // Preform csk collection calls here (allow online, narration characall entry, add chara db, add layout db, w/e)

    the_csk_collection_api::add_chara_db_entry_info(the_csk_collection_api::CharacterDatabaseEntry {
        ui_chara_id: smash::hash40("ui_chara_potemkin"),
        name_id: the_csk_collection_api::StringType::Optional(Some(the_csk_collection_api::CStrCSK::new("potemkin"))), 
        fighter_kind: the_csk_collection_api::Hash40Type::Optional(Some(0x12C30E8564 /* Hash40 of fighter_kind_kamui */)), 
        fighter_kind_corps: the_csk_collection_api::Hash40Type::Optional(Some(0x12C30E8564 /* Hash40 of fighter_kind_kamui */)), 
        ui_series_id: the_csk_collection_api::Hash40Type::Optional(Some(0x142be617be)), 
        fighter_type: the_csk_collection_api::Hash40Type::Optional(Some(0x1353795179 /* Hash40 of fighter_type_normal */)), 
        alt_chara_id: the_csk_collection_api::Hash40Type::Optional(Some(0x0)), 
        exhibit_year: the_csk_collection_api::ShortType::Optional(Some(2015)), 
        exhibit_day_order: the_csk_collection_api::IntType::Optional(Some(62501)), 
        ext_skill_page_num: the_csk_collection_api::SignedByteType::Optional(Some(0)), 
        is_img_ext_skill_page0: the_csk_collection_api::BoolType::Optional(Some(false)), 
        is_img_ext_skill_page1: the_csk_collection_api::BoolType::Optional(Some(false)), 
        is_img_ext_skill_page2: the_csk_collection_api::BoolType::Optional(Some(false)), 
        skill_list_order: the_csk_collection_api::SignedByteType::Optional(Some(67)), 
        disp_order: the_csk_collection_api::SignedByteType::Optional(Some(84)), 
        save_no: the_csk_collection_api::SignedByteType::Optional(Some(62)), 
        chara_count: the_csk_collection_api::SignedByteType::Optional(Some(1)), 
        can_select: the_csk_collection_api::BoolType::Optional(Some(true)), 
        is_usable_soundtest: the_csk_collection_api::BoolType::Optional(Some(true)), 
        is_called_pokemon: the_csk_collection_api::BoolType::Optional(Some(false)), 
        is_mii: the_csk_collection_api::BoolType::Optional(Some(false)), 
        is_boss: the_csk_collection_api::BoolType::Optional(Some(false)), 
        is_hidden_boss: the_csk_collection_api::BoolType::Optional(Some(false)), 
        is_dlc: the_csk_collection_api::BoolType::Optional(Some(false)), 
        is_patch: the_csk_collection_api::BoolType::Optional(Some(false)), 
        is_plural_message: the_csk_collection_api::BoolType::Optional(Some(false)), 
        is_plural_narration: the_csk_collection_api::BoolType::Optional(Some(false)), 
        is_article: the_csk_collection_api::BoolType::Optional(Some(false)), 
        extra_flags: the_csk_collection_api::IntType::Optional(Some(0)), 
        result_pf0: the_csk_collection_api::BoolType::Optional(Some(true)), 
        result_pf1: the_csk_collection_api::BoolType::Optional(Some(true)), 
        result_pf2: the_csk_collection_api::BoolType::Optional(Some(true)), 
        color_num: the_csk_collection_api::UnsignedByteType::Optional(Some(17)), 
        shop_item_tag: the_csk_collection_api::Hash40Type::Optional(Some(0x0)), 
        extra_hash_maps: the_csk_collection_api::Hash40Map::Overwrite(HashMap::from([
            (0x1337FC912E /* Hash40 of characall_label_c00 */, the_csk_collection_api::Hash40Type::Optional(Some(smash::hash40("vc_narration_characall_potemkin")))), 
            (0x1340FBA1B8 /* Hash40 of characall_label_c01 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
            (0x13D9F2F002 /* Hash40 of characall_label_c02 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
            (0x13AEF5C094 /* Hash40 of characall_label_c03 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
            (0x1330915537 /* Hash40 of characall_label_c04 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
            (0x13479665A1 /* Hash40 of characall_label_c05 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
            (0x13DE9F341B /* Hash40 of characall_label_c06 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
            (0x13A998048D /* Hash40 of characall_label_c07 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
            (0x1B8B13E500 /* Hash40 of characall_label_article_c00 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
            (0x1BFC14D596 /* Hash40 of characall_label_article_c01 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
            (0x1B651D842C /* Hash40 of characall_label_article_c02 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
            (0x1B121AB4BA /* Hash40 of characall_label_article_c03 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
            (0x1B8C7E2119 /* Hash40 of characall_label_article_c04 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
            (0x1BFB79118F /* Hash40 of characall_label_article_c05 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
            (0x1B62704035 /* Hash40 of characall_label_article_c06 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
            (0x1B157770A3 /* Hash40 of characall_label_article_c07 */, the_csk_collection_api::Hash40Type::Optional(Some(0x0))), 
            (0x160ab9eb98, the_csk_collection_api::Hash40Type::Optional(Some(0xE4DDD21E6 /* Hash40 of ui_chara_kamui */))),
        ])), 
        extra_index_maps: the_csk_collection_api::UnsignedByteMap::Overwrite(HashMap::from([
            (0x915C075DE /* Hash40 of c00_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x9B3B77E6A /* Hash40 of c01_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x9825F64F7 /* Hash40 of c02_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x924286F43 /* Hash40 of c03_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x9E18F51CD /* Hash40 of c04_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x947F85A79 /* Hash40 of c05_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x9761040E4 /* Hash40 of c06_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x9D0674B50 /* Hash40 of c07_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x9E48F9289 /* Hash40 of n00_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x942F8993D /* Hash40 of n01_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x9731083A0 /* Hash40 of n02_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x9D5678814 /* Hash40 of n03_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x910C0B69A /* Hash40 of n04_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x9B6B7BD2E /* Hash40 of n05_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x9875FA7B3 /* Hash40 of n06_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x92128AC07 /* Hash40 of n07_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x9F873561A /* Hash40 of c00_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x95E045DAE /* Hash40 of c01_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x96FEC4733 /* Hash40 of c02_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x9C99B4C87 /* Hash40 of c03_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x90C3C7209 /* Hash40 of c04_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x9AA4B79BD /* Hash40 of c05_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x99BA36320 /* Hash40 of c06_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x93DD46894 /* Hash40 of c07_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
            (0x11895f00fc, the_csk_collection_api::UnsignedByteType::Optional(Some(lowest_color as _))),
        ])), 
        ..Default::default()
    });
    the_csk_collection_api::add_chara_layout_db_entry_info(the_csk_collection_api::CharacterLayoutDatabaseEntry {
        ui_layout_id: smash::hash40("ui_chara_potemkin_00"), 
        ui_chara_id: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_chara_potemkin")), 
        chara_color: the_csk_collection_api::UnsignedByteType::Optional(Some(0)), 
        eye_0_flash_count: the_csk_collection_api::UnsignedByteType::Optional(Some(2)), 
        eye_0_flash0_pos_x: the_csk_collection_api::FloatType::Optional(Some(-88.0)), 
        eye_0_flash0_pos_y: the_csk_collection_api::FloatType::Optional(Some(232.0)), 
        eye_0_flash1_pos_x: the_csk_collection_api::FloatType::Optional(Some(62.0)), 
        eye_0_flash1_pos_y: the_csk_collection_api::FloatType::Optional(Some(220.0)), 
        eye_0_flash2_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        eye_0_flash2_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        eye_0_flash3_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        eye_0_flash3_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        eye_0_flash4_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        eye_0_flash4_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        eye_1_flash_count: the_csk_collection_api::UnsignedByteType::Optional(Some(2)), 
        eye_1_flash0_pos_x: the_csk_collection_api::FloatType::Optional(Some(-52.0)), 
        eye_1_flash0_pos_y: the_csk_collection_api::FloatType::Optional(Some(218.0)), 
        eye_1_flash1_pos_x: the_csk_collection_api::FloatType::Optional(Some(78.0)), 
        eye_1_flash1_pos_y: the_csk_collection_api::FloatType::Optional(Some(210.0)), 
        eye_1_flash2_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        eye_1_flash2_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        eye_1_flash3_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        eye_1_flash3_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        eye_1_flash4_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        eye_1_flash4_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        eye_2_flash_count: the_csk_collection_api::UnsignedByteType::Optional(Some(2)), 
        eye_2_flash0_pos_x: the_csk_collection_api::FloatType::Optional(Some(-27.0)), 
        eye_2_flash0_pos_y: the_csk_collection_api::FloatType::Optional(Some(93.0)), 
        eye_2_flash1_pos_x: the_csk_collection_api::FloatType::Optional(Some(69.0)), 
        eye_2_flash1_pos_y: the_csk_collection_api::FloatType::Optional(Some(85.0)), 
        eye_2_flash2_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        eye_2_flash2_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        eye_2_flash3_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        eye_2_flash3_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        eye_2_flash4_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        eye_2_flash4_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        eye_flash_info_pos_x: the_csk_collection_api::FloatType::Optional(Some(27.0)), 
        eye_flash_info_pos_y: the_csk_collection_api::FloatType::Optional(Some(2.0)), 
        spirits_eye_visible: the_csk_collection_api::BoolType::Optional(Some(true)), 
        chara_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(-6.0)), 
        chara_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(-62.0)), 
        chara_1_scale: the_csk_collection_api::FloatType::Optional(Some(1.02)), 
        chara_1_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(-9.0)), 
        chara_1_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(-66.0)), 
        chara_1_1_scale: the_csk_collection_api::FloatType::Optional(Some(1.39)), 
        chara_1_2_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        chara_1_2_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        chara_1_2_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
        chara_1_3_offset_x: the_csk_collection_api::FloatType::Optional(Some(-9.0)), 
        chara_1_3_offset_y: the_csk_collection_api::FloatType::Optional(Some(-40.0)), 
        chara_1_3_scale: the_csk_collection_api::FloatType::Optional(Some(1.39)), 
        chara_1_4_offset_x: the_csk_collection_api::FloatType::Optional(Some(-9.0)), 
        chara_1_4_offset_y: the_csk_collection_api::FloatType::Optional(Some(-40.0)), 
        chara_1_4_scale: the_csk_collection_api::FloatType::Optional(Some(1.39)), 
        chara_1_5_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        chara_1_5_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        chara_1_5_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
        chara_3_0_offset_x: the_csk_collection_api::FloatType::Optional(Some(-499.0)), 
        chara_3_0_offset_y: the_csk_collection_api::FloatType::Optional(Some(-153.0)), 
        chara_3_0_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
        chara_3_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(-439.0)), 
        chara_3_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(-133.0)), 
        chara_3_1_scale: the_csk_collection_api::FloatType::Optional(Some(0.9)), 
        chara_3_2_offset_x: the_csk_collection_api::FloatType::Optional(Some(-260.0)), 
        chara_3_2_offset_y: the_csk_collection_api::FloatType::Optional(Some(-60.0)), 
        chara_3_2_scale: the_csk_collection_api::FloatType::Optional(Some(0.76)), 
        chara_3_3_offset_x: the_csk_collection_api::FloatType::Optional(Some(-600.0)), 
        chara_3_3_offset_y: the_csk_collection_api::FloatType::Optional(Some(-153.0)), 
        chara_3_3_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
        chara_3_4_offset_x: the_csk_collection_api::FloatType::Optional(Some(-499.0)), 
        chara_3_4_offset_y: the_csk_collection_api::FloatType::Optional(Some(-153.0)), 
        chara_3_4_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
        chara_3_5_offset_x: the_csk_collection_api::FloatType::Optional(Some(-465.0)), 
        chara_3_5_offset_y: the_csk_collection_api::FloatType::Optional(Some(-183.0)), 
        chara_3_5_scale: the_csk_collection_api::FloatType::Optional(Some(0.96)), 
        chara_3_6_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        chara_3_6_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        chara_3_6_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
        chara_3_7_offset_x: the_csk_collection_api::FloatType::Optional(Some(-499.0)), 
        chara_3_7_offset_y: the_csk_collection_api::FloatType::Optional(Some(-153.0)), 
        chara_3_7_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
        chara_5_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        chara_5_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        chara_5_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
        chara_select_icon_list_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        chara_select_icon_list_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        chara_select_icon_list_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
        chara_7_0_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        chara_7_0_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        chara_7_0_scale: the_csk_collection_api::FloatType::Optional(Some(0.94)), 
        chara_7_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        chara_7_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        chara_7_1_scale: the_csk_collection_api::FloatType::Optional(Some(0.94)), 
        chara_0_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        chara_0_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
        chara_0_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
        ..Default::default()
    });
    //the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_potemkin");



    moveset::install();
}


#[arc_callback]
fn my_callback(hash: u64, data: &mut [u8]) -> Option<usize> {
    arcropolis_api::load_original_file(hash, &mut *data);
    let mut reader = std::io::Cursor::new(&mut *data);
    match motion_lib::read_stream(&mut reader) {
        Ok(mut motion_list) => {
            let motions_to_add = [
                "catched_pot_special_hi","catched_pot_special_hi_girl","catched_pot_special_hi_big","catched_pot_special_hi_dx",
                "catched_potemkin_buster","catched_potemkin_buster_girl","catched_potemkin_buster_big","catched_potemkin_buster_dx",
                "catched_pot_final_scene_01","catched_pot_final_scene_01_girl","catched_pot_final_scene_01_big","catched_pot_final_scene_01_dx",
                "catched_pot_final_scene_02","catched_pot_final_scene_02_girl","catched_pot_final_scene_02_big","catched_pot_final_scene_02_dx",
                "catched_pot_final_scene_03","catched_pot_final_scene_03_girl","catched_pot_final_scene_03_big","catched_pot_final_scene_03_dx"
            ];
            for entry in motions_to_add{
                let script_name = entry.replace("_", "").replace("girl", "").replace("big", "").replace("dx", "");
                let new_entry_name = motion_lib::hash40::hash40(entry);
                let extras = std::option::Option::Some(motion_lib::mlist::Extra{
                    xlu_start:0,
                    xlu_end:0,
                    cancel_frame:0,
                    no_stop_intp:false,
                });
                let new_motion = motion_lib::mlist::Motion {
                    game_script: motion_lib::hash40::hash40(["game_",script_name.as_str()].join("").as_str()),
                    flags: Flags{
                        turn: false,
                        r#loop: false,
                        r#move: false,
                        fix_trans: true,
                        fix_rot: false,
                        fix_scale: true,
                        unk_40: false,
                        unk_80: true,
                        unk_100: false,
                        unk_200: false,
                        unk_400: false,
                        unk_800: false,
                        unk_1000: false,
                        unk_2000: false
                    },
                    blend_frames: 0,
                    animations: vec![],
                    scripts: vec![
                        motion_lib::hash40::hash40(["expression_",script_name.as_str()].join("").as_str()),
                        motion_lib::hash40::hash40(["effect_",script_name.as_str()].join("").as_str()),
                        motion_lib::hash40::hash40(["sound_",script_name.as_str()].join("").as_str())
                    ],
                    extra: extras,
                };
                motion_list.list.insert(new_entry_name, new_motion);
            }
            let mut writer = std::io::Cursor::new(Vec::<u8>::new());
            motion_lib::write_stream(&mut writer, &motion_list).unwrap();
            let vec_res = writer.into_inner();
            let slice = vec_res.as_slice();
            data[..slice.len()].copy_from_slice(slice);
            Some(data.len())
        }
        Err(_) => {
            println!("Failed to patch {:#x}", hash);
            None
        },
    }
}


#[skyline::main(name = "hotpot")]
pub fn main() {
    if !check_deps() {
        return;
    }
    const MAX_FILE_SIZE: usize = 0x10000;
    pub const fighters: [&str; 97] = ["bayonetta", "brave", "buddy", "captain", "chrom", "cloud", "daisy", "dedede", "demon", "diddy", "dolly", "donkey", "duckhunt", "edge", "eflame", "element", "elight", "falco", "fox", "gamewatch", "ganon", "gaogaen", "gekkouga", "ike", "inkling", "jack", "kamui", "ken", "kirby", "koopa", "koopag", "koopajr", "krool", "link", "littlemac", "lucario", "lucas", "lucina", "luigi", "mario", "mariod", "marth", "master", "metaknight", "mewtwo", "miienemyf", "miienemyg", "miienemys", "miifighter", "miigunner", "miiswordsman", "murabito", "nana", "ness", "packun", "pacman", "palutena", "peach", "pfushigisou", "pichu", "pickel", "pikachu", "pikmin", "pit", "pitb", "plizardon", "popo", "ptrainer", "ptrainer_low", "purin", "pzenigame", "reflet", "richter", "ridley", "robot", "rockman", "rosetta", "roy", "ryu", "samus", "samusd", "sheik", "shizue", "shulk", "simon", "snake", "sonic", "szerosuit", "tantan", "toonlink", "trail", "wario", "wiifit", "wolf", "yoshi", "younglink", "zelda"];
    for fighter in fighters.iter() {
        for color in 0..256 {
            unsafe{
                if !(*fighter == "kamui" && MARKED_COLORS[color as usize]){
                    my_callback::install(arcropolis_api::hash40(&format!("fighter/{}/motion/body/c{:02}/motion_list.bin", fighter, color)), MAX_FILE_SIZE);
                }
            }
        }
    }
    
    unsafe {
        extern "C" {
            fn arcrop_register_event_callback(
                ty: arcropolis_api::Event,
                callback: arcropolis_api::EventCallbackFn,
            );
        }
        arcrop_register_event_callback(arcropolis_api::Event::ModFilesystemMounted, mods_mounted);
    }
}