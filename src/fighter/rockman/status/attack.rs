use crate::imports::status_imports::*;

unsafe extern "C" fn rockman_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Attack()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_ATTACK, rockman_attack_pre);
}