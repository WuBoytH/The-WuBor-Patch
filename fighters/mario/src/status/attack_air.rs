use super::*;
use super::super::helper::*;

unsafe extern "C" fn mario_attack_air_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    mario_remove_hammer(fighter);
    0.into()
}

unsafe extern "C" fn mario_landing_attack_air_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_LandingAttackAir();
    mario_remove_hammer(fighter);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_AIR, mario_attack_air_end);

    agent.status(End, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, mario_landing_attack_air_end);
}