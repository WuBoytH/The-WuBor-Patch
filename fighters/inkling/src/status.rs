use super::*;

mod guard_cancel_attack;

mod special_hi_jump;
mod special_hi_rot;

pub fn install(agent: &mut Agent) {
    guard_cancel_attack::install(agent);

    special_hi_jump::install(agent);
    special_hi_rot::install(agent);
}