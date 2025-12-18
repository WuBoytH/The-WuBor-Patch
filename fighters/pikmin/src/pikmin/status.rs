use super::*;

mod guard_cancel_attack_start;
mod guard_cancel_attack;

mod attack_dash;

pub fn install(agent: &mut Agent) {
    guard_cancel_attack_start::install(agent);
    guard_cancel_attack::install(agent);

    attack_dash::install(agent);
}