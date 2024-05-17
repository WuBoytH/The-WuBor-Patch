use super::*;

mod attack_air;

mod rebirth;

pub fn install(agent: &mut smashline::Agent) {
    attack_air::install(agent);

    rebirth::install(agent);
}