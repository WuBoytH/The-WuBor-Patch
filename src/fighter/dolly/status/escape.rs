use {
    smash::{
        lua2cpp::L2CFighterCommon,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*
};

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_ESCAPE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_escape_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Escape()
}

pub fn install() {
    install_status_scripts!(
        dolly_escape_main
    );
}