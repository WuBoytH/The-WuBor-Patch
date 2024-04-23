use crate::imports::*;
use crate::fighter::common::status::movement::dash::*;

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, fgc_dashback_pre);
    agent.status(Main, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, fgc_dashback_main);
}