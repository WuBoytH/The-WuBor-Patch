use crate::imports::status_imports::*;

#[status("mewtwo", FIGHTER_STATUS_KIND_ESCAPE_AIR)]
unsafe fn mewtwo_escape_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_EscapeAir()
}

pub fn install() {
    mewtwo_escape_air_main::install();
}