use crate::imports::status_imports::*;
use crate::fighter::belmont::status::special_n::*;

#[status("simon", FIGHTER_STATUS_KIND_SPECIAL_N)]
unsafe fn simon_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    belmont_special_n_main_inner(fighter)
}

#[status("simon", FIGHTER_STATUS_KIND_SPECIAL_N)]
unsafe fn simon_special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    belmont_special_n_end_inner(fighter)
}

pub fn install() {
    simon_special_n_main::install();
    simon_special_n_end::install();
}