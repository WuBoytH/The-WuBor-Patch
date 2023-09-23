use crate::imports::status_imports::*;
use crate::fighter::ryu::helper::*;

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_ATTACK_LW4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ken_attack_lw4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_attack_reset(fighter);
    fighter.status_AttackLw4()
}

pub fn install() {
    install_status_scripts!(
        ken_attack_lw4_main
    );
}