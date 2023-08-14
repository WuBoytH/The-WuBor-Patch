use crate::imports::status_imports::*;
use crate::fighter::common::status::movement::dash::*;

#[status("ken", FIGHTER_RYU_STATUS_KIND_DASH_BACK)]
unsafe fn ken_dashback_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_pre(fighter)
}

#[status("ken", FIGHTER_RYU_STATUS_KIND_DASH_BACK)]
unsafe fn ken_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

pub fn install() {
    ken_dashback_pre::install();
    ken_dashback_main::install();
}