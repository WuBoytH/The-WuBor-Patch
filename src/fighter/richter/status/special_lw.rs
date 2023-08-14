use crate::imports::status_imports::*;
use crate::fighter::belmont::status::special_lw::*;

#[status("richter", FIGHTER_STATUS_KIND_SPECIAL_LW)]
unsafe fn richter_special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    belmont_special_lw_pre_inner(fighter)
}

#[status("richter", FIGHTER_STATUS_KIND_SPECIAL_LW)]
unsafe fn richter_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    belmont_special_lw_main_inner(fighter)
}

#[status("richter", FIGHTER_STATUS_KIND_SPECIAL_LW)]
unsafe fn richter_special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    belmont_special_lw_end_inner(fighter)
}

pub fn install() {
    richter_special_lw_pre::install();
    richter_special_lw_main::install();
    richter_special_lw_end::install();
}