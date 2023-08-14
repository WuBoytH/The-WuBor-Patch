use crate::imports::status_imports::*;
use crate::fighter::belmont::status::special_lw::*;

#[status("simon", FIGHTER_STATUS_KIND_SPECIAL_LW)]
unsafe fn simon_special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    belmont_special_lw_pre_inner(fighter)
}

#[status("simon", FIGHTER_STATUS_KIND_SPECIAL_LW)]
unsafe fn simon_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    belmont_special_lw_main_inner(fighter)
}

#[status("simon", FIGHTER_STATUS_KIND_SPECIAL_LW)]
unsafe fn simon_special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    belmont_special_lw_end_inner(fighter)
}

pub fn install() {
    simon_special_lw_pre::install();
    simon_special_lw_main::install();
    simon_special_lw_end::install();
}