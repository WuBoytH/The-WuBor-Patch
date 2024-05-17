use super::*;

mod landing_attack_air;
mod special_hi_fall;

pub fn install(agent: &mut smashline::Agent) {
    landing_attack_air::install(agent);
    special_hi_fall::install(agent);
}