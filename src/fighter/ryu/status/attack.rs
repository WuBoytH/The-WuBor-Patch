use {
    crate::imports::status_imports::*,
    super::super::helper::*
};

unsafe extern "C" fn ryu_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_attack_main_inner(fighter)
}

pub fn install() {
    install_status_scripts!(
        ryu_attack_main
    );
}