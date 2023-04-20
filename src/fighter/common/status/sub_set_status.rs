use crate::imports::status_imports::*;

#[skyline::hook(replace = L2CFighterCommon_sub_set_status_pre_msc_common_table)]
unsafe fn sub_set_status_pre_msc_common_table(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if fighter.global_table[CHECK_GROUND_JUMP_MINI_ATTACK].get_bool() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_GROUND_JUMP_MINI_ATTACK].get_ptr());
            return callable(fighter);
        }
        // Disable the grab button from being used to perform the "short hop aerial macro"
        let cat1 = fighter.global_table[CMD_CAT1].get_i32();
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH == 0
        && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
        && fighter.sub_check_button_jump().get_bool() {
            fighter.change_status_jump_mini_attack(false.into());
            return true.into();
        }
    }
    false.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_set_status_pre_msc_common_table
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}