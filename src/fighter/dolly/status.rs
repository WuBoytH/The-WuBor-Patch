use {
    smash::{
        lua2cpp::L2CFighterCommon,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*
    },
    super::super::common::common_status::fgc_dashback_main
};

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_specialn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    FEINT[entry_id(fighter.module_accessor)] = false;
    0.into()
}

pub fn install() {
    install_status_scripts!(
        dolly_dashback_main,
        dolly_specialn_end
    );
}