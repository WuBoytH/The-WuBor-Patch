use super::*;

mod attack;
mod attack_s4;
// mod special_hi_rush;

pub fn install(agent: &mut Agent) {
    attack::install(agent);
    attack_s4::install(agent);
    // special_hi_rush::install(agent);
}