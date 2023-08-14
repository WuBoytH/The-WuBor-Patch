use crate::imports::status_imports::*;

#[status("dolly", FIGHTER_STATUS_KIND_ESCAPE)]
unsafe fn dolly_escape_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Escape()
}

pub fn install() {
    dolly_escape_main::install();
}