use {
    smash::{
        lua2cpp::L2CFighterCommon,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    crate::fighter::belmont::status::special_lw::*
};

#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn richter_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    belmont_special_lw_main_inner(fighter)
}

pub fn install() {
    install_status_scripts!(
        richter_special_lw_main
    );
}