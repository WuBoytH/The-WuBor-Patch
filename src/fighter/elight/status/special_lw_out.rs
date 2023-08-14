use crate::imports::status_imports::*;
use crate::fighter::element::status::special_lw_out::*;

#[status("elight", FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_LW_OUT)]
unsafe fn elight_special_lw_out_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    element_special_lw_out_main(fighter)
}

pub fn install() {
    elight_special_lw_out_main::install();
}