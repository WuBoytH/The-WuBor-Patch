use crate::imports::status_imports::*;
use crate::fighter::element::status::special_lw_out::*;

#[status("eflame", FIGHTER_EFLAME_STATUS_KIND_SPECIAL_LW_OUT)]
unsafe fn eflame_special_lw_out_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    element_special_lw_out_main(fighter)
}

pub fn install() {
    eflame_special_lw_out_main::install();
}