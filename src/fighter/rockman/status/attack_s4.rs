use crate::imports::status_imports::*;

unsafe extern "C" fn rockman_attack_s4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackS4()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_ATTACK_S4, rockman_attack_s4_main);
}