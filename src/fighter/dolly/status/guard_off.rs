use crate::imports::status_imports::*;

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_GUARD_OFF, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_guardoff_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardOff()
}

pub fn install() {
    install_status_scripts!(
        dolly_guardoff_main
    );
}