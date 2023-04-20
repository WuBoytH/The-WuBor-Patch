use crate::imports::status_imports::*;

#[status_script( agent = "mewtwo", status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn mewtwo_escape_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_EscapeAir()
}

pub fn install() {
    install_status_scripts!(
        mewtwo_escape_air_main
    );
}