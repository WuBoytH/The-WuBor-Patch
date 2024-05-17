mod attack_air;
mod special_n;
mod special_lw;

pub fn install(agent: &mut smashline::Agent) {
    attack_air::install(agent);
    special_n::install(agent);
    special_lw::install(agent);
}