use crate::imports::status_imports::*;

#[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_GUARD_ON, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ryu_guard_on_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_guard_on_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_guard_on_main_loop as *const () as _))
}

unsafe fn ryu_guard_on_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_CHARGE)
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B.into(), true.into());
        return 1.into();
    }
    fighter.status_GuardOn_Main()
}

pub fn install() {
    install_status_scripts!(
        ryu_guard_on_main
    );
}