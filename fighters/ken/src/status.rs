use super::*;

mod dash_back;

mod attack;

mod attack_lw4_start;
mod attack_lw4;

mod special_n2;

mod special_s;
mod special_s_loop;
mod special_s_end;

mod special_hi;

mod special_lw;

pub fn install(agent: &mut Agent) {
    dash_back::install(agent);

    attack::install(agent);

    attack_lw4_start::install(agent);
    attack_lw4::install(agent);

    special_n2::install(agent);

    special_s::install(agent);
    special_s_loop::install(agent);
    special_s_end::install(agent);

    special_hi::install(agent);

    special_lw::install(agent);
}