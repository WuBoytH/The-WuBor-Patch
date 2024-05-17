use super::*;
use crate::fighter::vars::element::status::special_lw::*;

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, element_special_lw_end);
}