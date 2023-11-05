use crate::imports::status_imports::*;
use crate::fighter::ryu::helper::*;

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn ken_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_attack_main_inner(fighter)
}

pub fn install() {
    install_status_scripts!(
        ken_attack_main
    );
}