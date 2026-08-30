use super::*;

mod guard_cancel_attack;

mod special_hi;
mod special_hi_2;
mod special_hi_fail;

pub fn install(agent: &mut Agent) {
    guard_cancel_attack::install(agent);

    special_hi::install(agent);
    special_hi_2::install(agent);
    special_hi_fail::install(agent);
}
