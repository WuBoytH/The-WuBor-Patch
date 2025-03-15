use super::*;

mod guard_cancel_attack;

mod special_hi;

pub fn install(agent: &mut Agent) {
    guard_cancel_attack::install(agent);

    special_hi::install(agent);
}