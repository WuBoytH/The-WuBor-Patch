use super::*;

unsafe extern "C" fn eflame_special_lw_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::fighter::instance::flag::BURNOUT) {
        return 0.into();
    }

    1.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_AIR_ITEM_THROW_UNIQ].assign(&L2CValue::Bool(false));
    fighter.global_table[FALL_BRAKE_UNIQ].assign(&L2CValue::Bool(false));
    fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(eflame_special_lw_uniq as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
}
