use crate::imports::status_imports::*;

#[status("eflame", FIGHTER_STATUS_KIND_ESCAPE_AIR)]
unsafe fn eflame_escapeair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_EscapeAir()
}

pub fn install() {
    eflame_escapeair_main::install();
}