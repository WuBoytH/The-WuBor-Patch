use super::*;

mod guard_cancel_attack_start;
mod guard_cancel_attack;

pub fn install(agent: &mut Agent) {
    guard_cancel_attack_start::install(agent);
    guard_cancel_attack::install(agent);
}