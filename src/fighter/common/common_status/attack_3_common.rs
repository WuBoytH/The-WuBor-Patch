use {
    smash::{
        lua2cpp::L2CFighterCommon,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    wubor_utils::table_const::*,
    super::attack::only_jabs,
};

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon28sub_attack3_uniq_check_paramEN3lib8L2CValueES2_S2_")]
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

pub fn install() {
    install_hooks!(
        sub_attack3_uniq_check_param
    );
}