use super::*;

mod guard_cancel_attack;

mod special_hi_loop;

pub fn install(agent: &mut Agent) {
    guard_cancel_attack::install(agent);

    special_hi_loop::install(agent);
}