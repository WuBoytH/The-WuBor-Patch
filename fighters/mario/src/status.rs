mod attack_s4;
mod attack_air;
mod special_s;
mod special_lw;
mod rebirth;

pub fn install(agent: &mut smashline::Agent) {
    attack_s4::install(agent);
    attack_air::install(agent);
    special_s::install(agent);
    special_lw::install(agent);
    rebirth::install(agent);
}
