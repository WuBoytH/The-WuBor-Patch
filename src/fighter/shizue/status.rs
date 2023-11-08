mod special_n_search;
mod special_lw_fire;

pub fn install(agent: &mut smashline::Agent) {
    special_n_search::install(agent);
    special_lw_fire::install(agent);
}