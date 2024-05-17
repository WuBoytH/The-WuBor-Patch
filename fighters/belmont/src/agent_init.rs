use super::*;

#[no_mangle]
pub unsafe extern "C" fn belmont_guard_cont_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_SIMON_STATUS_KIND_ATTACK_HOLD_START.into(), true.into());
        return true.into();
    }
    false.into()
}
