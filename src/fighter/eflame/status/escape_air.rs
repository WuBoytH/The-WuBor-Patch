use {
    smash::{
        lua2cpp::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*
};

#[status_script(agent = "eflame", status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn eflame_escapeair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_EscapeAir()
}

pub fn install() {
    install_status_scripts!(
        eflame_escapeair_main
    );
}