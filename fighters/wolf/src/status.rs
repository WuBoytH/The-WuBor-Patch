use super::*;

mod attack;
mod attack_s4;

mod special_n;
mod special_n_cancel;

// mod special_hi_rush;
// mod special_hi_rush_end;

pub fn install(agent: &mut Agent) {
    attack::install(agent);
    attack_s4::install(agent);

    special_n::install(agent);
    special_n_cancel::install(agent);

    // special_hi_rush::install(agent);
    // special_hi_rush_end::install(agent);
}