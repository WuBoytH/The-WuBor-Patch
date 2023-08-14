use crate::imports::status_imports::*;
use crate::fighter::belmont::status::special_n::*;

#[status("richter", FIGHTER_STATUS_KIND_SPECIAL_N)]
unsafe fn richter_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    belmont_special_n_main_inner(fighter)
}

#[status("richter", FIGHTER_STATUS_KIND_SPECIAL_N)]
unsafe fn richter_special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    belmont_special_n_end_inner(fighter)
}

pub fn install() {
    richter_special_n_main::install();
    richter_special_n_end::install();
}