use crate::imports::status_imports::*;
use crate::fighter::ryu::helper::*;

unsafe extern "C" fn ken_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_attack_main_inner(fighter)
}

pub fn install() {
    install_status_scripts!(
        ken_attack_main
    );
}