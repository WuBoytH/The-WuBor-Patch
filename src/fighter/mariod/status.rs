mod special_lw;

pub fn install(agent: &mut smashline::Agent) {
    special_lw::install(agent);
}