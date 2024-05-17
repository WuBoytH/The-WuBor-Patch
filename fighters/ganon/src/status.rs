use super::*;

mod attack_lw3;

mod special_n;

mod special_air_s_catch;
mod special_air_s_end;

pub fn install(agent: &mut smashline::Agent) {
    attack_lw3::install(agent);

    special_n::install(agent);

    special_air_s_catch::install(agent);
    special_air_s_end::install(agent);
}