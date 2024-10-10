use super::*;

mod squat;
mod jump;

mod look_up;
mod look_up_wait;
mod look_up_rv;

mod special_s;

mod special_air_s_start;
mod special_air_s_hold;
mod special_air_s_end;

mod special_lw;

mod special_air_lw_start;
mod special_air_lw_loop;
mod special_air_lw_bound;

mod trick;

pub fn install(agent: &mut Agent) {
    squat::install(agent);
    jump::install(agent);

    look_up::install(agent);
    look_up_wait::install(agent);
    look_up_rv::install(agent);

    special_s::install(agent);

    special_air_s_start::install(agent);
    special_air_s_hold::install(agent);
    special_air_s_end::install(agent);

    special_lw::install(agent);

    special_air_lw_start::install(agent);
    special_air_lw_loop::install(agent);
    special_air_lw_bound::install(agent);

    trick::install(agent);
}