use super::*;

mod special_n_action;

pub fn install(agent: &mut smashline::Agent) {
    special_n_action::install(agent);
}