use crate::imports::*;
use super::super::helper::*;

unsafe extern "C" fn mario_attack_s4_start_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_AttackXX4Start();
    mario_remove_hammer(fighter);
    0.into()
}

unsafe extern "C" fn mario_attack_s4_hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_AttackS4Hold();
    mario_remove_hammer(fighter);
    0.into()
}

unsafe extern "C" fn mario_attack_s4_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    mario_remove_hammer(fighter);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_S4_START, mario_attack_s4_start_end);

    agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, mario_attack_s4_hold_end);

    agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_S4, mario_attack_s4_end);
}