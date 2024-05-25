use super::*;

mod special_n_action;

pub fn install(agent: &mut Agent) {
    special_n_action::install(agent);
}