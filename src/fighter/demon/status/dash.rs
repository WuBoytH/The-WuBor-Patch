use crate::imports::status_imports::*;
use crate::fighter::common::status::dash::*;

#[status_script(agent = "demon", status = FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn demon_dashback_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_pre(fighter)
}

#[status_script(agent = "demon", status = FIGHTER_DEMON_STATUS_KIND_DASH_BACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn demon_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

pub fn install() {
    install_status_scripts!(
        demon_dashback_pre,
        demon_dashback_main
    );
}