use {
    crate::imports::status_imports::*,
    crate::fighter::common::status::movement::dash::*
};

unsafe extern "C" fn ryu_dashback_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_pre(fighter)
}

unsafe extern "C" fn ryu_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

pub fn install() {
    install_status_scripts!(
        ryu_dashback_pre,
        ryu_dashback_main
    );
}