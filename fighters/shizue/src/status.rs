use super::*;

mod special_n_search;

// mod special_hi_wait;
// mod special_hi_flap;
// mod special_hi_turn;

mod special_lw_fire;

pub fn install(agent: &mut Agent) {
    special_n_search::install(agent);
    
    // special_hi_wait::install(agent);
    // special_hi_flap::install(agent);
    // special_hi_turn::install(agent);

    special_lw_fire::install(agent);
}