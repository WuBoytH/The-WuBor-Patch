use crate::imports::status_imports::*;
use crate::fighter::element::status::special_lw::*;

#[status("elight", FIGHTER_STATUS_KIND_SPECIAL_LW)]
unsafe fn elight_special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    element_special_lw_end(fighter)
}

pub fn install() {
    elight_special_lw_end::install();
}