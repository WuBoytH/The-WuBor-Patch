use super::*;

#[no_mangle]
pub unsafe extern "C" fn belmont_guard_cont_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_SIMON_STATUS_KIND_ATTACK_HOLD_START.into(), true.into());
        return true.into();
    }
    false.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_GROUND_SPECIAL_UNIQ].assign(&L2CValue::Bool(false));
    fighter.global_table[CHECK_AIR_SPECIAL_UNIQ].assign(&L2CValue::Bool(false));
    fighter.global_table[CHECK_GROUND_ATTACK_UNIQ].assign(&L2CValue::Bool(false));
    fighter.global_table[CHECK_AIR_ITEM_THROW_UNIQ].assign(&L2CValue::Bool(false));
    fighter.global_table[GUARD_CONT_UNIQ].assign(&L2CValue::Ptr(belmont_guard_cont_pre as *const () as _));
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_start(on_start);
}
