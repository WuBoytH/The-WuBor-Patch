use {
    crate::imports::status_imports::*,
    super::super::helper::*
};

unsafe extern "C" fn ryu_specialhi(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specialhi_main(fighter);
    0.into()
}

unsafe extern "C" fn ryu_specialhi_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specialhi_main(fighter);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        ryu_specialhi,
        ryu_specialhi_command
    );
}