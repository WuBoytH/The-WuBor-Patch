use super::*;

mod walk;
mod attack;
mod attack_s3;
mod attack_s4;
mod attack_air;
mod ladder_attack;
mod special_n;
mod rockbuster;
mod special_s;
mod special_lw;
mod rebirth;
pub mod helper;

pub fn install(agent: &mut smashline::Agent) {
    walk::install(agent);
    attack::install(agent);
    attack_s3::install(agent);
    attack_s4::install(agent);
    attack_air::install(agent);
    ladder_attack::install(agent);
    special_n::install(agent);
    rockbuster::install(agent);
    special_s::install(agent);
    special_lw::install(agent);
    rebirth::install(agent);
}
