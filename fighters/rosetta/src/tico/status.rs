use super::*;

mod follow_guard;

mod free_guard;

mod guard_cancel_attack;

pub fn install(agent: &mut Agent) {
    follow_guard::install(agent);

    free_guard::install(agent);

    guard_cancel_attack::install(agent);
}