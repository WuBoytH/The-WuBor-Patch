use {
    crate::imports::status_imports::*,
    super::super::helper::*
};

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, ryu_specialhi_main);
    agent.status(smashline::Main, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND, ryu_specialhi_main);
}