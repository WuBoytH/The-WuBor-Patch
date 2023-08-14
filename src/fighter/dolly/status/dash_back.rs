use crate::imports::status_imports::*;
use crate::fighter::common::status::movement::dash::*;

#[status("dolly", FIGHTER_DOLLY_STATUS_KIND_DASH_BACK)]
unsafe fn dolly_dashback_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_pre(fighter)
}

#[status("dolly", FIGHTER_DOLLY_STATUS_KIND_DASH_BACK)]
unsafe fn dolly_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

pub fn install() {
    dolly_dashback_pre::install();
    dolly_dashback_main::install();
}