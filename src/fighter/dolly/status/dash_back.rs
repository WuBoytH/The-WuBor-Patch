use crate::imports::status_imports::*;
use crate::fighter::common::status::movement::dash::*;

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, fgc_dashback_pre);
    agent.status(smashline::Main, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, fgc_dashback_main);
}