use crate::imports::status_imports::*;
use crate::fighter::common::status::movement::dash::*;

unsafe extern "C" fn ken_dashback_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_pre(fighter)
}

unsafe extern "C" fn ken_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

pub fn install() {
    install_status_scripts!(
        ken_dashback_pre,
        ken_dashback_main
    );
}