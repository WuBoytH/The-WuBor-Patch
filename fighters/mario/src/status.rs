use super::*;

mod attack_s4;

mod attack_air;

mod special_s;

mod special_lw;
mod special_lw_jump;
mod special_lw_landing;

mod special_air_lw;
mod special_air_lw_fall;
mod special_air_lw_landing;
mod special_air_lw_cancel;

mod rebirth;

pub fn install(agent: &mut Agent) {
    attack_s4::install(agent);

    attack_air::install(agent);

    special_s::install(agent);

    special_lw::install(agent);
    special_lw_jump::install(agent);
    special_lw_landing::install(agent);

    special_air_lw::install(agent);
    special_air_lw_fall::install(agent);
    special_air_lw_landing::install(agent);
    special_air_lw_cancel::install(agent);

    rebirth::install(agent);
}
