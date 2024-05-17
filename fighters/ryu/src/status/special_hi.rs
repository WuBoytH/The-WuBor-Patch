use super::*;
use super::super::helper::*;

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, ryu_specialhi_main);
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND, ryu_specialhi_main);
}