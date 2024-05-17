use super::*;

mod attack_dash;

mod special_lw;
mod special_lw_hit;

pub fn install(agent: &mut smashline::Agent) {
    attack_dash::install(agent);

    special_lw::install(agent);
    special_lw_hit::install(agent);
}