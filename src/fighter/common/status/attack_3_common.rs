use crate::imports::status_imports::*;

#[skyline::hook(replace = L2CFighterCommon_sub_attack3_uniq_check_param)]
unsafe fn sub_attack3_uniq_check_param(fighter: &mut L2CFighterCommon, param_1: L2CValue, is_attack_lw3: L2CValue, is_button: L2CValue) {
    if param_1.get_bool() == false {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(is_button.get_ptr());
        if callable.get_bool()
        && only_jabs(fighter) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_INPUT) {
                if ComboModule::is_enable_combo_input(fighter.module_accessor) {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE);
                }
            }
        }
        
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_attack3_uniq_check_param
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}