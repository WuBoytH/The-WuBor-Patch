use crate::imports::status_imports::*;
use crate::fighter::common::status::movement::dash::*;

#[status("demon", FIGHTER_DOLLY_STATUS_KIND_DASH_BACK)]
unsafe fn demon_dashback_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_pre(fighter)
}

#[status("demon", FIGHTER_DEMON_STATUS_KIND_DASH_BACK)]
unsafe fn demon_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

pub fn install() {
    demon_dashback_pre::install();
    demon_dashback_main::install();
}