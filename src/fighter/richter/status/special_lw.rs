use crate::imports::status_imports::*;
use crate::fighter::belmont::status::special_lw::*;

#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn richter_special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    belmont_special_lw_pre_inner(fighter)
}

#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn richter_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    belmont_special_lw_main_inner(fighter)
}

#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn richter_special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    belmont_special_lw_end_inner(fighter)
}

pub fn install() {
    install_status_scripts!(
        richter_special_lw_pre,
        richter_special_lw_main,
        richter_special_lw_end
    );
}