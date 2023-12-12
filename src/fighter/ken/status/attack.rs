use crate::imports::status_imports::*;
use crate::fighter::ryu::helper::*;

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_ATTACK, ryu_attack_main_inner);
}