use super::*;
use super::helper::*;

unsafe extern "C" fn shizue_special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if shizue_check_rocket_fire(fighter)
    && shizue_check_attack_cancel(fighter) {
        VarModule::on_flag(fighter.module_accessor, vars::shizue::instance::flag::FIRE_ROCKET_ANYTIME);
        ControlModule::clear_command_one(
            fighter.module_accessor,
            *FIGHTER_PAD_COMMAND_CATEGORY1,
            *FIGHTER_PAD_CMD_CAT1_SPECIAL_LW
        );
        return 0.into();
    }
    1.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    if fighter_kind != *FIGHTER_KIND_SHIZUE {
        return;
    }
    fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(shizue_special_lw_pre as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
}
