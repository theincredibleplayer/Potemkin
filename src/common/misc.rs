use super::*;

pub unsafe extern "C" fn camera_checks(fighter: &mut L2CAgentBase) -> bool {
    let ret = !is_flat_stage() && FighterCutInManager__is_one_on_one();
    return ret;
}