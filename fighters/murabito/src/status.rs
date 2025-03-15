use super::*;

mod guard_cancel_attack;

mod special_n_search;

// mod special_hi_wait;
// mod special_hi_flap;
// mod special_hi_turn;

pub fn install(agent: &mut Agent) {
    guard_cancel_attack::install(agent);

    special_n_search::install(agent);

    // special_hi_wait::install(agent);
    // special_hi_flap::install(agent);
    // special_hi_turn::install(agent);
}