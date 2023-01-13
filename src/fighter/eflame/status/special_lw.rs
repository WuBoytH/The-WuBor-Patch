use {
    smash::{
        lua2cpp::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    crate::fighter::element::status::special_lw::*
};

#[status_script(agent = "eflame", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn eflame_special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    element_special_lw_end(fighter)
}

pub fn install() {
    install_status_scripts!(
        eflame_special_lw_end
    );
}