use super::*;

mod dash;
mod attack;
mod attacklw3;
mod attackstand2;
mod landing_attack_air;

pub fn install(agent: &mut Agent) {
    dash::install(agent);
    attack::install(agent);
    attacklw3::install(agent);
    attackstand2::install(agent);
    landing_attack_air::install(agent);
}