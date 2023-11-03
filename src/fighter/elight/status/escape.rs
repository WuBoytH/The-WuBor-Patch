use crate::imports::status_imports::*;

#[status_script(agent = "elight", status = FIGHTER_STATUS_KIND_ESCAPE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn elight_escape_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Escape()
}

pub fn install() {
    install_status_scripts!(
        elight_escape_main
    );
}