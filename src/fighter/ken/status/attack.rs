use crate::imports::*;
use crate::fighter::ryu::helper::*;

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK, ryu_attack_main_inner);
}