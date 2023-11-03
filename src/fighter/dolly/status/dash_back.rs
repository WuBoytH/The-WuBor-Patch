use crate::imports::status_imports::*;
use crate::fighter::common::status::movement::dash::*;

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn dolly_dashback_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_pre(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

pub fn install() {
    install_status_scripts!(
        dolly_dashback_pre,
        dolly_dashback_main
    );
}