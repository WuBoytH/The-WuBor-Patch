use super::*;

unsafe extern "C" fn gamewatch_landing_attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_LandingAttackAir()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, gamewatch_landing_attack_air_pre);
}