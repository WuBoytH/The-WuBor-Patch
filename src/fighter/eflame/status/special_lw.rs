use crate::imports::status_imports::*;
use crate::fighter::element::status::special_lw::*;

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_SPECIAL_LW, element_special_lw_end);
}