use crate::imports::status_imports::*;
use crate::fighter::ryu::helper::*;

unsafe extern "C" fn ken_attack_lw4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_attack_reset(fighter);
    fighter.status_AttackLw4()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_ATTACK_LW4, ken_attack_lw4_main);
}