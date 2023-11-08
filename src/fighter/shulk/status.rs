mod special_n_action;
mod special_lw_hit;

pub fn install(agent: &mut smashline::Agent) {
    special_n_action::install(agent);
    special_lw_hit::install(agent);
}