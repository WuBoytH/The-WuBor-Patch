use super::*;

mod attack;
mod attack_s4;
mod attack_lw4;
mod special_hi_rush;
mod appeal;

pub fn install(agent: &mut smashline::Agent) {
    attack::install(agent);
    attack_s4::install(agent);
    attack_lw4::install(agent);
    special_hi_rush::install(agent);
    appeal::install(agent);
}